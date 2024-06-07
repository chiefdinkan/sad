use std::path::PathBuf;
use tokio::task;

mod help;
mod read;

#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = help::help_args();
    let color_code = args.color_code;
    let files: Vec<PathBuf> = args.files;
    let line_number = args.line_number;

    let tasks: Vec<_> = files
        .into_iter()
        .map(|file| {
            let color_code = color_code.clone();
            task::spawn(async move {
                if let Err(e) = read::read_file(file.clone(), color_code.clone(), line_number).await
                {
                    eprintln!("Error reading {:?}: {}", file, e);
                }
            })
        })
        .collect();

    for handle in tasks {
        if let Err(e) = handle.await {
            eprintln!("Task failed: {:?}", e);
        }
    }

    Ok(())
}
