use clap::Parser;
use rayon::prelude::*;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The source directory
    #[arg(short, long)]
    src_dir: PathBuf,
    /// The destination directory
    #[arg(short, long)]
    dest_dir: PathBuf,
}

fn main() {
    let args = Args::parse();

    let content = crawl_files(&args.src_dir);
    let count = content.html_files.len();

    println!("- Found {:?} HTML files", count);

    let completed = Arc::new(Mutex::new(0));

    content.html_files.par_iter().for_each(|path| {
        let mut completed = completed.lock().unwrap();
        *completed = *completed + 1;
        println!("{completed} / {count}")
    });

    println!("FIN")
}

#[derive(Clone, Debug)]
struct Content {
    html_files: Vec<PathBuf>,
    css_files: Vec<PathBuf>,
}

const TEMPLATE_PATH: &'static str = "template.html";

fn crawl_files(path: &PathBuf) -> Content {
    let files: Vec<PathBuf> = walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|f| f.ok())
        .map(|f| f.into_path())
        .filter(|p| p.is_file())
        .collect();

    let mut html_files: Vec<PathBuf> = vec![];
    let mut css_files: Vec<PathBuf> = vec![];

    // Crawl and source all files
    for file in files {
        if let Some(extension) = file.extension() {
            match extension.to_str().unwrap() {
                "html" => html_files.push(file),
                "css" => css_files.push(file),
                _ => {}
            }
        }
    }

    Content {
        html_files,
        css_files,
    }
}
