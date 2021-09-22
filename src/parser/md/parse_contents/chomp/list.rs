use crate::{
    parser::md::{
        parse_contents::{self, Options, Validations},
        ContentIr,
    },
    Location, Res,
};

pub fn list(
    contents: &String,
    mut opts: Options,
    validations: &Validations,
) -> Option<Vec<ContentIr>> {
    let opts = {
        opts.make_cards = false;
        opts.make_paragraphs = false;
        opts
    };

    let chars: Vec<char> = contents.chars().collect();

    if chars.len() == 0 {
        return None;
    }

    let mut start_idx = None;
    let mut end_idx = chars.len() - 1;
    let mut stop = false;

    let mut items = vec![];

    let mut i = 0;

    while i < chars.len() {
        if stop == true {
            break;
        }

        let c = chars[i];

        let can_start = {
            if i == 0 {
                true
            } else {
                chars[i - 1] == '\n'
            }
        };

        let make_list = match c {
            '*' => true,
            _ => {
                if i < chars.len() - 1 {
                    let peeked1 = chars[i + 1];

                    peeked1 == ')'
                } else {
                    false
                }
            }
        };

        if can_start && make_list {
            for j in i..chars.len() {
                if chars[j] == '\n' {
                    if let None = start_idx {
                        start_idx = Some(i);
                    }

                    // Add item, mutate rest
                    items.push(contents[i..j].to_string());

                    break;
                }
            }
        }
        // Stop making the list if it's a new line or not the '*'
        else if can_start && start_idx.is_some() {
            stop = true;
            end_idx = i;
        }

        i += 1;
    }

    if items.is_empty() {
        return None;
    }

    if let Some(start_idx) = start_idx {
        let (before, after) = contents.split_at(end_idx);
        let (before, _) = before.split_at(start_idx);
        let mut before_validations = validations.clone();
        before_validations.skip_list = true;

        // Check if it's a
        let list = {
            // Trim off the bullet point
            let items: Vec<String> = items.iter().map(|i| i.replacen("* ", "", 1)).collect();

            // Check if it's ordered
            let lowercased_first = items[0].to_lowercase();
            if lowercased_first.starts_with("1)") || lowercased_first.starts_with("2)") {
                let items = items.iter().map(|i| {
                    match i.find(')') {
                        Some(s) => {
                            map_contents(i[(s + 1)..].to_string(), opts, validations)
                        },
                        None => todo!("You have an ordered, alphabetical list that is not well formed! Likely missing a paren."),
                    }
                }).flatten().collect();

                // It's numerical
                ContentIr::OrderedList {
                    items,
                    numeric: true,
                }
            } else if lowercased_first.starts_with("a)") {
                let items = items.iter().map(|i| {
                    match i.find(')') {
                        Some(s) => map_contents(i[(s + 1)..].to_string(), opts, validations),
                        None => todo!("You have an ordered, alphabetical list that is not well formed! Likely missing a paren."),
                    }
                }).flatten().collect();

                // It's alphabetical
                ContentIr::OrderedList {
                    items,
                    numeric: false,
                }
            } else {
                ContentIr::UnorderedList {
                    items: items
                        .iter()
                        .map(|i| map_contents(i.clone(), opts, validations))
                        .flatten()
                        .collect(),
                }
            }
        };

        return Some(vec![
            ContentIr::Unparsed {
                contents: before.into(),
                validations: before_validations,
            },
            list,
            ContentIr::Unparsed {
                contents: after.into(),
                validations: validations.clone(),
            },
        ]);
    }

    None
}

fn map_contents(s: String, opts: Options, validations: &Validations) -> Vec<ContentIr> {
    match parse_contents::execute(
        s,
        Location {
            column: 0,
            file: String::default(),
            line: 0,
            path: String::default(),
        },
        opts,
        Some(validations.clone()),
    ) {
        Res::Ok(contents) => vec![contents.item],
        // TODO: how to handle warning here?
        Res::Warn { item, .. } => vec![item.item],
        Res::Warnings { .. } => todo!("How to handle warnings in lists?"),
        Res::Error { .. } => todo!("How to handle errors in lists?"),
    }
}
