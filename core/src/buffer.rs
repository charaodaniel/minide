use std::{fs, path::{Path, PathBuf}};

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>,
    pub path: Option<PathBuf>,
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
        let path = if path.is_empty() { None } else { Some(PathBuf::from(path)) };

        Ok(Self { lines, path })
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        if let Some(path) = &self.path {
            let content = self.lines.join("\n");
            fs::write(path, content)?;
        }
        Ok(())
    }
}
