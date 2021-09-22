#[derive(Clone, Debug, PartialEq)]
pub struct Directory(String);

impl Directory {
    pub fn to_string(&self) -> String {
        let Self(s) = self;

        s.to_string()
    }
}

impl From<String> for Directory {
    fn from(s: String) -> Self {
        Self(s)
    }
}
