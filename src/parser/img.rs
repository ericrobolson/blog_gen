use super::IntermediateRepresentation;
use crate::{Item, Location, Res};

#[derive(Clone, Debug, PartialEq)]
pub struct Img {
    pub contents: Vec<u8>,
    pub file_name: String,
}

pub fn parse(file_path: String, location: Location) -> Res<IntermediateRepresentation> {
    let contents = match std::fs::read(&file_path) {
        Ok(c) => c,
        Err(e) => {
            return Res::Error {
                msg: format!("{:#?}", e),
                location: Some(location),
            }
        }
    };

    Res::Ok(Item {
        item: IntermediateRepresentation::Img(Img {
            contents,
            file_name: location.file.clone(),
        }),
        location: Some(location),
    })
}
