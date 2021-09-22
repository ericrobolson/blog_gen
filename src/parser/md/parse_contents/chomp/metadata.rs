use crate::parser::md::{parse_contents::Validations, ContentIr, Metadata};

pub fn metadata_display_title(
    contents: &String,
    validations: &Validations,
) -> Option<Vec<ContentIr>> {
    // This should only be parsed once, at the very top of the file.
    let mut validations = validations.clone();
    validations.skip_metadata_display_title = true;

    // Try to split it on the large chunks.
    const START: &'static str = "!!title";
    if let Some(start_idx) = contents.find(START) {
        let (before, start) = contents.split_at(start_idx);
        let start = start.replacen(START, "", 1);

        if let Some(end_idx) = start.find("\n") {
            let (content, after) = start.split_at(end_idx);
            let after = after.replacen("\n", "", 1);

            let mut ir = vec![];

            if before.len() > 0 {
                ir.push(ContentIr::Unparsed {
                    contents: before.into(),
                    validations: validations.clone(),
                });
            }

            ir.push(ContentIr::Metadata(Metadata::DisplayTitle(
                content.trim().into(),
            )));

            if after.len() > 0 {
                ir.push(ContentIr::Unparsed {
                    contents: after.into(),
                    validations: validations.clone(),
                });
            }

            return Some(ir);
        }
    }

    None
}

pub fn metadata_keywords(contents: &String, validations: &Validations) -> Option<Vec<ContentIr>> {
    // This should only be parsed once, at the very top of the file.
    let mut validations = validations.clone();
    validations.skip_metadata_keywords = true;

    // Try to split it on the large chunks.
    const START: &'static str = "!!keywords";
    if let Some(start_idx) = contents.find(START) {
        let (before, start) = contents.split_at(start_idx);
        let start = start.replacen(START, "", 1);

        if let Some(end_idx) = start.find("\n") {
            let (content, after) = start.split_at(end_idx);
            let after = after.replacen("\n", "", 1);

            let mut ir = vec![];

            if before.len() > 0 {
                ir.push(ContentIr::Unparsed {
                    contents: before.into(),
                    validations: validations.clone(),
                });
            }

            ir.push(ContentIr::Metadata(Metadata::Keywords(
                content
                    .trim()
                    .split_ascii_whitespace()
                    .map(|s| s.to_string())
                    .collect(),
            )));

            if after.len() > 0 {
                ir.push(ContentIr::Unparsed {
                    contents: after.into(),
                    validations: validations.clone(),
                });
            }

            return Some(ir);
        }
    }

    None
}

pub fn metadata_series(contents: &String, validations: &Validations) -> Option<Vec<ContentIr>> {
    // This should only be parsed once, at the very top of the file.
    let mut validations = validations.clone();
    validations.skip_metadata_series = true;

    // Try to split it on the large chunks.
    const START: &'static str = "!!series";
    if let Some(start_idx) = contents.find(START) {
        let (before, start) = contents.split_at(start_idx);
        let start = start.replacen(START, "", 1);

        if let Some(end_idx) = start.find("\n") {
            let (content, after) = start.split_at(end_idx);
            let after = after.replacen("\n", "", 1);

            let mut ir = vec![];

            if before.len() > 0 {
                ir.push(ContentIr::Unparsed {
                    contents: before.into(),
                    validations: validations.clone(),
                });
            }

            ir.push(ContentIr::Metadata(Metadata::Series(
                content
                    .trim()
                    .split_ascii_whitespace()
                    .map(|s| s.to_string())
                    .collect(),
            )));

            if after.len() > 0 {
                ir.push(ContentIr::Unparsed {
                    contents: after.into(),
                    validations: validations.clone(),
                });
            }

            return Some(ir);
        }
    }

    None
}

pub fn metadata_summary(contents: &String, validations: &Validations) -> Option<Vec<ContentIr>> {
    // This should only be parsed once, at the very top of the file.
    let mut validations = validations.clone();
    validations.skip_metadata_summary = true;

    // Try to split it on the large chunks.
    const START: &'static str = "!!summary";
    if let Some(start_idx) = contents.find(START) {
        let (before, start) = contents.split_at(start_idx);
        let start = start.replacen(START, "", 1);

        if let Some(end_idx) = start.find("\n") {
            let (content, after) = start.split_at(end_idx);
            let after = after.replacen("\n", "", 1);

            let mut ir = vec![];

            if before.len() > 0 {
                ir.push(ContentIr::Unparsed {
                    contents: before.into(),
                    validations: validations.clone(),
                });
            }

            ir.push(ContentIr::Metadata(Metadata::Summary(
                content.trim().into(),
            )));

            if after.len() > 0 {
                ir.push(ContentIr::Unparsed {
                    contents: after.into(),
                    validations: validations.clone(),
                });
            }

            return Some(ir);
        }
    }

    None
}
