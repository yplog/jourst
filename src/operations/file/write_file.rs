use std::{fs::File, path::Path};

pub fn write_file(path: &Path) -> Result<File, std::io::Error> {
    let display = path.display();
    let file = File::create(&path)?;
    Ok(file)
}