mod css_link;
pub mod element;
mod metadata;
mod page;
mod stylesheet;

pub use css_link::CssLink;
pub use element::Element;
pub use metadata::Metadata;
pub use page::Page;
pub use stylesheet::Stylesheet;

pub trait Html {
    fn to_html(&self) -> String;
}

pub trait HtmlElement {
    fn to_element(&self) -> Element;
}
