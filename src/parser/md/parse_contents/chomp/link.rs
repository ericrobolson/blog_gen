use crate::{
    parser::md::{
        parse_contents::{content_ir::LinkType, Validations},
        ContentIr,
    },
    Item, Location, Res,
};

pub fn link(
    contents: &String,
    location: &Location,
    validations: &Validations,
) -> Res<Option<Vec<ContentIr>>> {
    // See if it starts with a '('.
    let mut description_start = None;
    let mut description_end = None;
    let mut exclamation = None;
    let mut url_start = None;
    let mut url_end = None;

    let mut short_out = false;

    let mut idx = 0;
    let chars: Vec<char> = contents.chars().collect();

    if chars.len() == 0 {
        return Res::Ok(Item {
            item: None,
            location: Some(location.clone()),
        });
    }

    while idx < chars.len() {
        let c = chars[idx];

        //TODO: the error handling for links is... iffy.
        // This will get the basic versions going at least.

        match c {
            '[' => {
                if let Some(start) = description_start {
                    idx = start - 1;
                    short_out = true;
                }

                description_start = Some(idx);
            }
            ']' => {
                if description_start.is_some() {
                    description_end = Some(idx);
                }
            }
            '(' => {
                if description_end.is_some() {
                    url_start = Some(idx);
                }
            }
            ')' => {
                if url_start.is_some() {
                    url_end = Some(idx);
                    break;
                }
            }
            '!' => {
                if description_start.is_none() {
                    exclamation = Some(idx);
                }
            }
            _ => {}
        }

        if short_out {
            exclamation = None;
            description_start = None;
            description_end = None;
            url_start = None;
            url_end = None;
        }

        if description_start.is_some()
            && description_end.is_some()
            && url_start.is_some()
            && url_end.is_some()
        {
            break;
        }

        idx += 1;
    }

    if let Some(description_start) = description_start {
        if let Some(description_end) = description_end {
            if let Some(url_start) = url_start {
                if let Some(url_end) = url_end {
                    if description_start >= description_end || url_start >= url_end {
                        return Res::Ok(Item {
                            item: None,
                            location: Some(location.clone()),
                        });
                    }

                    let exclamation = {
                        if let Some(exclamation) = exclamation {
                            if description_start > 1 && exclamation == (description_start - 1) {
                                true
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    };

                    let description = &contents[description_start..description_end]
                        .replace("[", "")
                        .replace("]", "");
                    let mut url = String::from(
                        &contents[url_start..url_end]
                            .replace("(", "")
                            .replace(")", ""),
                    );

                    // Remove ! if it's a gif link
                    let description_start = {
                        if exclamation && description_start > 0 {
                            description_start - 1
                        } else {
                            description_start
                        }
                    };

                    let before = &contents[..description_start];

                    let mut end_parse = url_end + 1;
                    if end_parse == contents.len() {
                        end_parse -= 1;
                    }

                    let after = &contents[end_parse..];

                    let mut ir = vec![];

                    if before.len() > 0 {
                        let mut validations = validations.clone();
                        validations.skip_link = true;

                        ir.push(ContentIr::Unparsed {
                            contents: before.into(),
                            validations: validations,
                        });
                    }

                    // Push the link itself
                    {
                        let mut link_type = LinkType::Regular;

                        if exclamation {
                            const ASSETS_FOLDER: &'static str = "assets/";
                            const IMG: &'static str = "./_img/";
                            if url.starts_with(IMG) {
                                url = url.replace(IMG, "").to_lowercase();

                                // Flatten the file path
                                url = if url.contains("/") {
                                    url.split("/").last().unwrap_or_default().to_string()
                                } else {
                                    url
                                };

                                url = format!("{}{}", ASSETS_FOLDER, url);

                                link_type = LinkType::LocalImage;
                            } else {
                                let desc = description.trim();
                                if desc == "youtube" {
                                    link_type = LinkType::Youtube;
                                } else if desc == "iframe" {
                                    link_type = LinkType::Iframe;
                                } else {
                                    return Res::Warn{item: Item {
                                        item: None,
                                        location: Some(location.clone()),
                                    }, msg: format!("Misused link in file '{:?}'! This is probably a gif. Link: {:?}. Description: {:?}.", location.file, url, desc),
                                    };
                                }
                            }
                        }

                        ir.push(ContentIr::Link {
                            description: description.into(),
                            link_type,
                            url: url.into(),
                        });
                    }

                    if after.len() > 0 {
                        ir.push(ContentIr::Unparsed {
                            contents: after.into(),
                            validations: validations.clone(),
                        });
                    }

                    return Res::Ok(Item {
                        item: Some(ir),
                        location: Some(location.clone()),
                    });
                }
            }
        }
    }

    return Res::Ok(Item {
        item: None,
        location: Some(location.clone()),
    });
}
