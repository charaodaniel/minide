use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Deserialize)]
pub struct Icons {
    pub file: String,
    pub folder: String,
    pub extensions: HashMap<String, String>,
}

impl Icons {
    pub fn load(name: &str) -> Result<Self, std::io::Error> {
        let path = format!("icons/{}.json", name);
        let icons_str = fs::read_to_string(path)?;
        let icons: Icons = serde_json::from_str(&icons_str)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        Ok(icons)
    }
}
