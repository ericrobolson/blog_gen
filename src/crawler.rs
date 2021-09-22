use crate::{Directory, Item, Location, Res};
use walkdir::WalkDir;

pub struct File {
    pub kind: FileKind,
    pub path: String,
    pub file_name: String,
}

pub enum FileKind {
    Css,
    Img,
    Md,
}

pub fn execute(dir: Directory) -> Vec<Res<File>> {
    let directory = dir.to_string();

    let mut files = vec![];

    for entry in WalkDir::new(&directory)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let path = entry.path();

        let file_name = path.file_name().unwrap().to_str().unwrap_or("").to_string();
        let path_string = String::from(path.to_str().unwrap_or("".into()));

        if file_name == "config.json" {
            continue;
        }

        if let Some(extension) = path.extension() {
            if let Some(extension) = extension.to_str() {
                match extension.to_lowercase().as_str() {
                    "gif" | "png" => {
                        files.push(Res::Ok(Item {
                            item: File {
                                kind: FileKind::Img,
                                path: path_string,
                                file_name,
                            },
                            location: None,
                        }));
                    }
                    "md" => {
                        files.push(Res::Ok(Item {
                            item: File {
                                kind: FileKind::Md,
                                path: path_string,
                                file_name,
                            },
                            location: None,
                        }));
                    }
                    "css" => {
                        files.push(Res::Ok(Item {
                            item: File {
                                kind: FileKind::Css,
                                path: path_string,
                                file_name,
                            },
                            location: None,
                        }));
                    }
                    _ => files.push(Res::Error {
                        location: Some(Location {
                            column: 0,
                            file: file_name,
                            line: 0,
                            path: path_string,
                        }),
                        msg: format!("Unhandled file type '{}'!", extension),
                    }),
                }
            }
        }
    }

    files
}
