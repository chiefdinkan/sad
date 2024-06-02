use std::env;
use std::path::PathBuf;
use tokio::task;

mod read;
mod help;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help::print_help();
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
        help::print_help();
        std::process::exit(1);
    }

    let tasks = files.iter().map(|file| {
        let file = file.clone();
        let color_code = color_code.clone();
        task::spawn(async move {
            if let Err(e) = read::read_file(file.clone(), color_code).await {
                eprintln!("Error reading {:?}: {}", file, e);
            }
        })
    });
    let handles: Vec<_> = tasks.collect();

    for handle in handles {
        if let Err(e) = handle.await {
            eprintln!("Task failed: {:?}", e);
        }
    }
}

