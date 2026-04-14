use std::path::PathBuf;
use std::collections::HashSet;
use crate::explorer::model::{ExplorerNode, build_explorer_tree};
use core_types::workspace::DirectoryEntry;

// Helper function to normalize paths for consistent comparison
// This must match the implementation in explorer/model.rs
fn normalize_path(path: &std::path::Path) -> PathBuf {
    // Convert to string and remove any trailing separator
    let mut normalized = path.to_string_lossy().to_string();
    // Remove trailing separator if present
    while normalized.ends_with('/') || normalized.ends_with('\\') {
        normalized.pop();
    }
    // Ensure we use the same normalization as in model.rs
    PathBuf::from(normalized)
}

#[derive(Debug, Clone)]
pub enum InlineEditMode {
    None,
    CreateFile { parent: PathBuf },
    CreateFolder { parent: PathBuf },
    Rename { target: PathBuf },
}

#[derive(Debug, Clone)]
pub struct ExplorerState {
    pub workspace_root: PathBuf,
    pub file_tree: Vec<ExplorerNode>,
    pub expanded_directories: HashSet<PathBuf>,
    pub selected_file: Option<PathBuf>,
    pub inline_edit: InlineEditMode,
    pub inline_edit_name: String,
    pub hovered_row: Option<PathBuf>,
    pub show_context_menu: Option<PathBuf>,
}

impl ExplorerState {
    pub fn new() -> Self {
        Self {
            workspace_root: PathBuf::new(),
            file_tree: Vec::new(),
            expanded_directories: HashSet::new(),
            selected_file: None,
            inline_edit: InlineEditMode::None,
            inline_edit_name: String::new(),
            hovered_row: None,
            show_context_menu: None,
        }
    }
    
    pub fn set_workspace_root(&mut self, root: PathBuf) {
        self.workspace_root = normalize_path(&root);
        println!("Explorer: workspace root set to '{}'", self.workspace_root.display());
    }
    
    pub fn set_file_tree(&mut self, entries: Vec<DirectoryEntry>) {
        println!("Explorer: building tree from {} entries", entries.len());
        self.file_tree = build_explorer_tree(&entries);
        println!("Explorer: tree built with {} root nodes", self.file_tree.len());
        // Print first few entries for debugging
        for (i, entry) in entries.iter().take(5).enumerate() {
            println!("  Entry {}: path='{}', is_dir={}", i, entry.path, entry.is_dir);
        }
    }
    
    pub fn toggle_directory(&mut self, path: PathBuf) {
        let normalized_path = normalize_path(&path);
        
        let was_expanded = self.expanded_directories.contains(&normalized_path);
        if was_expanded {
            self.expanded_directories.remove(&normalized_path);
        } else {
            self.expanded_directories.insert(normalized_path.clone());
        }
        
        // Diagnostic log
        println!(
            "Explorer: toggled directory '{}' (normalized: '{}') from {} to {}",
            path.display(),
            normalized_path.display(),
            was_expanded,
            !was_expanded
        );
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
            let normalized_selected_str = normalized_selected.to_string_lossy().to_string();
            
            let normalized_path = normalize_path(path);
            let normalized_path_str = normalized_path.to_string_lossy().to_string();
            
            normalized_selected_str == normalized_path_str
        } else {
            false
        }
    }
    
    pub fn start_create_file(&mut self, parent: PathBuf) {
        self.inline_edit = InlineEditMode::CreateFile { parent };
        self.inline_edit_name = String::new();
    }
    
    pub fn start_create_folder(&mut self, parent: PathBuf) {
        self.inline_edit = InlineEditMode::CreateFolder { parent };
        self.inline_edit_name = String::new();
    }
    
    pub fn start_rename(&mut self, target: PathBuf) {
        // Clone target to avoid move issues
        let target_clone = target.clone();
        self.inline_edit = InlineEditMode::Rename { target: target_clone };
        // Set initial name to current name
        if let Some(node) = self.find_node(&target) {
            self.inline_edit_name = node.name.clone();
        } else {
            self.inline_edit_name = String::new();
        }
    }
    
    pub fn cancel_inline_edit(&mut self) {
        self.inline_edit = InlineEditMode::None;
        self.inline_edit_name.clear();
    }
    
    pub fn set_inline_edit_name(&mut self, name: String) {
        self.inline_edit_name = name;
    }
    
    pub fn set_hovered_row(&mut self, path: Option<PathBuf>) {
        self.hovered_row = path;
    }
    
    pub fn set_context_menu(&mut self, path: Option<PathBuf>) {
        self.show_context_menu = path;
    }
    
    fn find_node(&self, path: &PathBuf) -> Option<&ExplorerNode> {
        fn find_recursive<'a>(nodes: &'a [ExplorerNode], target: &PathBuf) -> Option<&'a ExplorerNode> {
            for node in nodes {
                if &node.path == target {
                    return Some(node);
                }
                if let Some(found) = find_recursive(&node.children, target) {
                    return Some(found);
                }
            }
            None
        }
        find_recursive(&self.file_tree, path)
    }
    
    // Get visible rows for rendering
    pub fn visible_rows(&self) -> Vec<VisibleRow> {
        let mut rows = Vec::new();
        self.collect_visible_rows(&self.file_tree, 0, &mut rows);
        
        // Diagnostic log
        println!(
            "Explorer: generated {} visible rows ({} expanded directories)",
            rows.len(),
            self.expanded_directories.len()
        );
        rows
    }
    
    fn collect_visible_rows(&self, nodes: &[ExplorerNode], depth: usize, rows: &mut Vec<VisibleRow>) {
        for node in nodes {
            let is_expanded = self.is_expanded(&node.path);
            let is_selected = self.is_selected(&node.path);
            let is_hovered = self.hovered_row.as_ref().map_or(false, |hovered| hovered == &node.path);
            
            // Debug: print node info
            println!(
                "Explorer node: path='{}', is_dir={}, depth={}, is_expanded={}, children={}",
                node.path.display(),
                node.is_dir,
                depth,
                is_expanded,
                node.children.len()
            );
            
            rows.push(VisibleRow {
                path: node.path.clone(),
                name: node.name.clone(),
                is_dir: node.is_dir,
                depth,
                is_expanded,
                is_selected,
                is_hovered,
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
    pub is_hovered: bool,
}
