use crate::Location;
use ansi_term::Colour;

pub fn error(msg: &str, location: Option<Location>) {
    let location = pretty_print_loc(location);
    let colour = Colour::Red;

    println!("{}{}: {}", colour.paint("Error"), location, msg);
}

pub fn info(info: &str, msg: &str) {
    let colour = Colour::Cyan;

    println!("{}: {}", colour.paint(info), msg);
}

pub fn success(msg: &str, location: Option<Location>) {
    let location = pretty_print_loc(location);
    let colour = Colour::Green;

    println!("{}{}: {}", colour.paint("Success"), location, msg);
}

pub fn warn(msg: &str, location: Option<Location>) {
    let location = pretty_print_loc(location);
    let colour = Colour::Yellow;

    println!("{}{}: {}", colour.paint("Warning"), location, msg);
}

fn pretty_print_loc(location: Option<Location>) -> String {
    match location {
        Some(loc) => format!(
            " at path '{}', ln {}, col {}",
            loc.path, loc.line, loc.column
        ),
        None => String::new(),
    }
}
