use std::fs;

pub struct Explorer {
    pub files: Vec<String>,
    pub selected: usize,
}

impl Explorer {
    pub fn new() -> Self {
        let files = fs::read_dir(".").unwrap()
            .map(|res| res.map(|e| e.file_name().into_string().unwrap()).unwrap())
            .collect();
        Self { files, selected: 0 }
    }
}
