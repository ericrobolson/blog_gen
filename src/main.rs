pub mod builder;
pub mod config;
pub mod console;
pub mod crawler;
pub mod directory;
pub mod env_args;
pub mod generator;
pub mod html;
pub mod parser;
pub mod theme;

use std::sync::Arc;

use config::Config;
use directory::Directory;
use parser::IntermediateRepresentation;

fn main() {
    let start = std::time::Instant::now();

    // Source the context for this session
    let context = match env_args::parse() {
        Some(res) => match res {
            Res::Ok(item) => item.item,
            Res::Warn { item, msg } => {
                console::warn(&msg, item.location);
                item.item
            }
            Res::Error { location, msg } => {
                console::error(&msg, location);
                return;
            }
            Res::Warnings { item, msgs } => {
                for (warning, location) in msgs.iter() {
                    console::warn(warning, location.clone());
                }

                item.item
            }
        },
        // Nothing available, so simply return
        None => return,
    };

    // Traverse the input directory
    let crawled_files = match map_results(crawler::execute(context.input.clone())) {
        Ok(results) => results,
        Err(_) => return,
    };

    console::success(
        &format!("Crawled all files at '{}'.", context.input.to_string()),
        None,
    );

    // Parse all intermediate representations
    let ir = match map_results(crawl_files(crawled_files, &context)) {
        Ok(ir) => ir,
        Err(_) => return,
    };

    // Build the IR + analyze it
    let analyzed_ir = match map_results(builder::execute(context.clone(), ir)) {
        Ok(ir) => ir,
        Err(_) => return,
    };

    // Finally generate all code
    match map_results(generator::execute(analyzed_ir, context.clone())) {
        Ok(_) => {
            let msg = format!("Success! Took {:?}.", std::time::Instant::now() - start);

            console::success(&msg, None)
        }
        Err(_) => {
            return;
        }
    }
}

#[allow(unreachable_code)]
fn crawl_files(
    files: Vec<Item<crawler::File>>,
    context: &Arc<Context>,
) -> Vec<Res<IntermediateRepresentation>> {
    #[cfg(feature = "multithread")]
    {
        use rayon::prelude::*;
        return files
            .par_iter()
            .map(|f| parser::execute(f, context.clone()))
            .collect();
    }

    files
        .iter()
        .map(|f| parser::execute(f, context.clone()))
        .collect()
}

#[derive(Clone, Debug, PartialEq)]
pub struct Context {
    pub config: Config,
    pub input: Directory,
    pub links: Links,
    pub output: Directory,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Links {
    about_me_link: String,
    catalog_link: String,
    index_link: String,
    projects_link: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Item<T> {
    pub item: T,
    pub location: Option<Location>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Location {
    pub column: u32,
    pub file: String,
    pub line: u32,
    pub path: String,
}

pub enum Res<T> {
    Ok(Item<T>),
    Warn {
        item: Item<T>,
        msg: String,
    },
    Warnings {
        item: Item<T>,
        msgs: Vec<(String, Option<Location>)>,
    },
    Error {
        location: Option<Location>,
        msg: String,
    },
}

/// Maps all results from a given collection.
pub fn map_results<T>(items: Vec<Res<T>>) -> Result<Vec<Item<T>>, ()> {
    let mut results = vec![];
    let mut warnings = vec![];
    let mut errors = vec![];

    for file in items {
        match file {
            Res::Ok(item) => results.push(item),
            Res::Warn { item, msg } => {
                warnings.push((msg, item.location.clone()));
                results.push(item);
            }
            Res::Warnings { item, mut msgs } => {
                warnings.append(&mut msgs);
                results.push(item);
            }
            Res::Error { location, msg } => {
                errors.push((msg, location));
            }
        }
    }

    // Print all warnings
    for (msg, location) in warnings.iter() {
        console::warn(&msg, location.clone());
    }

    // Print all errors
    let exit = !errors.is_empty();
    for (msg, location) in errors.iter() {
        console::error(&msg, location.clone());
    }

    if warnings.is_empty() == false {
        console::info("Warning Count", &format!("{} warnings", warnings.len()));
    }

    if errors.is_empty() == false {
        console::info("Error Count", &format!("{} errors", errors.len()));
    }

    if exit {
        return Err(());
    }

    Ok(results)
}
