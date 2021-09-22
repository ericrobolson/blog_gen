use crate::html::stylesheet;

use super::{CssLink, Element, Html, Metadata};

#[derive(Clone, Debug, PartialEq)]
pub struct Page {
    pub content: Element,
    pub css: CssLink,
    pub file_name: String,
    pub metadata: Metadata,
    pub title: String,
}

impl Html for Page {
    fn to_html(&self) -> String {
        format!(
            "
<!DOCTYPE html>
<html>
    <head>
        <title>{}</title>
        <!-- metadata -->
        {}
        <!-- css -->
        {}
    </head>
    <body>
        <!-- page class -->
        <div class={}>
            <!-- content wrapper -->
            <div class={}>
                <!-- content -->
                {}
            </div>
        </div>
    </body>
</html>
",
            self.title,
            self.metadata.to_html(),
            self.css.to_html(),
            stylesheet::Class::Page.selector().to_str(),
            stylesheet::Class::ContentWrapper.selector().to_str(),
            self.content.to_html()
        )
    }
}
