#[derive(Debug, Clone, PartialEq)]
pub struct Css(String);

impl Css {
    pub fn to_str(&self) -> &str {
        &self.0
    }
}

impl From<&'static str> for Css {
    fn from(s: &'static str) -> Self {
        Self(s.into())
    }
}

impl From<String> for Css {
    fn from(s: String) -> Self {
        Self(s)
    }
}
