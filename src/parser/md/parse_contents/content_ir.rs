use crate::{
    html::{
        element::{self, ListType},
        Element, Html, HtmlElement,
    },
    parser::md::Link,
    Links,
};

use super::Validations;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BlockKind {
    Code,
    Quote,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ContentIr {
    Block {
        contents: Box<ContentIr>,
        block_kind: BlockKind,
    },
    Card(Vec<ContentIr>),
    Div {
        contents: Vec<ContentIr>,
    },
    Header {
        header_size: u32,
        id: String,
        text: String,
    },
    InlineCode(String),
    Link {
        description: String,
        url: String,
        link_type: LinkType,
    },
    Metadata(Metadata),
    Navigation {
        links: Links,
        next: Option<Link>,
        previous: Option<Link>,
    },
    OrderedList {
        items: Vec<ContentIr>,
        numeric: bool,
    },
    Paragraph {
        children: Vec<ContentIr>,
    },
    Text {
        text: String,
    },
    Unparsed {
        contents: String,
        validations: Validations,
    },
    UnorderedList {
        items: Vec<ContentIr>,
    },
}

impl ContentIr {
    pub fn section(header: String, mut contents: Vec<ContentIr>) -> Self {
        contents.insert(
            0,
            Self::Header {
                header_size: 3,
                text: header.clone(),
                id: Self::section_header_id(header),
            },
        );

        Self::Card(contents)
    }

    pub fn section_header_id(header: String) -> String {
        format!("{}_headerId", header.to_lowercase().replace(" ", ""))
    }

    pub fn text(text: String) -> Self {
        Self::Text { text }
    }

    pub fn title(text: String, id: String) -> Self {
        Self::Header {
            header_size: 2,
            text,
            id,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LinkType {
    Iframe,
    LocalImage,
    Regular,
    Youtube,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Metadata {
    DisplayTitle(String),
    Keywords(Vec<String>),
    Series(Vec<String>),
    Summary(String),
}

impl HtmlElement for ContentIr {
    fn to_element(&self) -> Element {
        // When modifying this, ensure all text is escaped.
        match self {
            ContentIr::Block {
                contents,
                block_kind,
            } => match block_kind {
                BlockKind::Code => element::code_block(contents.to_element().to_html()),
                BlockKind::Quote => element::card(vec![contents.to_element()], true),
            },
            ContentIr::Card(children) => {
                element::card(children.iter().map(|ir| ir.to_element()).collect(), false)
            }
            ContentIr::Div { contents } => {
                element::div(contents.iter().map(|ir| ir.to_element()).collect())
            }
            ContentIr::Header {
                header_size,
                id,
                text,
            } => {
                let fragment = element::link(None, "#".into(), format!("#{}", id), true);

                element::header(
                    *header_size,
                    Some(id.clone()),
                    vec![fragment, element::text(&escape(text))],
                )
            }
            ContentIr::InlineCode(code) => element::inline_code(escape(&code)),
            ContentIr::Link {
                description,
                url,
                link_type,
            } => match link_type {
                LinkType::Iframe => element::iframe(description.clone(), url.clone()),
                LinkType::LocalImage => element::image(None, description.clone(), url.clone()),
                LinkType::Regular => element::link(None, description.clone(), url.clone(), true),
                LinkType::Youtube => element::youtube(url.clone()),
            },
            ContentIr::OrderedList { items, numeric } => element::list(
                items.iter().map(|i| i.to_element()).collect(),
                if *numeric {
                    ListType::OrderedNumeric
                } else {
                    ListType::OrderedAlpha
                },
            ),
            ContentIr::Metadata(_) => element::nil(),
            ContentIr::Navigation {
                links,
                next,
                previous,
            } => {
                let map = |s: &Option<Link>| match s {
                    Some(link) => Some(link.to_str().to_string()),
                    None => None,
                };

                let bottom_nav = element::bottom_nav(map(previous), map(next));
                let top_nav = element::top_nav(links.clone());

                element::div(vec![bottom_nav, top_nav])
            }
            ContentIr::Paragraph { children } => {
                element::p(children.iter().map(|i| i.to_element()).collect())
            }
            ContentIr::Text { text } => element::text(&escape(text)),
            ContentIr::Unparsed { .. } => {
                todo!("Attempted to convert unparsed content to an element!")
            }
            ContentIr::UnorderedList { items } => element::list(
                items.iter().map(|i| i.to_element()).collect(),
                ListType::Unordered,
            ),
        }
    }
}

fn escape(s: &str) -> String {
    let mut output = String::new();
    html_escape::encode_text_minimal_to_string(s, &mut output);

    output
}
