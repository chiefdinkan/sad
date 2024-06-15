// Code needs major refac
use colored::*;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokio::task;

pub fn is_valid_hex_color(code: &str) -> bool {
    code.len() == 6 && code.chars().all(|c| c.is_ascii_hexdigit()) 
}

pub async fn read_file(
    file: PathBuf,
    color_code: Option<String>,
    show_line_numbers: bool,
) -> io::Result<()> {
    let file_path = file.display().to_string();

    let file = File::open(&file)?; 
    let reader = BufReader::new(file);
    let reader = Arc::new(Mutex::new(reader));
    let mut content = Vec::new();

    loop {
        let reader = Arc::clone(&reader);
        let result = task::spawn_blocking(move || {
            let mut reader = reader.lock().unwrap();
            let mut buffer = [0; 8192];
            let bytes_read = reader.read(&mut buffer)?;
            Ok::<_, io::Error>((bytes_read, buffer))
        })
        .await;

        match result {
            Ok(Ok((bytes_read, buffer))) => {
                if bytes_read == 0 {
                    break;
                }
                content.extend_from_slice(&buffer[..bytes_read]);
            }
            Ok(Err(e)) => return Err(e),
            Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
        }
    }

    let output = String::from_utf8_lossy(&content).to_string(); // does UTF-8 for better compatability ?
    let lines: Vec<_> = output.lines().collect();

    if let Some(color_code) = color_code {
        // I am going to regret this in future {pls use match in future}
        if show_line_numbers {
            for (i, line) in lines.iter().enumerate() {
                let colored_line_number = format!("{:>4}: ", i + 1).truecolor(
                    u8::from_str_radix(&color_code[0..2], 16).unwrap(),
                    u8::from_str_radix(&color_code[2..4], 16).unwrap(),
                    u8::from_str_radix(&color_code[4..6], 16).unwrap(),
                );
                let colored_line = line.truecolor(
                    u8::from_str_radix(&color_code[0..2], 16).unwrap(),
                    u8::from_str_radix(&color_code[2..4], 16).unwrap(),
                    u8::from_str_radix(&color_code[4..6], 16).unwrap(),
                );
                println!("{}{}", colored_line_number, colored_line);
            }
        } else {
            let colored_output = output.truecolor(
                u8::from_str_radix(&color_code[0..2], 16).unwrap(),
                u8::from_str_radix(&color_code[2..4], 16).unwrap(),
                u8::from_str_radix(&color_code[4..6], 16).unwrap(),
            );
            println!("{}", colored_output);
        }
    } else {
        for (i, line) in lines.iter().enumerate() {
            if show_line_numbers {
                println!("{:>4}: {}", i + 1, line); 
            } else {
                println!("{}", line);
            }
        }
    }

    println!("finished reading {}: {} bytes", file_path, content.len()); // will be a flag soon??
    Ok(())
}
