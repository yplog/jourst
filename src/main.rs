use args::Cli;
use clap::Parser;
use dotenv::dotenv;
use operations::folder::{is_directory_exists, read_directory, create_directory};
use std::env;

mod operations;
mod models;
mod args;

const DEFAULT_APPLICATION_PATH: &str = ".jourst";

fn main() {
    let base_path = application_path();

    let is_base_path_exists = is_directory_exists(&base_path);

    if !is_base_path_exists {
        println!("Base path does not exists.");
        println!("Creating base path: {}", &base_path);

        match create_directory(&base_path) {
            Ok(path) => println!("Created base path: {}", path),
            Err(err) => {
                println!("Error: {}", err.to_string());
                std::process::exit(1);
            }
        }
    } else {
        match read_directory(&base_path) {
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
    }

    let matches = Cli::parse();
    println!("{:?}", matches);
}

fn application_path() -> String {
    dotenv().ok();

    let dotenv_path =
        std::env::var("APPLICATION_PATH").unwrap_or(DEFAULT_APPLICATION_PATH.to_string());

    let home_dir = match env::var("HOME") {
        Ok(val) => val,
        Err(e) => {
            println!("Error: {}", e.to_string());
            println!("Please set the HOME environment variable.");

            std::process::exit(1);
        }
    };

    let base_path = format!("{}/{}", home_dir, dotenv_path);

    base_path
}
