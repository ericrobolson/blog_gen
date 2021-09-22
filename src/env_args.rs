use std::{env, fs, sync::Arc};

use super::{console, Context, Res};
use crate::{config::Config, Directory, Item, Links};

pub fn parse() -> Option<Res<Arc<Context>>> {
    let env_args = {
        let mut env_args: Vec<String> = env::args().collect();
        env_args.remove(0);
        env_args
    };

    if env_args.is_empty() {
        return Some(Res::Error {
            location: None,
            msg: "No valid args! Type 'help' to list all options".into(),
        });
    }

    let mut warnings = vec![];

    match Command::parse(env_args) {
        Ok(command) => match command {
            Command::Help => {
                Command::list_help();
                return None;
            }
            Command::Generate { input, output } => {
                let config = match parse_config_file(input.clone()) {
                    Res::Ok(c) => c,
                    Res::Warn { item, msg } => {
                        warnings.push((msg, None));
                        item
                    }
                    Res::Warnings { item, mut msgs } => {
                        warnings.append(&mut msgs);
                        item
                    }
                    Res::Error { location, msg } => return Some(Res::Error { location, msg }),
                };

                // TODO: wire up warnings
                return Some(Res::Ok(Item {
                    item: Arc::new(Context {
                        config: config.item,
                        input,
                        output,
                        links: Links {
                            about_me_link: "about_me.html".into(),
                            catalog_link: "catalog.html".into(),
                            index_link: "index.html".into(),
                            projects_link: "projects.html".into(),
                        },
                    }),
                    location: None,
                }));
            }
        },
        Err(e) => Some(Res::Error {
            location: None,
            msg: e,
        }),
    }
}

fn parse_config_file(input_dir: Directory) -> Res<Config> {
    let dir = input_dir.to_string();
    let dir = if dir.ends_with("/") {
        dir
    } else {
        format!("{}/", dir)
    };

    let config_path = format!("{}config.json", dir);

    let file = match fs::read_to_string(config_path) {
        Ok(file) => file,
        Err(e) => {
            return Res::Error {
                location: None,
                msg: format!("Error loading config.json: \n\t\t{:?}.\n\tEnsure you have a config.json file at {:?}.", e, input_dir.to_string()),
            }
        }
    };

    let config: Config = match serde_json::from_str::<Config>(&file) {
        Ok(c) => c,
        Err(e) => return Res::Error {
            location: None,
            msg: format!(
                "Error parsing config.json: \n\t\t{:?}. \n\tEnsure that you've got a well formed JSON file.",
                e
            ),
        },
    };

    Res::Ok(Item {
        item: config,
        location: None,
    })
}

enum Command {
    Help,
    Generate { input: Directory, output: Directory },
}

impl Command {
    pub fn list_help() {
        console::info("{input_directory} {output_directory}", "Generates a static site based on an input directory, writing it to an output directory.");
        console::info("help", "List all available commands.");
        console::info(
            "config.json",
            &format!(
                "Example config.json file: \n\t{}",
                serde_json::to_string(&Config::example_config()).unwrap()
            ),
        );

        console::info(
            "HTML",
            "Use triple quotes like ```some code``` for code blocks.",
        );
        console::info(
            "HTML",
            "Use triple quotes with 'quote' like ```quote some quote``` for quotes.",
        );
        console::info(
            "HTML",
            "Use the following format for links: '[Description](url)'.",
        );
        console::info(
            "HTML",
            "Use the following format for images: '![Description](url)'.",
        );
        console::info(
            "HTML",
            "Use the following format for youtube: '![youtube](youtube_embedded_url)'.",
        );
        console::info(
            "HTML",
            "Use the following format for titles: '!!title Some Title terminated by newline'.",
        );
        console::info(
            "HTML",
            "Use the following format for keywords: '!!keywords a list of keywords terminated by newline'.",
        );
        console::info(
            "HTML",
            "Use the following format for the summary: '!!summary a list of keywords terminated by newline'.",
        );
        console::info(
            "HTML",
            "Use the following format for headers: '## a header!'.",
        );
    }

    pub fn parse(mut args: Vec<String>) -> Result<Command, String> {
        let first = args.remove(0);

        match first.to_lowercase().as_str() {
            "help" => Ok(Command::Help),
            _ => {
                let input_dir = first;
                if args.is_empty() {
                    return Err("Unable to parse output directory!".into());
                }

                let output_dir = args.remove(0);

                if !args.is_empty() {
                    console::warn(&format!("Received unused CLI args: {:?}", args), None);
                }

                console::success(
                    &format!(
                        "Parsed input '{}' and output '{}' directories.",
                        input_dir, output_dir
                    ),
                    None,
                );

                Ok(Command::Generate {
                    input: input_dir.into(),
                    output: output_dir.into(),
                })
            }
        }
    }
}
