use std::{path::Path, fs::File};

pub fn open_file(path: &Path) -> Result<File, std::io::Error> {
    let file = File::open(&path)?;
    Ok(file)
}
