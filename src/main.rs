use colored::*;
use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokio::task;

#[tokio::main]
async fn main() {
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
                if is_valid_hex_color(code) {
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

    let tasks = files.iter().map(|file| {
        let file = file.clone();
        let color_code = color_code.clone();
        task::spawn(async move {
            if let Err(e) = read_file(file.clone(), color_code).await {
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

fn print_help() {
    println!("Usage: sad <file> [<file2> ...]");
    println!("Usage: -c or --color <hexcode>");
    println!("       sad -c <color_hex_code> <file> [<file2> ...]");
    println!("Example: sad -c ff0000 file.txt");
}

fn is_valid_hex_color(code: &str) -> bool {
    code.len() == 6 && code.chars().all(|c| c.is_ascii_hexdigit())
}

async fn read_file(file: PathBuf, color_code: Option<String>) -> io::Result<()> {
    let file_path = file.display().to_string();

    let file = File::open(&file)?;
    let reader = BufReader::new(file);
    let reader = Arc::new(Mutex::new(reader));

    let mut buffer = [0; 8192];
    let mut total_bytes_read = 0;
    let mut content = Vec::new();

    loop {
        let reader = Arc::clone(&reader);
        let result = task::spawn_blocking(move || {
            let mut reader = reader.lock().unwrap();
            let bytes_read = reader.read(&mut buffer)?;
            Ok::<_, io::Error>((bytes_read, buffer))
        })
        .await;

        match result {
            Ok(Ok((bytes_read, buffer))) => {
                if bytes_read == 0 {
                    break;
                }

                total_bytes_read += bytes_read;
                content.extend_from_slice(&buffer[..bytes_read]);
            }
            Ok(Err(e)) => return Err(e),
            Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
        }
    }

    let output = String::from_utf8_lossy(&content).to_string();
    if let Some(color_code) = color_code {
        let colored_output = output.truecolor(
            u8::from_str_radix(&color_code[0..2], 16).unwrap(),
            u8::from_str_radix(&color_code[2..4], 16).unwrap(),
            u8::from_str_radix(&color_code[4..6], 16).unwrap(),
        );
        println!("{}", colored_output);
    } else {
        println!("{}", output);
    }
    println!("Finished reading {}: {} bytes", file_path, total_bytes_read);
    Ok(())
}
