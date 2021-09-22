use crate::html::Html;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct Theme {
    pub accent_blue: Color,
    pub accent_green: Color,
    pub accent_red: Color,
    pub fill_primary_dark_mode: Color,
    pub fill_primary_light_mode: Color,
    pub fill_secondary_dark_mode: Color,
    pub fill_secondary_light_mode: Color,
    pub text_primary_dark_mode: Color,
    pub text_primary_light_mode: Color,
    pub text_secondary_dark_mode: Color,
    pub text_secondary_light_mode: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            accent_blue: [30, 71, 246].into(),
            accent_green: [104, 159, 57].into(),
            accent_red: [204, 0, 0].into(),
            fill_primary_dark_mode: [37, 37, 37].into(),
            fill_primary_light_mode: [255, 255, 255].into(),
            fill_secondary_dark_mode: [0, 0, 0].into(),
            fill_secondary_light_mode: [224, 224, 224].into(),
            text_primary_dark_mode: [255, 255, 255].into(),
            text_primary_light_mode: [3, 3, 3].into(),
            text_secondary_dark_mode: [162, 162, 162].into(),
            text_secondary_light_mode: [96, 96, 96].into(),
        }
    }
}

#[derive(Copy, Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Html for Color {
    fn to_html(&self) -> String {
        format!("rgba({},{},{},{})", self.r, self.g, self.b, self.a)
    }
}

impl From<[u8; 3]> for Color {
    fn from([r, g, b]: [u8; 3]) -> Self {
        Self {
            r,
            g,
            b,
            a: u8::MAX,
        }
    }
}
