use std::path::PathBuf;
use core_types::workspace::DirectoryEntry;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ExplorerNode {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub children: Vec<ExplorerNode>,
}

impl ExplorerNode {
    pub fn new(entry: &DirectoryEntry) -> Self {
        Self {
            path: PathBuf::from(&entry.path),
            name: entry.name.clone(),
            is_dir: entry.is_dir,
            children: Vec::new(),
        }
    }
}

pub fn build_explorer_tree(entries: &[DirectoryEntry]) -> Vec<ExplorerNode> {
    if entries.is_empty() {
        return Vec::new();
    }
    
    // Create all nodes
    let mut nodes: HashMap<PathBuf, ExplorerNode> = HashMap::new();
    for entry in entries {
        let path = normalize_path(std::path::Path::new(&entry.path));
        let node = ExplorerNode {
            path: path.clone(),
            name: entry.name.clone(),
            is_dir: entry.is_dir,
            children: Vec::new(),
        };
        nodes.insert(path, node);
    }
    
    // Build parent-child relationships
    let mut root_nodes = Vec::new();
    let mut path_list: Vec<PathBuf> = nodes.keys().cloned().collect();
    
    // Sort paths by length (deepest first) to ensure children are processed before their parents
    path_list.sort_by_key(|path| std::cmp::Reverse(path.components().count()));
    
    for path in path_list {
        if let Some(mut node) = nodes.remove(&path) {
            // Find parent
            if let Some(parent_path) = path.parent() {
                let parent_path = normalize_path(parent_path);
                if let Some(parent_node) = nodes.get_mut(&parent_path) {
                    // Add as child to parent
                    parent_node.children.push(node);
                    continue;
                }
            }
            // No parent found or parent not in the list -> it's a root node
            root_nodes.push(node);
        }
    }
    
    // Sort children for each node
    fn sort_children(node: &mut ExplorerNode) {
        node.children.sort_by(|a, b| {
            if a.is_dir != b.is_dir {
                b.is_dir.cmp(&a.is_dir) // Directories first
            } else {
                a.name.to_lowercase().cmp(&b.name.to_lowercase())
            }
        });
        for child in &mut node.children {
            sort_children(child);
        }
    }
    
    // Sort root nodes and their children recursively
    root_nodes.sort_by(|a, b| {
        if a.is_dir != b.is_dir {
            b.is_dir.cmp(&a.is_dir)
        } else {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        }
    });
    
    for node in &mut root_nodes {
        sort_children(node);
    }
    
    root_nodes
}

// Helper function to normalize paths for consistent comparison
fn normalize_path(path: &std::path::Path) -> PathBuf {
    // Convert to string and remove any trailing separator
    let mut normalized = path.to_string_lossy().to_string();
    // Remove trailing separator if present
    while normalized.ends_with('/') || normalized.ends_with('\\') {
        normalized.pop();
    }
    PathBuf::from(normalized)
}

