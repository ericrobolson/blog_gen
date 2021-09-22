use super::Html;

#[derive(Clone, Debug, PartialEq)]
pub struct CssLink(String);

impl Html for CssLink {
    fn to_html(&self) -> String {
        format!("<link rel=\"stylesheet\" href=\"{}\">", self.0)
    }
}

impl From<String> for CssLink {
    fn from(s: String) -> Self {
        Self(s)
    }
}
