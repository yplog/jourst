use std::{fs::File, path::Path};

pub fn open_file(path: &Path) -> Result<File, std::io::Error> {
    let file = File::open(&path)?;
    Ok(file)
}
