# Asynchronous File Reader in Rust with Tokio

This Rust program reads multiple files concurrently using Tokio for asynchronous tasks. 

1. **Clone the repository:**

   ```bash
   git clone <repository-url>
   cd <repository-name>
   ```

2. **Build the project:**

   ```bash
   cargo build
   ```

3. **Run the program:**

   ```bash
   cargo run <file1> [<file2> ...]
   ```

   Replace `<file1>`, `<file2>`, etc., with the paths to the files you want to read.

## Or grab one from the releases(only x64 binary for linux right now)

## Example

```bash
cargo run ./file1.txt ./file2.txt
```
## TODO

- [] pipe functionality.
- [] colorful output.
- [] more testing with various environments.

## Explanation

  - Parses command-line arguments to get a list of files.
  - Spawns asynchronous tasks to read each file concurrently using `task::spawn`.
  - Uses `tokio::join!` to wait for all tasks to complete.
  - Opens each file and creates a `BufReader`.
  - Wraps the reader in an `Arc<Mutex<BufReader<File>>>` to safely share across threads.
  - Uses `task::spawn_blocking` to perform blocking I/O operations (like file read) without blocking the Tokio runtime.
  - Reads file content in chunks, accumulates content in a `Vec<u8>`, and prints the content and total bytes read.

## Output

For each file, the program will output the content and the number of bytes read, like this:

```
Content of <file>: 
<file-content>

Finished reading <file>: <bytes> bytes
```

