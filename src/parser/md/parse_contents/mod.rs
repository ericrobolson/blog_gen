mod chomp;
mod content_ir;
mod hierarchy;

use crate::{Item, Location, Res};
pub use content_ir::{ContentIr, LinkType, Metadata};

pub(crate) enum ContinueRunning {
    No,
    Yes,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Options {
    pub make_cards: bool,
    pub make_paragraphs: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Validations {
    pub skip_code_block: bool,
    pub skip_header: bool,
    pub skip_inline_code: bool,
    pub skip_link: bool,
    pub skip_list: bool,
    pub skip_metadata_display_title: bool,
    pub skip_metadata_keywords: bool,
    pub skip_metadata_series: bool,
    pub skip_metadata_summary: bool,
    pub validate_paragraph_len: bool,
}

pub fn execute(
    contents: String,
    location: Location,
    opts: Options,
    validations: Option<Validations>,
) -> Res<ContentIr> {
    let mut warnings = vec![];

    // Preprocess things to simplify parsing.
    let contents = {
        let mut contents = contents.replace("\r\n", "\n");

        // Add a newline to ensure that it gets processed correctly
        if !contents.ends_with("\n") {
            contents.push('\n');
        }

        contents
    };

    let validations = validations.unwrap_or(Validations {
        skip_code_block: false,
        skip_header: false,
        skip_inline_code: false,
        skip_link: false,
        skip_list: false,
        skip_metadata_display_title: false,
        skip_metadata_keywords: false,
        skip_metadata_series: false,
        skip_metadata_summary: false,
        validate_paragraph_len: true,
    });

    // Since this is a recursive processor, executing until nothing remains, set it up so that nothing has
    // been processed yet.
    let mut ir = vec![ContentIr::Unparsed {
        contents,
        validations,
    }];

    // Continue processing + reprocessing all elements until there's nothing left.
    loop {
        let result = match parse(&ir, &location, opts) {
            Res::Ok(item) => item.item,
            Res::Warn { item, msg } => {
                warnings.push((msg, item.location.clone()));
                item.item
            }
            Res::Error { location, msg } => return Res::Error { location, msg },
            Res::Warnings { item, mut msgs } => {
                warnings.append(&mut msgs);
                item.item
            }
        };

        let (new_elements, continue_running) = result;

        ir = new_elements;

        match continue_running {
            ContinueRunning::Yes => {}
            ContinueRunning::No => break,
        }
    }

    // Map out paragraphs
    let ir = if opts.make_paragraphs {
        hierarchy::paragraphs(ir)
    } else {
        ir
    };

    // Map out headers
    let contents = hierarchy::headers(ir);

    // Clean the tree of all empty values
    let contents = hierarchy::clean(contents);

    if warnings.len() == 0 {
        Res::Ok(Item {
            item: ContentIr::Div { contents },
            location: Some(location),
        })
    } else if warnings.len() == 1 {
        Res::Warn {
            item: Item {
                item: ContentIr::Div { contents },
                location: Some(location),
            },
            msg: warnings[0].clone().0,
        }
    } else {
        Res::Warnings {
            item: Item {
                item: ContentIr::Div { contents },
                location: Some(location),
            },
            msgs: warnings,
        }
    }
}

pub(crate) fn parse(
    elements: &Vec<ContentIr>,
    location: &Location,
    opts: Options,
) -> Res<(Vec<ContentIr>, ContinueRunning)> {
    let mut new = vec![];
    let mut continue_running = ContinueRunning::No;

    let mut warnings = vec![];

    for element in elements {
        match element {
            ContentIr::Unparsed {
                contents,
                validations,
            } => {
                // Attempt to process something.
                // If something was found, add it to the end and keep going.
                let ir;
                match try_process(contents, &location, opts, validations) {
                    Res::Ok(item) => ir = item.item,
                    Res::Warn { item, msg } => {
                        warnings.push((msg, item.location.clone()));
                        ir = item.item;
                    }
                    Res::Error { location, msg } => {
                        return Res::Error {
                            location: location.clone(),
                            msg,
                        }
                    }
                    Res::Warnings { item, mut msgs } => {
                        warnings.append(&mut msgs);
                        ir = item.item;
                    }
                }

                if let Some(mut ir) = ir {
                    new.append(&mut ir);
                    continue_running = ContinueRunning::Yes;
                }
                // Nothing matched, so return it as text.
                else if contents.len() > 0 {
                    let text: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();

                    // Validate each is under 350 characters for SEO/engagement purposes
                    const MAX_TEXT_CHARS: usize = 350;
                    const DISPLAY_TEXT_LEN: usize = 20;

                    for text in text.iter() {
                        let char_count = text.chars().count();

                        if char_count > MAX_TEXT_CHARS && validations.validate_paragraph_len {
                            let display_text =
                                text[0..char_count.min(DISPLAY_TEXT_LEN)].to_string();

                            let display_text = if DISPLAY_TEXT_LEN < display_text.chars().count() {
                                format!("{}...", display_text)
                            } else {
                                display_text
                            };

                            let suggested_nl = text
                                [MAX_TEXT_CHARS..char_count.min(MAX_TEXT_CHARS + DISPLAY_TEXT_LEN)]
                                .to_string();

                            warnings.push((
                                format!(
                                    "\n\tParagraph '{}' exceeded limit of {} by {} characters!\n\tSuggested fix is newline at: '{}'",
                                    display_text,
                                    MAX_TEXT_CHARS,
                                    char_count - MAX_TEXT_CHARS,
                                    suggested_nl
                                ),
                                Some(location.clone()),
                            ));
                        }
                    }

                    new.append(
                        &mut text
                            .iter()
                            .map(|t| ContentIr::Text { text: t.clone() })
                            .collect(),
                    );
                }
            }
            _ => {
                new.push(element.clone());
            }
        }
    }

    let item = Item {
        item: (new, continue_running),
        location: Some(location.clone()),
    };

    if warnings.is_empty() {
        Res::Ok(item)
    } else if warnings.len() == 1 {
        Res::Warn {
            item,
            msg: warnings[0].0.clone(),
        }
    } else {
        Res::Warnings {
            item,
            msgs: warnings,
        }
    }
}

/// Attempts to process the contents, stepping through the various types until nothing is left.
fn try_process(
    contents: &String,
    location: &Location,
    opts: Options,
    validations: &Validations,
) -> Res<Option<Vec<ContentIr>>> {
    // Do these in priority.
    // New things that are added should be processed here.
    let mut validations = validations.clone();

    if !validations.skip_code_block {
        if let Some(ir) = chomp::block(contents, opts, &validations) {
            return Res::Ok(Item {
                item: Some(ir),
                location: Some(location.clone()),
            });
        } else {
            validations.skip_code_block = true;
        }
    }

    if !validations.skip_metadata_display_title {
        if let Some(ir) = chomp::metadata_display_title(contents, &validations) {
            return Res::Ok(Item {
                item: Some(ir),
                location: Some(location.clone()),
            });
        } else {
            validations.skip_metadata_display_title = true;
        }
    }

    if !validations.skip_metadata_keywords {
        if let Some(ir) = chomp::metadata_keywords(contents, &validations) {
            return Res::Ok(Item {
                item: Some(ir),
                location: Some(location.clone()),
            });
        } else {
            validations.skip_metadata_keywords = true;
        }
    }

    if !validations.skip_metadata_series {
        if let Some(ir) = chomp::metadata_series(contents, &validations) {
            return Res::Ok(Item {
                item: Some(ir),
                location: Some(location.clone()),
            });
        } else {
            validations.skip_metadata_series = true;
        }
    }

    if !validations.skip_metadata_summary {
        if let Some(ir) = chomp::metadata_summary(contents, &validations) {
            return Res::Ok(Item {
                item: Some(ir),
                location: Some(location.clone()),
            });
        } else {
            validations.skip_metadata_summary = true;
        }
    }

    if !validations.skip_header {
        if let Some(ir) = chomp::header(contents, &validations) {
            return Res::Ok(Item {
                item: Some(ir),
                location: Some(location.clone()),
            });
        } else {
            validations.skip_header = true;
        }
    }

    if !validations.skip_list {
        if let Some(ir) = chomp::list(contents, opts, &validations) {
            return Res::Ok(Item {
                item: Some(ir),
                location: Some(location.clone()),
            });
        } else {
            validations.skip_list = true;
        }
    }

    if !validations.skip_inline_code {
        if let Some(ir) = chomp::inline_code(contents, &validations) {
            return Res::Ok(Item {
                item: Some(ir),
                location: Some(location.clone()),
            });
        } else {
            validations.skip_list = true;
        }
    }

    if !validations.skip_link {
        let mut warnings = vec![];
        let ir;

        match chomp::link(contents, location, &validations) {
            Res::Ok(item) => ir = item.item,
            Res::Warn { item, msg } => {
                ir = item.item;
                warnings.push((msg, item.location.clone()));
            }
            Res::Error { location, msg } => return Res::Error { location, msg },
            Res::Warnings { item, mut msgs } => {
                ir = item.item;
                warnings.append(&mut msgs);
            }
        }

        if let Some(ir) = ir {
            if warnings.is_empty() {
                return Res::Ok(Item {
                    item: Some(ir),
                    location: Some(location.clone()),
                });
            } else if warnings.len() == 1 {
                return Res::Warn {
                    item: Item {
                        item: Some(ir),
                        location: Some(location.clone()),
                    },
                    msg: warnings[0].0.clone(),
                };
            } else {
                return Res::Warnings {
                    item: Item {
                        item: Some(ir),
                        location: Some(location.clone()),
                    },
                    msgs: warnings,
                };
            }
        } else {
            validations.skip_link = true;
        }
    }

    return Res::Ok(Item {
        item: None,
        location: Some(location.clone()),
    });
}
