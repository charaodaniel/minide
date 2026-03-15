use std::fs;
use std::path::PathBuf;

pub struct Editor {
    pub path: Option<PathBuf>,
    pub content: Vec<String>,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub search_term: String,
    pub search_results: Vec<(usize, usize)>,
    pub selected_search_result: Option<usize>,
    pub is_searching: bool,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            path: None,
            content: Vec::new(),
            cursor_x: 0,
            cursor_y: 0,
            search_term: String::new(),
            search_results: Vec::new(),
            selected_search_result: None,
            is_searching: false,
        }
    }

    pub fn open(&mut self, path: &PathBuf) {
        if let Ok(content_str) = fs::read_to_string(path) {
            self.content = content_str.lines().map(|s| s.to_string()).collect();
        } else {
            self.content = Vec::new();
        }
        self.path = Some(path.clone());
        self.cursor_x = 0;
        self.cursor_y = 0;
        self.cancel_search();
    }

    pub fn get_content_as_string(&self) -> String {
        self.content.join("\n")
    }

    pub fn perform_search(&mut self) {
        self.search_results.clear();
        if self.search_term.is_empty() {
            return;
        }

        for (line_idx, line) in self.content.iter().enumerate() {
            for (char_idx, _) in line.match_indices(&self.search_term) {
                self.search_results.push((line_idx, char_idx));
            }
        }

        if !self.search_results.is_empty() {
            self.selected_search_result = Some(0);
            self.jump_to_selected_result();
        }
    }

    pub fn cancel_search(&mut self) {
        self.is_searching = false;
        self.search_term.clear();
        self.search_results.clear();
        self.selected_search_result = None;
    }

    fn jump_to_selected_result(&mut self) {
        if let Some(selected) = self.selected_search_result {
            if let Some((line, col)) = self.search_results.get(selected) {
                self.cursor_y = *line;
                self.cursor_x = *col;
            }
        }
    }

    pub fn next_result(&mut self) {
        if let Some(selected) = self.selected_search_result {
            let next = if selected >= self.search_results.len() - 1 {
                0
            } else {
                selected + 1
            };
            self.selected_search_result = Some(next);
            self.jump_to_selected_result();
        }
    }

    pub fn previous_result(&mut self) {
        if let Some(selected) = self.selected_search_result {
            let prev = if selected == 0 {
                self.search_results.len() - 1
            } else {
                selected - 1
            };
            self.selected_search_result = Some(prev);
            self.jump_to_selected_result();
        }
    }
    
    pub fn insert_char(&mut self, c: char) {
        if self.is_searching {
            self.search_term.push(c);
            self.perform_search();
        } else if self.cursor_y < self.content.len() {
            if self.cursor_x <= self.content[self.cursor_y].len() {
                self.content[self.cursor_y].insert(self.cursor_x, c);
                self.cursor_x += 1;
            }
        } else if self.cursor_y == self.content.len() {
            self.content.push(c.to_string());
            self.cursor_x = 1;
        }
    }

    pub fn delete_char(&mut self) {
        if self.is_searching {
             self.search_term.pop();
             self.perform_search();
        } else if self.cursor_y < self.content.len() {
            if self.cursor_x > 0 {
                self.content[self.cursor_y].remove(self.cursor_x - 1);
                self.cursor_x -= 1;
            } else if self.cursor_y > 0 {
                let prev_line_len = self.content[self.cursor_y - 1].len();
                let current_line = self.content.remove(self.cursor_y);
                self.content[self.cursor_y - 1].push_str(&current_line);
                self.cursor_y -= 1;
                self.cursor_x = prev_line_len;
            }
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
        } else if self.cursor_y > 0 {
            self.cursor_y -= 1;
            self.cursor_x = self.content[self.cursor_y].len();
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_y < self.content.len() {
            if self.cursor_x < self.content[self.cursor_y].len() {
                self.cursor_x += 1;
            } else if self.cursor_y < self.content.len() - 1 {
                self.cursor_y += 1;
                self.cursor_x = 0;
            }
        }
    }

    pub fn move_cursor_up(&mut self) {
        if self.cursor_y > 0 {
            self.cursor_y -= 1;
        }
        if !self.content.is_empty() {
            self.cursor_x = self.cursor_x.min(self.content[self.cursor_y].len());
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.cursor_y < self.content.len() - 1 {
            self.cursor_y += 1;
        }
        if !self.content.is_empty() {
            self.cursor_x = self.cursor_x.min(self.content[self.cursor_y].len());
        }
    }
}
