use std::env;
use std::path::PathBuf;

use crate::read;

fn print_help() {
    println!("Usage: sad <file> [<file2> ...]");
    println!("Usage: -c or --color <hexcode>");
    println!("       sad -c <color_hex_code> <file> [<file2> ...]");
    println!("       Example: sad -c ff0000 file.txt");
}

pub struct Args {
    pub color_code: Option<String>,
    pub files: Vec<PathBuf>,
}

pub fn help_args() -> Args {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        std::process::exit(1);
    }

    let mut color_code = None;
    let mut files = Vec::new();
    let mut i = 1;

    while i < args.len() {
        if args[i] == "-c" || args[i] == "--color" {
            if i + 1 < args.len() {
                let code = &args[i + 1];
                if read::is_valid_hex_color(code) {
                    color_code = Some(code.clone());
                    i += 1;
                } else {
                    eprintln!("Error: Invalid color code '{}'", code);
                    std::process::exit(1);
                }
            } else {
                eprintln!("Error: Missing color code after -c or --color");
                std::process::exit(1);
            }
        } else if !args[i].starts_with('-') {
            files.push(PathBuf::from(&args[i]));
        } else {
            eprintln!("Invalid flag {}.", args[i]);
            std::process::exit(1);
        }
        i += 1;
    }

    if files.is_empty() {
        print_help();
        std::process::exit(1);
    }

    Args {
        color_code,
        files,
    }
}
