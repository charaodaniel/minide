use crate::editor::Editor;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Entry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub depth: usize,
    pub is_expanded: bool,
}

impl Entry {
    fn new(path: &Path, depth: usize) -> Option<Self> {
        Some(Self {
            name: path.file_name()?.to_string_lossy().into_owned(),
            path: path.to_path_buf(),
            is_dir: path.is_dir(),
            depth,
            is_expanded: false,
        })
    }
}

pub struct Explorer {
    pub entries: Vec<Entry>,
    pub selected: usize,
    pub editor: Editor,
}

impl Explorer {
    pub fn new() -> Self {
        let mut entries = Vec::new();
        if let Ok(paths) = fs::read_dir(".") {
            entries = paths
                .flatten()
                .filter_map(|p| Entry::new(&p.path(), 0))
                .collect();
            // Sort: directories first, then files, all alphabetically
            entries.sort_by(|a, b| {
                b.is_dir.cmp(&a.is_dir).then_with(|| a.name.cmp(&b.name))
            });
        }
        Self {
            entries,
            selected: 0,
            editor: Editor::new(),
        }
    }

    pub fn get_files_for_display(&self) -> Vec<String> {
        self.entries
            .iter()
            .map(|entry| {
                let prefix = "  ".repeat(entry.depth);
                let icon = if entry.is_dir {
                    if entry.is_expanded { "v " } else { "> " }
                } else {
                    "  "
                };
                format!("{}{}{}", prefix, icon, entry.name)
            })
            .collect()
    }

    pub fn open_selected(&mut self) {
        if self.selected >= self.entries.len() {
            return;
        }

        if !self.entries[self.selected].is_dir {
            let filename = self.entries[self.selected].path.clone();
            self.editor.open(&filename);
            return;
        }

        let is_expanded = self.entries[self.selected].is_expanded;
        let start_depth = self.entries[self.selected].depth;

        if is_expanded {
            // --- Collapse ---
            self.entries[self.selected].is_expanded = false;
            let mut end_of_children = self.selected + 1;
            while end_of_children < self.entries.len()
                && self.entries[end_of_children].depth > start_depth
            {
                end_of_children += 1;
            }
            if end_of_children > self.selected + 1 {
                self.entries.drain(self.selected + 1..end_of_children);
            }
        } else {
            // --- Expand ---
            self.entries[self.selected].is_expanded = true;
            let path = self.entries[self.selected].path.clone();
            let depth = self.entries[self.selected].depth + 1;

            if let Ok(paths) = fs::read_dir(path) {
                let mut new_entries: Vec<Entry> = paths
                    .flatten()
                    .filter_map(|p| Entry::new(&p.path(), depth))
                    .collect();

                new_entries.sort_by(|a, b| {
                    b.is_dir.cmp(&a.is_dir).then_with(|| a.name.cmp(&b.name))
                });

                for (i, new_entry) in new_entries.into_iter().enumerate() {
                    self.entries.insert(self.selected + 1 + i, new_entry);
                }
            }
        }
    }

    pub fn scroll_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn scroll_down(&mut self) {
        if self.selected < self.entries.len() - 1 {
            self.selected += 1;
        }
    }
}
