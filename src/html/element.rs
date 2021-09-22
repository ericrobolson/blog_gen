use crate::{html::stylesheet::Alignment, Links};

use super::{
    stylesheet::{Class, Position},
    Html,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Element {
    pub classes: Vec<Class>,
    pub id: Option<String>,
    pub kind: Kind,
}

impl Html for Element {
    fn to_html(&self) -> String {
        let css = {
            if self.classes.is_empty() {
                String::default()
            } else {
                format!(
                    "class=\"{}\"",
                    self.classes
                        .iter()
                        .map(|c| c.selector().to_str().into())
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            }
        };

        let selector;
        let mut children = String::new();
        let mut attributes: Option<String> = None;

        match &self.kind {
            Kind::Card(contents) => {
                selector = String::from("div");
                children = contents.clone();
            }
            Kind::Codeblock(text) => {
                selector = String::from("div");
                children = text.clone();
            }
            Kind::Div {
                children: child_elements,
            } => {
                selector = String::from("div");
                children = child_elements
                    .iter()
                    .map(|c| c.to_html())
                    .collect::<Vec<String>>()
                    .join("\n");
            }
            Kind::Header {
                header_size,
                children: child_elements,
            } => {
                selector = format!("h{}", header_size);
                children = child_elements
                    .iter()
                    .map(|c| c.to_html())
                    .collect::<Vec<String>>()
                    .join("\n");
            }
            Kind::List { items, list_type } => {
                let (s, attr) = match list_type {
                    ListType::OrderedAlpha => (String::from("ol"), Some("type=\"a\"".into())),
                    ListType::OrderedNumeric => (String::from("ol"), Some("type=\"1\"".into())),
                    ListType::Unordered => (String::from("ul"), None),
                };

                attributes = attr;
                children = items
                    .iter()
                    .map(|e| e.to_html())
                    .map(|e| format!("<li>{}</li>", e))
                    .collect();
                selector = s;
            }
            Kind::Link {
                description,
                redirect,
                url,
            } => {
                let redirect_text = if *redirect {
                    ""
                } else {
                    "target=\"_blank\" rel=\"noreferrer noopener\""
                };

                selector = String::from("a");
                children = description.clone();
                attributes = Some(format!("href=\"{}\" {}", url, redirect_text));
            }
            Kind::Iframe { description, url } => {
                selector = String::from("iframe");
                attributes = Some(
                    format!("
                    src=\"{}\" 
                    title=\"{}\" 
                    frameborder=\"0\" 
                    allow=\"accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture\" 
                    allowfullscreen",
                    url, description));
            }
            Kind::Image { description, url } => {
                selector = String::from("img");
                attributes = Some(format!("src=\"{}\" alt=\"{}\"", url, description));
            }
            Kind::Nil => return String::default(),
            Kind::Paragraph { children: c } => {
                selector = String::from("p");
                children = c.iter().map(|e| e.to_html()).collect();
            }
            Kind::Span { children: c } => {
                selector = String::from("span");
                children = c.iter().map(|e| e.to_html()).collect();
            }
            Kind::Text(text) => return text.clone(),
            Kind::Youtube { url } => {
                selector = String::from("iframe");
                attributes = Some(
                    format!("
                        src=\"{}\" 
                        title=\"YouTube video player\" 
                        frameborder=\"0\" 
                        allow=\"accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture\" 
                        allowfullscreen",
                        url));
            }
        }

        let attributes = match attributes {
            Some(s) => s,
            None => String::default(),
        };

        let id = match &self.id {
            Some(id) => format!("id=\"{}\"", id),
            None => String::default(),
        };

        format!(
            "<{} {} {} {}>{}</{}>",
            selector, id, attributes, css, children, selector
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    Card(String),
    Codeblock(String),
    Div {
        children: Vec<Element>,
    },
    Header {
        header_size: u32,
        children: Vec<Element>,
    },
    Iframe {
        description: String,
        url: String,
    },
    Image {
        description: String,
        url: String,
    },
    Link {
        description: String,
        redirect: bool,
        url: String,
    },
    List {
        items: Vec<Element>,
        list_type: ListType,
    },
    Nil,
    Paragraph {
        children: Vec<Element>,
    },
    Span {
        children: Vec<Element>,
    },
    Text(String),
    Youtube {
        url: String,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum ListType {
    OrderedAlpha,
    OrderedNumeric,
    Unordered,
}

fn aspect_ratio_16x9_container(content: Element) -> Element {
    Element {
        classes: vec![
            Class::AspectRatio16x9Container,
            Class::HoverShadow,
            Class::Padded {
                position: Position::Bottom,
            },
            Class::Padded {
                position: Position::Top,
            },
            Class::Shadow,
        ],
        id: None,
        kind: Kind::Div {
            children: vec![content],
        },
    }
}

pub fn bottom_nav(previous: Option<String>, next: Option<String>) -> Element {
    let previous = make_link(previous, "🡄 Previous".into(), "previous_page_link".into());
    let next = make_link(next, "Forward 🡆".into(), "next_page_link".into());

    Element {
        classes: vec![Class::Navigation {
            position: Position::Bottom,
        }],
        id: None,
        kind: Kind::Div {
            children: vec![previous, next],
        },
    }
}

pub fn card(children: Vec<Element>, dark_mode: bool) -> Element {
    Element {
        classes: vec![
            Class::Card { dark_mode },
            Class::HoverShadow,
            Class::Outlined { dark_mode },
            Class::Padded {
                position: Position::Bottom,
            },
            Class::Padded {
                position: Position::Top,
            },
            Class::Shadow,
            Class::TextAlign {
                alignment: Alignment::Left,
            },
        ],
        id: None,
        kind: Kind::Div { children },
    }
}

pub fn code_block(code: String) -> Element {
    Element {
        classes: vec![
            Class::Codeblock,
            Class::ContentWrapper,
            Class::HoverShadow,
            Class::Outlined {
                dark_mode: Class::Codeblock.dark_mode(),
            },
            Class::Padded {
                position: Position::Bottom,
            },
            Class::Padded {
                position: Position::Top,
            },
            Class::Shadow,
        ],
        id: None,
        kind: Kind::Codeblock(code),
    }
}

pub fn div(children: Vec<Element>) -> Element {
    Element {
        classes: vec![],
        id: None,
        kind: Kind::Div { children },
    }
}

pub fn header(header_size: u32, id: Option<String>, children: Vec<Element>) -> Element {
    Element {
        classes: vec![],
        id,
        kind: Kind::Header {
            children,
            header_size,
        },
    }
}

pub fn iframe(description: String, url: String) -> Element {
    aspect_ratio_16x9_container(Element {
        classes: vec![Class::AspectRatio16x9Inner],
        id: None,
        kind: Kind::Iframe { description, url },
    })
}

pub fn image(id: Option<String>, description: String, url: String) -> Element {
    let img_element = Element {
        classes: vec![
            Class::FullImg,
            Class::HoverShadow,
            Class::Padded {
                position: Position::Bottom,
            },
            Class::Padded {
                position: Position::Top,
            },
            Class::Shadow,
        ],
        id,
        kind: Kind::Image {
            description,
            url: url.clone(),
        },
    }
    .to_html();

    Element {
        classes: vec![],
        id: None,
        kind: Kind::Link {
            description: img_element,
            redirect: false,
            url,
        },
    }
}

pub fn inline_code(code: String) -> Element {
    Element {
        classes: vec![
            Class::Codeblock,
            Class::Outlined {
                dark_mode: Class::Codeblock.dark_mode(),
            },
        ],
        id: None,
        kind: Kind::Span {
            children: vec![text(&code)],
        },
    }
}

pub fn link(id: Option<String>, description: String, url: String, redirect: bool) -> Element {
    Element {
        classes: vec![],
        id,
        kind: Kind::Link {
            description,
            redirect,
            url,
        },
    }
}

pub fn list(items: Vec<Element>, list_type: ListType) -> Element {
    Element {
        classes: vec![Class::List],
        id: None,
        kind: Kind::List { items, list_type },
    }
}

fn make_link(url: Option<String>, description: String, id: String) -> Element {
    let disabled = url.is_none();
    let url = url.unwrap_or_default();

    match disabled {
        true => text(&description),
        false => link(Some(id), description, url, true),
    }
}

pub fn nil() -> Element {
    Element {
        classes: vec![],
        id: None,
        kind: Kind::Nil,
    }
}

pub fn p(children: Vec<Element>) -> Element {
    Element {
        classes: vec![],
        id: None,
        kind: Kind::Paragraph { children },
    }
}

pub fn text(s: &str) -> Element {
    Element {
        classes: vec![],
        id: None,
        kind: Kind::Text(s.into()),
    }
}

pub fn top_nav(links: Links) -> Element {
    let children = {
        let about_me = make_link(
            Some(links.about_me_link),
            "About Me".into(),
            "aboutMeNav".into(),
        );

        let catalog = make_link(
            Some(links.catalog_link),
            "Catalog".into(),
            "catalogNav".into(),
        );

        let index = make_link(Some(links.index_link), "Home".into(), "homeNav".into());

        let projects = make_link(
            Some(links.projects_link),
            "Projects".into(),
            "projectsNav".into(),
        );

        vec![index, catalog, projects, about_me]
    };

    Element {
        classes: vec![Class::Navigation {
            position: Position::Top,
        }],
        id: None,
        kind: Kind::Div { children },
    }
}

pub fn youtube(url: String) -> Element {
    aspect_ratio_16x9_container(Element {
        classes: vec![Class::AspectRatio16x9Inner],
        id: None,
        kind: Kind::Youtube { url },
    })
}
