use super::Html;
use super::*;
use crate::theme::Theme;
pub use css::Css;
pub use selector::Selector;

#[derive(Debug, Clone, PartialEq)]
pub enum Class {
    AspectRatio16x9Container,
    AspectRatio16x9Inner,
    Card { dark_mode: bool },
    Codeblock,
    ContentWrapper,
    FullImg,
    HoverShadow,
    List,
    Navigation { position: Position },
    Outlined { dark_mode: bool },
    Padded { position: Position },
    Page,
    Shadow,
    TextAlign { alignment: Alignment },
}

impl Class {
    pub fn all() -> Vec<Self> {
        vec![
            Class::AspectRatio16x9Container,
            Class::AspectRatio16x9Inner,
            Class::Card { dark_mode: false },
            Class::Card { dark_mode: true },
            Class::Codeblock,
            Class::ContentWrapper,
            Class::FullImg,
            Class::HoverShadow,
            Class::List,
            Class::Outlined { dark_mode: false },
            Class::Outlined { dark_mode: true },
            Class::Navigation {
                position: Position::Bottom,
            },
            Class::Navigation {
                position: Position::Left,
            },
            Class::Navigation {
                position: Position::Right,
            },
            Class::Navigation {
                position: Position::Top,
            },
            Class::Padded {
                position: Position::Bottom,
            },
            Self::Padded {
                position: Position::Left,
            },
            Self::Padded {
                position: Position::Right,
            },
            Self::Padded {
                position: Position::Top,
            },
            Self::Page,
            Self::Shadow,
            Self::TextAlign {
                alignment: Alignment::Center,
            },
            Self::TextAlign {
                alignment: Alignment::Justify,
            },
            Self::TextAlign {
                alignment: Alignment::Left,
            },
            Self::TextAlign {
                alignment: Alignment::Right,
            },
        ]
    }

    pub fn css(&self, theme: &Theme) -> Css {
        match self {
            Class::AspectRatio16x9Container => aspect_ratio_16x9_container(self),
            Class::AspectRatio16x9Inner => aspect_ratio_16x9_inner(self),
            Class::Card { dark_mode } => card(self, *dark_mode, theme),
            Class::Codeblock => codeblock(self, theme),
            Class::ContentWrapper => content_wrapper(self),
            Class::FullImg => full_img(self),
            Class::HoverShadow => hover_shadow(self),
            Class::List => list(self),
            Class::Navigation { position } => nav(self, position, theme),
            Class::Outlined { dark_mode } => outline(self, *dark_mode, theme),
            Class::Padded { position } => padded(self, *position),
            Class::Page => page(self),
            Class::Shadow => shadow(self),
            Class::TextAlign { alignment } => text_align(self, alignment),
        }
    }

    pub fn dark_mode(&self) -> bool {
        match self {
            Class::AspectRatio16x9Container => false,
            Class::AspectRatio16x9Inner => false,
            Class::Card { dark_mode } => *dark_mode,
            Class::Codeblock => true,
            Class::ContentWrapper => false,
            Class::FullImg => false,
            Class::HoverShadow => false,
            Class::List => false,
            Class::Navigation { .. } => false,
            Class::Outlined { dark_mode } => *dark_mode,
            Class::Padded { .. } => false,
            Class::Page => false,
            Class::Shadow => false,
            Class::TextAlign { .. } => false,
        }
    }

    pub fn selector(&self) -> Selector {
        let prefix: String = match self {
            Class::AspectRatio16x9Container => "aspectRatio16x9Container".into(),
            Class::AspectRatio16x9Inner => "aspectRatio16x9Inner".into(),
            Class::Card { .. } => "card".into(),
            Class::Codeblock => "codeblock".into(),
            Class::ContentWrapper => "contentWrapper".into(),
            Class::FullImg => "fullImg".into(),
            Class::HoverShadow => "hoverShadow".into(),
            Class::List => "list".into(),
            Class::Navigation { position } => format!("nav{}", position.to_str()),
            Class::Outlined { .. } => "outlined".into(),
            Class::Padded { position } => format!("padded{}", position.to_str()),
            Class::Page => "page".into(),
            Class::Shadow => "shadow".into(),
            Class::TextAlign { alignment } => format!("alignment{}", alignment.to_str()),
        };

        let post_fix = match self.dark_mode() {
            false => "Light",
            true => "Dark",
        };

        format!("{}{}", prefix, post_fix).into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Alignment {
    Center,
    Justify,
    Left,
    Right,
}

impl Alignment {
    pub fn to_str(&self) -> &'static str {
        match self {
            Alignment::Center => "center",
            Alignment::Justify => "justify",
            Alignment::Left => "left",
            Alignment::Right => "right",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Position {
    Bottom,
    Left,
    Right,
    Top,
}

impl Position {
    pub fn to_str(&self) -> &'static str {
        match self {
            Position::Bottom => "bottom",
            Position::Left => "left",
            Position::Right => "right",
            Position::Top => "top",
        }
    }
}

pub fn anchor(theme: &Theme) -> Css {
    let main_rules = vec![
        format!("color: {}", theme.accent_blue.to_html()),
        "text-overflow: ellipsis".into(),
    ];

    let main_class = format!("a {{ {} }}", combine_rules(main_rules));

    main_class.into()
}

fn aspect_ratio_16x9_container(class: &Class) -> Css {
    let main_rules = vec![
        "height: 0px".into(),
        "padding-bottom: 56.25%".into(),
        "position: relative".into(),
        "width: 100%".into(),
    ];

    let main_class = format!(
        ".{} {{ {} }}",
        class.selector().to_str(),
        combine_rules(main_rules)
    );

    main_class.into()
}

fn aspect_ratio_16x9_inner(class: &Class) -> Css {
    let main_rules = vec![
        "height: 100%".into(),
        "left: 0".into(),
        "position: absolute".into(),
        "top: 0".into(),
        "width: 100%".into(),
    ];

    let main_class = format!(
        ".{} {{ {} }}",
        class.selector().to_str(),
        combine_rules(main_rules)
    );

    main_class.into()
}

pub fn body(theme: &Theme) -> Css {
    let nav_padding = "64px";
    let mobile_top_nav_padding = "128px";

    let rules = vec![
        format!("background: {}", theme.fill_secondary_light_mode.to_html()),
        format!("color: {}", theme.text_primary_light_mode.to_html()),
        "font-family: Arial, Helvetica, sans-serif".into(),
        "font-size: 18pt".into(),
        "font-weight: 400".into(),
        "margin: auto".into(),
        "max-width: 100vw".into(),
        format!("padding: {} 0 {} 0", nav_padding, nav_padding),
        "text-align: center".into(),
    ];
    let selector = "body";

    let main_class = format!("{} {{ {} }}", selector, combine_rules(rules));

    media_screen_query(
        main_class,
        false,
        selector,
        TOP_NAV_COLUMN_PX_LIMIT,
        vec![format!(
            "padding: {} 0 {} 0",
            mobile_top_nav_padding, nav_padding
        )],
    )
}

fn card(class: &Class, dark_mode: bool, theme: &Theme) -> Css {
    let background_color = match dark_mode {
        true => theme.fill_primary_dark_mode.to_html(),
        false => theme.fill_primary_light_mode.to_html(),
    };

    let text_color = match dark_mode {
        true => theme.text_primary_dark_mode.to_html(),
        false => theme.text_primary_light_mode.to_html(),
    };

    let main_rules = vec![
        format!("background: {}", background_color),
        format!("color: {}", text_color),
        "display: flex".into(),
        "flex-direction: column".into(),
        "padding: 16px".into(),
        "text-align: justify".into(),
        "width: 100%".into(),
        "word-wrap: normal".into(),
    ];

    let main_class = format!(
        ".{} {{ {} }}",
        class.selector().to_str(),
        combine_rules(main_rules)
    );

    main_class.into()
}

fn codeblock(class: &Class, theme: &Theme) -> Css {
    let rules = vec![
        format!("background: {}", theme.fill_primary_dark_mode.to_html()),
        format!("border-color: {}", theme.fill_secondary_dark_mode.to_html()),
        format!("color: {}", theme.text_primary_dark_mode.to_html()),
        "flex: none".into(),
        "line-height: 1.25".into(),
        "overflow-x: auto".into(),
        "padding-left: 8px".into(),
        "padding-right: 8px".into(),
        "text-align: left".into(),
        "text-justify: none".into(),
        "white-space: pre".into(),
    ];

    let main_class = format!(
        ".{} {{ {} }}",
        class.selector().to_str(),
        combine_rules(rules)
    );

    main_class.into()
}

fn content_wrapper(class: &Class) -> Css {
    format!(
        ".{} {{ 
            padding: 16px;
        }}        
        ",
        class.selector().to_str()
    )
    .into()
}

fn full_img(class: &Class) -> Css {
    let main_rules = vec![
        "height: auto".into(),
        "max-width: 100%".into(),
        "width: 100%".into(),
    ];

    let main_class = format!(
        ".{} {{ {} }}",
        class.selector().to_str(),
        combine_rules(main_rules)
    );

    main_class.into()
}

fn hover_shadow(class: &Class) -> Css {
    let hover_rules: Vec<String> = vec!["box-shadow: 0 8px 12px 0 rgba(0, 0, 0, 0.2)".into()];

    let hover_class = format!(
        ".{}:hover {{ {} }}",
        class.selector().to_str(),
        combine_rules(hover_rules)
    );

    hover_class.into()
}

pub fn html() -> Css {
    // Ensure it doesn't have a horizontal scroll bar
    String::from(
        "
html {
    box-sizing: border-box;
    height: 100%;
        overflow-y: scroll;
}

*,
*:before,
*:after {
    box-sizing: inherit;
}

",
    )
    .into()
}

fn list(class: &Class) -> Css {
    let main_rules = vec![
        "display: flex".into(),
        "flex-direction: column".into(),
        "align-content: left".into(),
        "text-align: left".into(),
    ];

    let main_class = format!(
        ".{} {{ {} }}",
        class.selector().to_str(),
        combine_rules(main_rules)
    );

    format!("{}", main_class).into()
}

fn media_screen_query(
    main_class: String,
    is_custom_class: bool,
    id: &str,
    max_width_pixels: u32,
    media_rules: Vec<String>,
) -> Css {
    let size_rules = format!("(max-width: {}px)", max_width_pixels);

    let prefix = if is_custom_class { "." } else { "" };

    let media_class = format!("{}{} {{ {} }}", prefix, id, combine_rules(media_rules));

    let media_class = format!("@media screen and {} {{ {} }}", size_rules, media_class);

    format!("{}\n{}", main_class, media_class).into()
}

const TOP_NAV_COLUMN_PX_LIMIT: u32 = 400;

pub fn nav(class: &Class, position: &Position, theme: &Theme) -> Css {
    let rules = vec![
        "align-content: center".into(),
        format!("background: {}", theme.fill_secondary_light_mode.to_html()),
        match position {
            Position::Top => "top: 0",
            Position::Bottom | _ => "bottom: 0",
        }
        .into(),
        "display: flex".into(),
        "flex-direction: row".into(),
        "justify-content: space-evenly".into(),
        "left: 0".into(),
        "padding: 16px".into(),
        "position: fixed".into(),
        "width: 100%".into(),
    ];

    let main_class = format!(
        ".{} {{ {} }}",
        class.selector().to_str(),
        combine_rules(rules)
    );

    if *position == Position::Top {
        media_screen_query(
            main_class,
            true,
            class.selector().to_str(),
            TOP_NAV_COLUMN_PX_LIMIT,
            vec!["flex-direction: column".into()],
        )
    } else {
        main_class.into()
    }
}

fn outline(class: &Class, dark_mode: bool, theme: &Theme) -> Css {
    let color = match dark_mode {
        true => theme.fill_secondary_dark_mode.to_html(),
        false => theme.fill_secondary_light_mode.to_html(),
    };

    let main_rules = vec![
        "border: 1px solid".into(),
        format!("border-color: {}", color).into(),
        "border-radius: 8px".into(),
    ];

    let main_class = format!(
        ".{} {{ {} }}",
        class.selector().to_str(),
        combine_rules(main_rules)
    );

    format!("{}", main_class).into()
}

fn padded(class: &Class, position: Position) -> Css {
    let margin_position = match position {
        Position::Bottom => "bottom",
        Position::Left => "left",
        Position::Right => "right",
        Position::Top => "top",
    };

    format!(
        "
.{} {{
    margin-{}: 16px;
}}",
        class.selector().to_str(),
        margin_position,
    )
    .into()
}

fn page(class: &Class) -> Css {
    format!(
        "
.{} {{
    align-content: center;            
    display: flex;
    flex-direction: column;
    justify-content: center;
    max-width: 100vw;
    width: 100vw - (100vw - 100%);
}}",
        class.selector().to_str()
    )
    .into()
}

fn shadow(class: &Class) -> Css {
    let main_rules = vec!["box-shadow: 0 4px 8px 0 rgba(0, 0, 0, 0.2)".into()];

    let main_class = format!(
        ".{} {{ {} }}",
        class.selector().to_str(),
        combine_rules(main_rules)
    );

    main_class.into()
}

fn text_align(class: &Class, alignment: &Alignment) -> Css {
    let main_rules = vec![format!("text-align: {}", alignment.to_str())];

    let main_class = format!(
        ".{} {{ {} }}",
        class.selector().to_str(),
        combine_rules(main_rules)
    );

    main_class.into()
}

fn combine_rules(rules: Vec<String>) -> String {
    rules
        .iter()
        .map(|r| format!("{};", r))
        .collect::<Vec<String>>()
        .join("\n")
}
