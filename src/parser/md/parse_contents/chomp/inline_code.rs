use crate::parser::md::{parse_contents::Validations, ContentIr};

pub fn inline_code(contents: &String, validations: &Validations) -> Option<Vec<ContentIr>> {
    // Try to split it on the large chunks.
    if let Some(start_idx) = contents.find("`") {
        let (before, code_start) = contents.split_at(start_idx);
        let code_start = code_start.replacen("`", "", 1);

        if let Some(end_idx) = code_start.find("`") {
            let (code, after) = code_start.split_at(end_idx);
            let after = after.replacen("`", "", 1);

            let mut before_validations = validations.clone();
            before_validations.skip_inline_code = true;

            let mut ir = vec![];

            if before.len() > 0 {
                ir.push(ContentIr::Unparsed {
                    contents: before.into(),
                    validations: before_validations,
                });
            }

            if code.len() > 0 {
                ir.push(ContentIr::InlineCode(code.into()));
            }

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
