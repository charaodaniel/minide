use std::fs;

pub fn open_file(path: &str) -> Result<String, std::io::Error> {

    let content = fs::read_to_string(path)?;

    Ok(content)
}