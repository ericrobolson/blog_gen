use std::sync::Arc;

use crate::{
    crawler::{File, FileKind},
    Context, Item, Location, Res,
};

mod css;
mod img;
mod md;
mod page;

pub use css::Css;
pub use img::Img;
pub use md::{ContentIr, LinkType, Md};
pub use page::Page;

#[derive(Clone, Debug, PartialEq)]
pub enum IntermediateRepresentation {
    Css(Css),
    Img(Img),
    Md(Md),
    Page(Page),
}

pub fn execute(file: &Item<File>, context: Arc<Context>) -> Res<IntermediateRepresentation> {
    let location = Location {
        column: 0,
        file: file.item.file_name.clone(),
        line: 0,
        path: file.item.path.clone(),
    };

    let path = file.item.path.clone();

    match file.item.kind {
        FileKind::Css => css::parse(path, location, context),
        FileKind::Img => img::parse(path, location),
        FileKind::Md => md::parse(path, location),
    }
}
