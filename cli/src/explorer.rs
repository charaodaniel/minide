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

    pub fn scroll_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn scroll_down(&mut self) {
        if self.selected < self.files.len() - 1 {
            self.selected += 1;
        }
    }
}
