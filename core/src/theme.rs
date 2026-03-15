use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Theme {
    pub background: String,
    pub text: String,
    pub keyword: String,
    pub string: String,
}

impl Theme {
    pub fn load(name: &str) -> Result<Self, std::io::Error> {
        let path = format!("themes/{}.json", name);
        let theme_str = fs::read_to_string(path)?;
        let theme: Theme = serde_json::from_str(&theme_str)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        Ok(theme)
    }
}
