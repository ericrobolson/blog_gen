use crate::{
    parser::md::{
        parse_contents::{self, content_ir::BlockKind, Options, Validations},
        ContentIr,
    },
    Location, Res,
};

pub fn block(
    contents: &String,
    opts: Options,
    validations: &Validations,
) -> Option<Vec<ContentIr>> {
    // Try to split it on the large chunks.

    let (start_idx, block_kind, start_token) = if let Some(idx) = contents.find("```quote") {
        (Some(idx), BlockKind::Quote, "```quote")
    } else if let Some(idx) = contents.find("```") {
        (Some(idx), BlockKind::Code, "```")
    } else {
        (None, BlockKind::Code, "")
    };

    if let Some(start_idx) = start_idx {
        let (before, code_start) = contents.split_at(start_idx);
        let code_start = code_start.replacen(start_token, "", 1);

        if let Some(end_idx) = code_start.find("```") {
            let (code, after) = code_start.split_at(end_idx);
            let after = after.replacen("```", "", 1);

            let mut before_validations = validations.clone();
            before_validations.skip_code_block = true;

            let mut ir = vec![];

            if before.len() > 0 {
                ir.push(ContentIr::Unparsed {
                    contents: before.into(),
                    validations: before_validations,
                });
            }

            if code.len() > 0 {
                let contents = match block_kind {
                    BlockKind::Code => ContentIr::Text { text: code.into() },
                    BlockKind::Quote => {
                        let quote_validations = {
                            let mut validations = validations.clone();
                            validations.validate_paragraph_len = false;
                            validations
                        };

                        match parse_contents::execute(
                            code.into(),
                            Location {
                                column: 0,
                                file: String::default(),
                                line: 0,
                                path: String::default(),
                            },
                            opts,
                            Some(quote_validations.clone()),
                        ) {
                            Res::Ok(contents) => contents.item,
                            Res::Warn { item, msg } => {
                                //TODO: How to handle warning in blocks?
                                println!("Unhandled warning for blocks: {:?}", msg);
                                item.item
                            }
                            Res::Warnings { msgs, .. } => {
                                //TODO: How to handle warnings in blocks?
                                println!(
                                    "Unhandled warnings for blocks: {:?}",
                                    msgs.iter().map(|s| format!("\n{:?}", s))
                                );
                                ContentIr::Text { text: code.into() }
                            }
                            Res::Error { msg, .. } => {
                                //TODO: How to handle errors in blocks?
                                println!("Unhandled errors for blocks: {:?}", msg);
                                ContentIr::Text { text: code.into() }
                            }
                        }
                    }
                };

                ir.push(ContentIr::Block {
                    contents: Box::new(contents),
                    block_kind,
                });
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
