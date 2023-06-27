use std::io::{Error, ErrorKind, Result as IoResult};
use std::fs;

pub fn read_directory(path: &str) -> IoResult<Vec<String>> {
    let metadata = fs::metadata(path)?;
    if !metadata.is_dir() {
        return Err(Error::new(ErrorKind::NotFound, "Directory not found"));
    }

    let files = fs::read_dir(path)?
        .filter_map(|entry| {
            entry
                .ok()
                .and_then(|e| e.file_name().into_string().ok())
        })
        .collect();

    Ok(files)
}