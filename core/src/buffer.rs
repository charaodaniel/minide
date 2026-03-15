use std::{fs, path::Path};

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn open(path: &str) -> Result<Self, std::io::Error> {
        let lines = if Path::new(path).exists() {
            fs::read_to_string(path)?
                .lines()
                .map(String::from)
                .collect()
        } else {
            Vec::new()
        };

        Ok(Self { lines })
    }
}
