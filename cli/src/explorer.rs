use std::fs;

pub struct Explorer {
    pub files: Vec<String>,
    pub selected: usize,
    pub active_file: Option<String>,
}

impl Explorer {
    pub fn new() -> Self {
        let files = fs::read_dir(".").unwrap()
            .map(|res| res.map(|e| e.file_name().into_string().unwrap()).unwrap())
            .collect();
        Self {
            files,
            selected: 0,
            active_file: None,
        }
    }

    pub fn open_selected(&mut self) {
        let filename = self.files[self.selected].clone();
        self.active_file = fs::read_to_string(filename).ok();
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
