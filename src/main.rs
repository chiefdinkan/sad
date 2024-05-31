use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokio::task;
use colored::*;

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
        if args[i] == "-c" {
            if i + 1 < args.len() {
                color_code = Some(args[i + 1].clone());
                i += 1;
            } else {
                println!("Error: Missing color code after -c");
                std::process::exit(1);
            }
        } else {
            files.push(PathBuf::from(&args[i]));
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
                println!("Error reading {:?}: {}", file, e);
            }
        })
    });
    let handles: Vec<_> = tasks.collect();

    let _ = tokio::join!(async {
        for handle in handles {
            handle.await.unwrap();
        }
    });
}

fn print_help() {
    println!("Usage: sad <file> [<file2> ...]");
    println!("       sad -c <color_hex_code> <file> [<file2> ...]");
    println!("Example: sad -c ff0000 file.txt");
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
        let bytes_read = task::spawn_blocking(move || {
            let mut reader = reader.lock().unwrap();
            let bytes_read = reader.read(&mut buffer)?;
            Ok::<_, io::Error>((bytes_read, buffer))
        })
        .await
        .expect("task failed")?;

        let (bytes_read, buffer) = bytes_read;

        if bytes_read == 0 {
            break;
        }

        total_bytes_read += bytes_read;

        content.extend_from_slice(&buffer[..bytes_read]);
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
