use std::fs;

pub fn create_directory(path: &str) -> Result<&str, std::io::Error> {
    match fs::create_dir(path) {
        Ok(()) => {
            println!("The directory was created successfully.");
            Ok(&path)
        }
        Err(err) => {
            println!("Error creating directory: {:?}", err);
            Err(err)
        }
    }
}
