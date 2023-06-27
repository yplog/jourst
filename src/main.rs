mod operations;

use operations::folder::read_directory;
use std::path::Path;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

const BASE_PATH: &str = "~/.config/jourst/";

fn main() {
    println!("Hello, world!");
    let path = Path::new(".");

    match read_directory(".") {
        Ok(files) => {
            for file in files {
                println!("Name: {}", file);
            }
        }
        Err(err) => {
            println!("Error: {}", err.to_string());
            std::process::exit(1);
        }
    }

    let args = Cli::parse();
}
