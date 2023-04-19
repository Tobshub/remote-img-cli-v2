use std::path::PathBuf;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let command: Option<&str> = args.get(1).map(|x| &**x);
    if command.is_some() {
        let command = command.unwrap();
        match command {
            "--upload" | "-u" => {
                for path in &args[2..] {
                    let abs_path = fs::canonicalize(PathBuf::from(path));
                    match abs_path {
                        Ok(path) => upload_file_at_path(&path),
                        Err(_) => println!("Could not find file: {}", path),
                    }
                }
            }
            "--help" | "-h" | _ => display_help_message(),
        }
    } else {
        display_help_message();
    }
}

fn upload_file_at_path(path: &PathBuf) {
    println!("Uploading file at {:?}", path)
}

fn display_help_message() {
    println!(
        "\
Tobsmg CLI - v2.0.0

Usage:

    --server <server-url>       Configure the remote server url
    -u, --upload                Upload files to the server
    -t, --temp-upload          Temporarily Upload files to the server 
    "
    );
}
