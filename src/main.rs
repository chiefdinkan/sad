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
        println!("Usage: sad <file> [<file2> ...]");
        std::process::exit(1);
    }

    let files: Vec<PathBuf> = args[1..].iter().map(PathBuf::from).collect();

    let tasks = files.iter().map(|file| {
        let file = file.clone();
        task::spawn(async move {
            if let Err(e) = read_file(file.clone()).await {
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

async fn read_file(file: PathBuf) -> io::Result<()> {
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

    println!(
        "Content of {}: \n{}",
        file_path,
        String::from_utf8_lossy(&content)
    );
    println!("Finished reading {}: {} bytes", file_path, total_bytes_read);
    Ok(())
}
