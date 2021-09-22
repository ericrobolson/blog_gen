mod filename;
mod parse_contents;

use super::IntermediateRepresentation;
use crate::{
    parser::md::parse_contents::{Metadata, Options},
    Item, Location, Res,
};
pub use parse_contents::{ContentIr, LinkType};

#[derive(Clone, Debug, PartialEq)]
pub struct Link(String);

impl Link {
    pub fn to_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Md {
    pub contents: Item<ContentIr>,
    pub date: String,
    pub day: String,
    pub default_title: String,
    pub display_title: String,
    pub html_file_name: Link,
    pub keywords: Vec<String>,
    pub month: String,
    pub month_year: String,
    pub navigation: Navigation,
    pub path: String,
    pub series: Vec<String>,
    pub summary: String,
    pub year: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Navigation {
    pub next: Vec<Link>,
    pub previous: Vec<Link>,
}

pub fn parse(file_path: String, location: Location) -> Res<IntermediateRepresentation> {
    let path = std::path::Path::new(&file_path);
    let path_string = String::from(path.to_str().unwrap_or("".into()));
    let file_name = path.file_name().unwrap().to_str().unwrap_or("").to_string();
    let mut warnings = vec![];

    match filename::validate(&file_name) {
        Ok(_) => {}
        Err(e) => {
            return Res::Error {
                location: Some(location),
                msg: e,
            }
        }
    }

    const DATE_LEN: usize = 10;

    let date = &file_name[..DATE_LEN].to_string().replace(".", "_");

    let default_title = &file_name[DATE_LEN..]
        .to_string()
        .replace("_", "")
        .replace(".md", "");

    let date = date.clone().replace("_", ".");

    let split: Vec<String> = date.split(".").map(|m| m.to_string()).collect();
    const YEAR_IDX: usize = 0;
    const MONTH_IDX: usize = 1;
    const DAY_IDX: usize = 2;

    let contents = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            return Res::Error {
                location: Some(location),
                msg: format!("{:#?}", e),
            }
        }
    };

    let parse_opts = Options {
        make_cards: true,
        make_paragraphs: true,
    };

    let contents = match parse_contents::execute(contents, location.clone(), parse_opts, None) {
        Res::Ok(item) => item,
        Res::Warn { item, msg } => {
            warnings.push((msg, item.location.clone()));
            item
        }
        Res::Error { location, msg } => return Res::Error { location, msg },
        Res::Warnings { item, mut msgs } => {
            warnings.append(&mut msgs);
            item
        }
    };

    let display_title = match get_display_title(&contents) {
        Res::Ok(title) => title.item,
        Res::Warn { item, msg } => {
            warnings.push((msg, item.location.clone()));
            item.item
        }
        Res::Warnings { item, mut msgs } => {
            warnings.append(&mut msgs);
            item.item
        }
        Res::Error { location, msg } => return Res::Error { location, msg },
    };

    let keywords = match get_keywords(&contents) {
        Res::Ok(words) => words.item,
        Res::Warn { item, msg } => {
            warnings.push((msg, item.location.clone()));
            item.item
        }
        Res::Warnings { item, mut msgs } => {
            warnings.append(&mut msgs);
            item.item
        }
        Res::Error { location, msg } => return Res::Error { location, msg },
    };

    let series = match get_series(&contents) {
        Res::Ok(words) => words.item,
        Res::Warn { item, msg } => {
            warnings.push((msg, item.location.clone()));
            item.item
        }
        Res::Warnings { item, mut msgs } => {
            warnings.append(&mut msgs);
            item.item
        }
        Res::Error { location, msg } => return Res::Error { location, msg },
    };

    let summary = match get_summary(&contents) {
        Res::Ok(title) => title.item,
        Res::Warn { item, msg } => {
            warnings.push((msg, item.location.clone()));
            item.item
        }
        Res::Warnings { item, mut msgs } => {
            warnings.append(&mut msgs);
            item.item
        }
        Res::Error { location, msg } => return Res::Error { location, msg },
    };

    let contents = Item {
        location: Some(location.clone()),
        item: ContentIr::Div {
            contents: vec![
                ContentIr::title(format!("{} - {}", date, display_title), "title".into()),
                contents.item,
            ],
        },
    };

    let item = Item {
        item: IntermediateRepresentation::Md(Md {
            contents,
            date: date.clone(),
            day: split[DAY_IDX].clone(),
            default_title: default_title.clone(),
            display_title,
            html_file_name: Link(format!("{}_{}.html", date, default_title)),
            keywords,
            month: split[MONTH_IDX].clone(),
            month_year: format!("{},{}", split[YEAR_IDX], split[MONTH_IDX]),
            navigation: Navigation {
                next: vec![],
                previous: vec![],
            },
            series,
            summary,
            path: path_string.clone(),
            year: split[YEAR_IDX].clone(),
        }),
        location: Some(location),
    };

    if warnings.len() == 1 {
        Res::Warn {
            item,
            msg: warnings[0].0.clone(),
        }
    } else if warnings.len() > 1 {
        Res::Warnings {
            item,
            msgs: warnings,
        }
    } else {
        Res::Ok(item)
    }
}

fn get_display_title(contents: &Item<ContentIr>) -> Res<String> {
    match &contents.item {
        ContentIr::Div {
            contents: content_ir,
        } => {
            for ir in content_ir.iter() {
                let item = Item {
                    location: contents.location.clone(),
                    item: ir.clone(),
                };

                let result = get_display_title(&item);
                match result {
                    Res::Error { .. } => {
                        // skip over to parse the next
                    }
                    _ => return result,
                }
            }
        }
        ContentIr::Metadata(m) => match m {
            Metadata::DisplayTitle(title) => {
                const MAX_SEO_TITLE_LEN: usize = 65;

                // Warn if there is no title
                if title.len() == 0 {
                    return Res::Warn {
                        item: Item {
                            item: "Placeholder!".into(),
                            location: contents.location.clone(),
                        },
                        msg: "Display title empty!".into(),
                    };
                }
                // Warn if over 65chars for SEO
                else if title.chars().count() >= MAX_SEO_TITLE_LEN {
                    return Res::Warn {
                        item: Item {
                            item: title.into(),
                            location: contents.location.clone(),
                        },
                        msg: format!(
                            "Display title exceeds {} characters! Got {}.",
                            MAX_SEO_TITLE_LEN,
                            title.chars().count()
                        ),
                    };
                }

                return Res::Ok(Item {
                    location: contents.location.clone(),
                    item: title.clone(),
                });
            }
            _ => {}
        },
        _ => {}
    }

    Res::Error {
        location: contents.location.clone(),
        msg:
            "Display title not found! Start each page with '!!title YOUR TITLE' ended with newline."
                .into(),
    }
}

fn get_keywords(contents: &Item<ContentIr>) -> Res<Vec<String>> {
    const MIN_KEYWORDS_LEN: usize = 5;

    match &contents.item {
        ContentIr::Div {
            contents: content_ir,
        } => {
            for ir in content_ir.iter() {
                let item = Item {
                    location: contents.location.clone(),
                    item: ir.clone(),
                };

                let result = get_keywords(&item);
                match result {
                    Res::Error { .. } => {
                        // skip over to parse the next
                    }
                    _ => return result,
                }
            }
        }
        ContentIr::Metadata(m) => match m {
            Metadata::Keywords(words) => {
                if words.len() == 0 {
                    return Res::Warn {
                        item: Item {
                            item: vec!["A".into(), "Placeholder!".into()],
                            location: contents.location.clone(),
                        },
                        msg: "Keywords empty!".into(),
                    };
                }
                // SEO optimization
                else if words.len() < MIN_KEYWORDS_LEN {
                    return Res::Warn {
                        item: Item {
                            item: words.clone(),
                            location: contents.location.clone(),
                        },
                        msg: format!(
                            "Keywords should have at least {} words! Got {}.",
                            MIN_KEYWORDS_LEN,
                            words.len()
                        ),
                    };
                }

                return Res::Ok(Item {
                    location: contents.location.clone(),
                    item: words.clone(),
                });
            }
            _ => {}
        },
        _ => {}
    }

    Res::Error {
        location: contents.location.clone(),
        msg:
            "Keywords not found! Start each page with '!!keywords A LIST OF KEYWORDS' ended with newline."
                .into(),
    }
}

fn get_series(contents: &Item<ContentIr>) -> Res<Vec<String>> {
    match &contents.item {
        ContentIr::Div {
            contents: content_ir,
        } => {
            for ir in content_ir.iter() {
                let item = Item {
                    location: contents.location.clone(),
                    item: ir.clone(),
                };

                let result = get_series(&item);
                match result {
                    Res::Error { .. } => {
                        // skip over to parse the next
                    }
                    _ => return result,
                }
            }
        }
        ContentIr::Metadata(m) => match m {
            Metadata::Series(words) => {
                return Res::Ok(Item {
                    location: contents.location.clone(),
                    item: words.clone(),
                });
            }
            _ => {}
        },
        _ => {}
    }

    Res::Error {
        location: contents.location.clone(),
        msg: "Series not found! Start each page with '!!series SERIES_TITLE' ended with newline."
            .into(),
    }
}

fn get_summary(contents: &Item<ContentIr>) -> Res<String> {
    const MIN_META_DESCRIPTION_LEN: usize = 30;
    const MAX_META_DESCRIPTION_LEN: usize = 160;

    match &contents.item {
        ContentIr::Div {
            contents: content_ir,
        } => {
            for ir in content_ir.iter() {
                let item = Item {
                    location: contents.location.clone(),
                    item: ir.clone(),
                };

                let result = get_summary(&item);
                match result {
                    Res::Error { .. } => {
                        // skip over to parse the next
                    }
                    _ => return result,
                }
            }
        }
        ContentIr::Metadata(m) => match m {
            Metadata::Summary(summary) => {
                if summary.len() == 0 {
                    return Res::Warn {
                        item: Item {
                            item: "A placeholder!".into(),
                            location: contents.location.clone(),
                        },
                        msg: "Summary empty!".into(),
                    };
                } else if summary.chars().count() < MIN_META_DESCRIPTION_LEN {
                    return Res::Warn {
                        item: Item {
                            item: summary.clone(),
                            location: contents.location.clone(),
                        },
                        msg: format!(
                            "Summary should have a minimum len of {}! Got {}.",
                            MIN_META_DESCRIPTION_LEN,
                            summary.chars().count()
                        ),
                    };
                } else if summary.chars().count() > MAX_META_DESCRIPTION_LEN {
                    return Res::Warn {
                        item: Item {
                            item: summary.clone(),
                            location: contents.location.clone(),
                        },
                        msg: format!(
                            "Summary should have a maximum len of {}! Got {}.",
                            MAX_META_DESCRIPTION_LEN,
                            summary.chars().count()
                        ),
                    };
                }

                return Res::Ok(Item {
                    location: contents.location.clone(),
                    item: summary.clone(),
                });
            }
            _ => {}
        },
        _ => {}
    }

    Res::Error {
        location: contents.location.clone(),
        msg:
            "Keywords not found! Start each page with '!!keywords A LIST OF KEYWORDS' ended with newline."
                .into(),
    }
}
