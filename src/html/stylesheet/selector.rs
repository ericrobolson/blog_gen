#[derive(Debug, Clone, PartialEq)]
pub struct Selector(String);

impl Selector {
    pub fn to_str(&self) -> &str {
        &self.0
    }
}

impl From<&'static str> for Selector {
    fn from(s: &'static str) -> Self {
        Self(s.into())
    }
}

impl From<String> for Selector {
    fn from(s: String) -> Self {
        Self(s)
    }
}
