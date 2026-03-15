use crate::editor::Editor;
use std::fs;
use std::path::{Path, PathBuf};

pub struct TreeNode {
    pub path: PathBuf,
    pub children: Vec<TreeNode>,
    pub is_expanded: bool,
}

impl TreeNode {
    fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
            children: Vec::new(),
            is_expanded: false,
        }
    }
}

fn load_children(node: &mut TreeNode) {
    if node.path.is_dir() {
        if let Ok(entries) = fs::read_dir(&node.path) {
            node.children = entries
                .filter_map(|entry| entry.ok())
                .map(|entry| TreeNode::new(&entry.path()))
                .collect();
        }
    }
}

fn build_flat_list_recursive(
    flat_list: &mut Vec<(usize, PathBuf)>,
    node: &TreeNode,
    depth: usize,
) {
    if depth > 0 {
        flat_list.push((depth - 1, node.path.clone()));
    }

    if node.is_expanded {
        for child in &node.children {
            build_flat_list_recursive(flat_list, child, depth + 1);
        }
    }
}

fn find_node_by_path<'a>(node: &'a TreeNode, path: &Path) -> Option<&'a TreeNode> {
    if node.path == path {
        return Some(node);
    }
    for child in &node.children {
        if let Some(found) = find_node_by_path(child, path) {
            return Some(found);
        }
    }
    None
}

fn find_node_by_path_mut<'a>(
    node: &'a mut TreeNode,
    path: &Path,
) -> Option<&'a mut TreeNode> {
    if node.path == path {
        return Some(node);
    }
    for child in &mut node.children {
        if let Some(found) = find_node_by_path_mut(child, path) {
            return Some(found);
        }
    }
    None
}

pub struct Explorer {
    pub root: TreeNode,
    pub selected_index: usize,
    pub editor: Editor,
    pub flat_list: Vec<(usize, PathBuf)>,
}

impl Explorer {
    pub fn new() -> Self {
        let mut root = TreeNode::new(Path::new("."));
        root.is_expanded = true;
        load_children(&mut root);

        let mut explorer = Self {
            root,
            selected_index: 0,
            editor: Editor::new(),
            flat_list: Vec::new(),
        };
        explorer.update_flat_list();
        explorer
    }

    fn update_flat_list(&mut self) {
        self.flat_list.clear();
        build_flat_list_recursive(&mut self.flat_list, &self.root, 0);
    }

    pub fn get_files_for_display(&self) -> Vec<String> {
        self.flat_list
            .iter()
            .map(|(depth, path)| {
                let is_dir = path.is_dir();
                let prefix = "  ".repeat(*depth);
                let icon = if is_dir {
                    if self.is_expanded(path) {
                        "▼"
                    } else {
                        "▶"
                    }
                } else {
                    " "
                };
                let file_name = path.file_name().unwrap().to_str().unwrap();
                format!("{} {} {}", prefix, icon, file_name)
            })
            .collect()
    }

    fn is_expanded(&self, path: &Path) -> bool {
        find_node_by_path(&self.root, path).map_or(false, |n| n.is_expanded)
    }

    pub fn toggle_directory(&mut self) {
        if let Some((_, path)) = self.flat_list.get(self.selected_index).cloned() {
            if path.is_dir() {
                if let Some(node) = find_node_by_path_mut(&mut self.root, &path) {
                    node.is_expanded = !node.is_expanded;
                    if node.is_expanded && node.children.is_empty() {
                        load_children(node);
                    }
                }
                self.update_flat_list();
            }
        }
    }

    pub fn open_selected(&mut self) {
        if let Some((_, path)) = self.flat_list.get(self.selected_index).cloned() {
            if path.is_dir() {
                self.toggle_directory();
            } else {
                self.editor.open(&path);
            }
        }
    }

    pub fn preview_selected(&mut self) {
        if let Some((_, path)) = self.flat_list.get(self.selected_index) {
            if path.is_file() {
                self.editor.preview(path);
            } else {
                self.editor.close();
            }
        }
    }

    pub fn select_next(&mut self) {
        if self.selected_index < self.flat_list.len() - 1 {
            self.selected_index += 1;
            self.preview_selected();
        }
    }

    pub fn select_previous(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
            self.preview_selected();
        }
    }

    pub fn scroll_up(&mut self) {
        self.select_previous();
    }

    pub fn scroll_down(&mut self) {
        self.select_next();
    }
}
