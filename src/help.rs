use clap::{Arg, Command};
use std::path::PathBuf;

use crate::read;

pub struct ArgsCf {
    pub color_code: Option<String>,
    pub files: Vec<PathBuf>,
}

pub fn help_args() -> ArgsCf {
    let matches = Command::new("sad")
        .about("A command line tool for outputing text files")
        .arg(
            Arg::new("files")
                .help("Input file(s)")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("color")
                .short('c')
                .long("color")
                .help("Hex color code")
                .required(false)
                .num_args(1),
        )
        .get_matches();

    let color_code = matches.get_one::<String>("color").cloned();

    if let Some(ref code) = color_code {
        if !read::is_valid_hex_color(code) {
            eprintln!("Error: Invalid color code '{}'", code);
            std::process::exit(1);
        }
    }

    let files = matches
        .get_many::<String>("files")
        .expect("required argument")
        .map(PathBuf::from)
        .collect();

    ArgsCf { color_code, files }
}
