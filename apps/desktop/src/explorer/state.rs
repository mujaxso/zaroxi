use std::path::PathBuf;
use std::collections::HashSet;
use crate::explorer::model::{ExplorerNode, build_explorer_tree};
use core_types::workspace::DirectoryEntry;

// Helper function to normalize paths for consistent comparison
fn normalize_path(path: &PathBuf) -> PathBuf {
    use std::path::Component;
    
    // Get the components and rebuild the path
    let mut components: Vec<_> = path.components().collect();
    
    // Handle empty path
    if components.is_empty() {
        return PathBuf::new();
    }
    
    // Rebuild path from components
    let mut normalized = PathBuf::new();
    for component in components {
        normalized.push(component);
    }
    
    // Convert to string and remove any trailing separator
    let mut normalized_str = normalized.to_string_lossy().to_string();
    while normalized_str.ends_with(std::path::MAIN_SEPARATOR) {
        normalized_str.pop();
    }
    
    PathBuf::from(normalized_str)
}

#[derive(Debug, Clone)]
pub struct ExplorerState {
    pub workspace_root: PathBuf,
    pub file_tree: Vec<ExplorerNode>,
    pub expanded_directories: HashSet<PathBuf>,
    pub selected_file: Option<PathBuf>,
}

impl ExplorerState {
    pub fn new() -> Self {
        Self {
            workspace_root: PathBuf::new(),
            file_tree: Vec::new(),
            expanded_directories: HashSet::new(),
            selected_file: None,
        }
    }
    
    pub fn set_workspace_root(&mut self, root: PathBuf) {
        self.workspace_root = root;
    }
    
    pub fn set_file_tree(&mut self, entries: Vec<DirectoryEntry>) {
        self.file_tree = build_explorer_tree(&entries);
    }
    
    pub fn toggle_directory(&mut self, path: PathBuf) {
        // Normalize the path by removing any trailing separator
        let normalized_path = normalize_path(&path);
        if self.expanded_directories.contains(&normalized_path) {
            self.expanded_directories.remove(&normalized_path);
        } else {
            self.expanded_directories.insert(normalized_path);
        }
    }
    
    pub fn select_file(&mut self, path: PathBuf) {
        let normalized_path = normalize_path(&path);
        self.selected_file = Some(normalized_path);
    }
    
    pub fn is_expanded(&self, path: &PathBuf) -> bool {
        let normalized_path = normalize_path(path);
        self.expanded_directories.contains(&normalized_path)
    }
    
    pub fn is_selected(&self, path: &PathBuf) -> bool {
        if let Some(selected) = &self.selected_file {
            let normalized_selected = normalize_path(selected);
            let normalized_path = normalize_path(path);
            normalized_selected == normalized_path
        } else {
            false
        }
    }
    
    // Get visible rows for rendering
    pub fn visible_rows(&self) -> Vec<VisibleRow> {
        let mut rows = Vec::new();
        self.collect_visible_rows(&self.file_tree, 0, &mut rows);
        rows
    }
    
    fn collect_visible_rows(&self, nodes: &[ExplorerNode], depth: usize, rows: &mut Vec<VisibleRow>) {
        for node in nodes {
            let is_expanded = self.is_expanded(&node.path);
            rows.push(VisibleRow {
                path: node.path.clone(),
                name: node.name.clone(),
                is_dir: node.is_dir,
                depth,
                is_expanded,
                is_selected: self.is_selected(&node.path),
            });
            
            if node.is_dir && is_expanded {
                self.collect_visible_rows(&node.children, depth + 1, rows);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct VisibleRow {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub depth: usize,
    pub is_expanded: bool,
    pub is_selected: bool,
}
