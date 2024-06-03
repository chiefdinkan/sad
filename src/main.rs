use tokio::task;
use std::path::PathBuf;

mod help;
mod read;

#[tokio::main]

async fn main() {
    let args = help::help_args();
    let color_code = args.color_code;
    let files: Vec<PathBuf> = args.files;

    let tasks = files.iter().map(|file| {
        let file = file.clone();
        let color_code = color_code.clone();
        task::spawn(async move {
            if let Err(e) = read::read_file(file.clone(), color_code.clone()).await {
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
