use crate::{
    editor::Editor,
    git::{self, GitStatus},
};
use git2::Repository;
use std::fs;
use std::path::{Path, PathBuf};

pub struct TreeNode {
    pub path: PathBuf,
    pub children: Vec<TreeNode>,
    pub is_expanded: bool,
    pub git_status: Option<GitStatus>,
}

impl TreeNode {
    fn new(path: &Path, repo: &Option<Repository>) -> Self {
        Self {
            path: path.to_path_buf(),
            children: Vec::new(),
            is_expanded: false,
            git_status: repo
                .as_ref()
                .and_then(|r| git::get_file_status(r, path)),
        }
    }
}

fn load_children(node: &mut TreeNode, repo: &Option<Repository>) {
    if node.path.is_dir() {
        if let Ok(entries) = fs::read_dir(&node.path) {
            node.children = entries
                .filter_map(|entry| entry.ok())
                .map(|entry| TreeNode::new(&entry.path(), repo))
                .collect();
        }
    }
}

fn build_flat_list_recursive(
    flat_list: &mut Vec<(usize, PathBuf, Option<GitStatus>)>, // Include status
    node: &TreeNode,
    depth: usize,
) {
    if depth > 0 {
        flat_list.push((depth - 1, node.path.clone(), node.git_status.clone()));
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
    pub flat_list: Vec<(usize, PathBuf, Option<GitStatus>)>, // Include status
    pub repo: Option<Repository>,
}

impl Explorer {
    pub fn new() -> Self {
        let repo = Repository::open(".").ok();
        let mut root = TreeNode::new(Path::new("."), &repo);
        root.is_expanded = true;
        load_children(&mut root, &repo);

        let mut explorer = Self {
            root,
            selected_index: 0,
            editor: Editor::new(),
            flat_list: Vec::new(),
            repo,
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
            .map(|(depth, path, git_status)| {
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

                let status_char = git_status.as_ref().map_or(' ', |s| match s {
                    GitStatus::New => 'A',
                    GitStatus::Modified => 'M',
                    GitStatus::Deleted => 'D',
                    GitStatus::Renamed => 'R',
                    GitStatus::Typechange => 'T',
                    GitStatus::Ignored => 'I',
                    GitStatus::Untracked => '?',
                    GitStatus::Conflicted => 'C',
                    GitStatus::Unmodified => ' ',
                });

                let file_name = path.file_name().unwrap().to_str().unwrap();
                format!("{} {} {} {}", prefix, icon, status_char, file_name)
            })
            .collect()
    }

    pub fn is_expanded(&self, path: &Path) -> bool {
        find_node_by_path(&self.root, path).map_or(false, |n| n.is_expanded)
    }

    pub fn toggle_directory(&mut self) {
        if let Some((_, path, _)) = self.flat_list.get(self.selected_index).cloned() {
            if path.is_dir() {
                let repo = &self.repo;
                if let Some(node) = find_node_by_path_mut(&mut self.root, &path) {
                    node.is_expanded = !node.is_expanded;
                    if node.is_expanded && node.children.is_empty() {
                        load_children(node, repo);
                    }
                }
                self.update_flat_list();
            }
        }
    }

    pub fn open_selected(&mut self) {
        if let Some((_, path, _)) = self.flat_list.get(self.selected_index).cloned() {
            if path.is_dir() {
                self.toggle_directory();
            } else {
                self.editor.open(&path);
            }
        }
    }

    pub fn preview_selected(&mut self) {
        if let Some((_, path, _)) = self.flat_list.get(self.selected_index) {
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
