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
    
    // Sort entries: directories first, then files, alphabetically
    let mut sorted_entries: Vec<&DirectoryEntry> = entries.iter().collect();
    sorted_entries.sort_by(|a, b| {
        if a.is_dir != b.is_dir {
            b.is_dir.cmp(&a.is_dir) // Directories first
        } else {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        }
    });
    
    // Create a map from path to node
    let mut path_to_node: HashMap<String, ExplorerNode> = HashMap::new();
    
    // First pass: create all nodes without children
    for entry in sorted_entries {
        let path_str = entry.path.clone();
        let node = ExplorerNode::new(entry);
        path_to_node.insert(path_str, node);
    }
    
    // Second pass: build parent-child relationships
    // We need to collect parent-child pairs first
    let mut parent_to_children: HashMap<String, Vec<String>> = HashMap::new();
    
    for path in path_to_node.keys() {
        let node_path = std::path::Path::new(path);
        if let Some(parent_path) = node_path.parent() {
            let parent_str = parent_path.to_string_lossy().to_string();
            if path_to_node.contains_key(&parent_str) {
                parent_to_children.entry(parent_str)
                    .or_insert_with(Vec::new)
                    .push(path.clone());
            }
        }
    }
    
    // Third pass: build the tree by adding children to their parents
    // We need to collect child nodes first, then add them to parents
    let mut child_nodes_by_parent: HashMap<String, Vec<ExplorerNode>> = HashMap::new();
    
    // First, remove all child nodes from path_to_node
    for (parent_path, child_paths) in parent_to_children {
        let mut children = Vec::new();
        for child_path in child_paths {
            if let Some(child_node) = path_to_node.remove(&child_path) {
                children.push(child_node);
            }
        }
        child_nodes_by_parent.insert(parent_path, children);
    }
    
    // Now, add children to their parents
    for (parent_path, mut children) in child_nodes_by_parent {
        if let Some(parent_node) = path_to_node.get_mut(&parent_path) {
            // Sort children: directories first, then files, alphabetically
            children.sort_by(|a, b| {
                if a.is_dir != b.is_dir {
                    b.is_dir.cmp(&a.is_dir)
                } else {
                    a.name.to_lowercase().cmp(&b.name.to_lowercase())
                }
            });
            
            parent_node.children = children;
        }
    }
    
    // Collect remaining nodes (root nodes)
    let mut root_nodes: Vec<ExplorerNode> = path_to_node.into_values().collect();
    
    // Sort root nodes
    root_nodes.sort_by(|a, b| {
        if a.is_dir != b.is_dir {
            b.is_dir.cmp(&a.is_dir)
        } else {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        }
    });
    
    root_nodes
}

