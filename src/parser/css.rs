use super::IntermediateRepresentation;
use crate::{html::Html, Context, Item, Location, Res};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct Css {
    pub contents: Vec<u8>,
    pub file_name: String,
}

pub fn parse(
    file_path: String,
    location: Location,
    context: Arc<Context>,
) -> Res<IntermediateRepresentation> {
    let contents = match std::fs::read_to_string(&file_path) {
        Ok(c) => c,
        Err(e) => {
            return Res::Error {
                msg: format!("{:#?}", e),
                location: Some(location),
            }
        }
    };

    // Preprocess CSS.
    let contents = {
        const TEXT_PRIMARY_LIGHT_MODE: &'static str = "_TXT_L_PRIMARY";
        const TEXT_SECONDARY_LIGHT_MODE: &'static str = "_TXT_L_SECONDARY";
        const TEXT_PRIMARY_DARK_MODE: &'static str = "_TXT_D_PRIMARY";
        const TEXT_SECONDARY_DARK_MODE: &'static str = "_TXT_D_SECONDARY";

        const FILL_PRIMARY_LIGHT_MODE: &'static str = "_FILL_L_PRIMARY";
        const FILL_SECONDARY_LIGHT_MODE: &'static str = "_FILL_L_SECONDARY";
        const FILL_PRIMARY_DARK_MODE: &'static str = "_FILL_D_PRIMARY";
        const FILL_SECONDARY_DARK_MODE: &'static str = "_FILL_D_SECONDARY";

        const ACCENT_RED: &'static str = "_ACCENT_RED";
        const ACCENT_GREEN: &'static str = "_ACCENT_GREEN";
        const ACCENT_BLUE: &'static str = "_ACCENT_BLUE";

        let colors = &context.config.theme;

        contents
            .replace(
                TEXT_PRIMARY_LIGHT_MODE,
                &colors.text_primary_light_mode.to_html(),
            )
            .replace(
                TEXT_SECONDARY_LIGHT_MODE,
                &colors.text_secondary_light_mode.to_html(),
            )
            .replace(
                TEXT_PRIMARY_DARK_MODE,
                &colors.text_primary_dark_mode.to_html(),
            )
            .replace(
                TEXT_SECONDARY_DARK_MODE,
                &colors.text_secondary_dark_mode.to_html(),
            )
            .replace(
                FILL_PRIMARY_LIGHT_MODE,
                &colors.fill_primary_light_mode.to_html(),
            )
            .replace(
                FILL_SECONDARY_LIGHT_MODE,
                &colors.fill_secondary_light_mode.to_html(),
            )
            .replace(
                FILL_PRIMARY_DARK_MODE,
                &colors.fill_primary_dark_mode.to_html(),
            )
            .replace(
                FILL_SECONDARY_DARK_MODE,
                &colors.fill_secondary_dark_mode.to_html(),
            )
            .replace(ACCENT_RED, &colors.accent_red.to_html())
            .replace(ACCENT_GREEN, &colors.accent_green.to_html())
            .replace(ACCENT_BLUE, &colors.accent_blue.to_html())
    };

    Res::Ok(Item {
        item: IntermediateRepresentation::Css(Css {
            contents: contents.into_bytes(),
            file_name: location.file.clone(),
        }),
        location: Some(location),
    })
}
