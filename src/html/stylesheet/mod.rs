mod class;
mod css;
mod selector;

use super::Html;
use crate::{theme::Theme, Context};
pub use class::{Alignment, Class, Position};
pub use css::Css;
pub use selector::Selector;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct Stylesheet {
    classes: Vec<Class>,
    theme: Theme,
}

impl Stylesheet {
    pub fn new(context: &Arc<Context>) -> Self {
        Self {
            classes: Class::all(),
            theme: context.config.theme.clone(),
        }
    }
}

impl Html for Stylesheet {
    fn to_html(&self) -> String {
        let mut generated = self
            .classes
            .iter()
            .map(|c| c.css(&self.theme))
            .collect::<Vec<Css>>();

        generated.push(class::anchor(&self.theme));
        generated.push(class::body(&self.theme));
        generated.push(class::html());

        let generated_css = generated
            .iter()
            .map(|css| css.to_str().into())
            .collect::<Vec<String>>()
            .join("\n");

        format!("{}", generated_css)
    }
}
