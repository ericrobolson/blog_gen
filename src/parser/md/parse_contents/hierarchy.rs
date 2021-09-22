use std::ops::Deref;

use super::ContentIr;

pub fn headers(mut ir: Vec<ContentIr>) -> Vec<ContentIr> {
    // Do any hierarchal mappings
    let mut contents = vec![];
    let mut processing_header = None;
    let mut header_collection = vec![];

    while ir.len() > 0 {
        let i = ir.remove(0);

        match &i {
            ContentIr::Header { .. } => {
                // Push the previous header on to the contents
                if let Some(header) = processing_header {
                    let header_children = ContentIr::Div {
                        contents: header_collection,
                    };

                    header_collection = vec![header, header_children];

                    contents.push(ContentIr::Card(header_collection));
                    header_collection = vec![];
                }

                processing_header = Some(i);
            }
            _ => {
                if processing_header.is_some() {
                    header_collection.push(i);
                } else {
                    contents.push(i);
                }
            }
        }
    }

    // If there's a working header, add it to the end
    if let Some(header) = processing_header {
        let header_children = ContentIr::Div {
            contents: header_collection,
        };

        header_collection = vec![header, header_children];

        contents.push(ContentIr::Card(header_collection));
    }

    contents
}

pub fn paragraphs(mut ir: Vec<ContentIr>) -> Vec<ContentIr> {
    let mut contents = vec![];

    let mut paragraph_collection = vec![];
    let mut prev_element_is_text = false;

    while ir.len() > 0 {
        let i = ir.remove(0);

        let paragraph_child;
        let mut is_text = false;

        match i {
            ContentIr::Block { .. } => paragraph_child = false,
            ContentIr::Card(_) => paragraph_child = false,
            ContentIr::Div { .. } => paragraph_child = false,
            ContentIr::Header { .. } => paragraph_child = false,
            ContentIr::InlineCode(_) => paragraph_child = true,
            ContentIr::Link { .. } => paragraph_child = true,
            ContentIr::Navigation { .. } => paragraph_child = false,
            ContentIr::Metadata(_) => paragraph_child = false,
            ContentIr::OrderedList { .. } => paragraph_child = false,
            ContentIr::Paragraph { .. } => paragraph_child = false,
            ContentIr::Text { text: _ } => {
                is_text = true;
                paragraph_child = true;
            }
            ContentIr::Unparsed { .. } => paragraph_child = false,
            ContentIr::UnorderedList { .. } => paragraph_child = false,
        }

        if !paragraph_child {
            if paragraph_collection.len() > 0 {
                contents.push(ContentIr::Paragraph {
                    children: paragraph_collection.clone(),
                });
            }

            paragraph_collection.clear();
            contents.push(i);
        } else {
            if prev_element_is_text && is_text && paragraph_collection.is_empty() == false {
                contents.push(ContentIr::Paragraph {
                    children: paragraph_collection.clone(),
                });
                paragraph_collection.clear();
            }

            paragraph_collection.push(i);
        }

        prev_element_is_text = is_text;
    }

    if paragraph_collection.len() > 0 {
        contents.push(ContentIr::Paragraph {
            children: paragraph_collection,
        });
    }

    // Go through and remove all empty paragraphs

    contents
}

// Removes all empty elements
pub fn clean(mut contents: Vec<ContentIr>) -> Vec<ContentIr> {
    let mut cleaned = vec![];

    while contents.len() > 0 {
        let ir = contents.remove(0);

        let ir = match &ir {
            ContentIr::Block {
                block_kind,
                contents,
            } => {
                let mut contents = clean(vec![contents.deref().clone()]);
                if contents.is_empty() {
                    None
                } else {
                    Some(ContentIr::Block {
                        contents: Box::new(contents.remove(0)),
                        block_kind: *block_kind,
                    })
                }
            }
            ContentIr::Card(contents) => {
                let contents = clean(contents.clone());
                if contents.is_empty() {
                    None
                } else {
                    Some(ContentIr::Card(contents))
                }
            }
            ContentIr::Div { contents } => {
                let contents = clean(contents.clone());
                if contents.is_empty() {
                    None
                } else {
                    Some(ContentIr::Div { contents })
                }
            }
            ContentIr::Header { text, .. } => {
                if text.is_empty() {
                    None
                } else {
                    Some(ir)
                }
            }
            ContentIr::InlineCode(text) => {
                if text.is_empty() {
                    None
                } else {
                    Some(ir)
                }
            }
            ContentIr::Link { .. } => Some(ir),
            ContentIr::Metadata(_) => Some(ir),
            ContentIr::Navigation { .. } => Some(ir),
            ContentIr::OrderedList { items, numeric } => {
                let items = clean(items.clone());
                if items.is_empty() {
                    None
                } else {
                    Some(ContentIr::OrderedList {
                        items,
                        numeric: *numeric,
                    })
                }
            }
            ContentIr::Paragraph { children } => {
                let children = clean(children.clone());
                if children.is_empty() {
                    None
                } else {
                    Some(ContentIr::Paragraph { children })
                }
            }
            ContentIr::Text { text } => {
                if text.is_empty() {
                    None
                } else {
                    Some(ir)
                }
            }
            ContentIr::Unparsed { .. } => Some(ir),
            ContentIr::UnorderedList { items } => {
                let items = clean(items.clone());
                if items.is_empty() {
                    None
                } else {
                    Some(ContentIr::UnorderedList { items })
                }
            }
        };

        if let Some(ir) = ir {
            cleaned.push(ir);
        }
    }

    cleaned
}
