use crate::parser::md::{parse_contents::Validations, ContentIr};

pub fn header(contents: &String, validations: &Validations) -> Option<Vec<ContentIr>> {
    let chars: Vec<char> = contents.chars().collect();

    if chars.len() == 0 {
        return None;
    }

    let mut start_idx = None;
    let mut header_size = 0;
    let mut end_idx = chars.len() - 1;
    let mut stop = false;

    for i in 0..chars.len() {
        if stop == true {
            break;
        }

        let c = chars[i];

        let can_start_header = {
            if i == 0 {
                true
            } else {
                chars[i - 1] == '\n'
            }
        };

        if can_start_header && c == '#' {
            header_size += 1;
            stop = true;
            start_idx = Some(i);
            let mut continuous = true;

            for j in i..chars.len() {
                if chars[j] == '#' && continuous {
                    header_size += 1;
                } else {
                    continuous = false;
                }

                end_idx = j;

                if chars[j] == '\n' {
                    break;
                }
            }
        }
    }

    if let Some(start_idx) = start_idx {
        let (before, after) = contents.split_at(end_idx);
        let (before, header) = before.split_at(start_idx);
        let mut before_validations = validations.clone();
        before_validations.skip_header = true;

        let mut ir = vec![];
        if before.len() > 0 {
            ir.push(ContentIr::Unparsed {
                contents: before.into(),
                validations: before_validations,
            });
        }

        let header: String = header.replace("#", "").into();
        if header.len() > 0 {
            ir.push(ContentIr::Header {
                id: header.replace(" ", ""),
                header_size,
                text: header,
            });
        }

        if after.len() > 0 {
            ir.push(ContentIr::Unparsed {
                contents: after.into(),
                validations: validations.clone(),
            })
        }

        return Some(ir);
    }

    None
}
