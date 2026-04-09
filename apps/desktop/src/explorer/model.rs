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
    
    // Convert entries to nodes
    let mut nodes: Vec<ExplorerNode> = entries
        .iter()
        .map(|entry| ExplorerNode::new(entry))
        .collect();
    
    // Sort nodes: directories first, then files, alphabetically
    nodes.sort_by(|a, b| {
        if a.is_dir != b.is_dir {
            b.is_dir.cmp(&a.is_dir) // Directories first
        } else {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        }
    });
    
    // Build a map from path to node index
    let mut path_to_index: HashMap<String, usize> = HashMap::new();
    for (i, node) in nodes.iter().enumerate() {
        let mut path_str = node.path.to_string_lossy().to_string();
        while path_str.ends_with('/') || path_str.ends_with('\\') {
            path_str.pop();
        }
        path_to_index.insert(path_str, i);
    }
    
    // For each node, find its parent and add it as a child
    // We need to work with indices because we can't have multiple mutable references
    let mut children_by_parent: HashMap<usize, Vec<usize>> = HashMap::new();
    
    for i in 0..nodes.len() {
        let node_path = &nodes[i].path;
        if let Some(parent_path) = node_path.parent() {
            let mut parent_str = parent_path.to_string_lossy().to_string();
            while parent_str.ends_with('/') || parent_str.ends_with('\\') {
                parent_str.pop();
            }
            
            if let Some(&parent_idx) = path_to_index.get(&parent_str) {
                children_by_parent.entry(parent_idx)
                    .or_insert_with(Vec::new)
                    .push(i);
            }
        }
    }
    
    // Now build the tree by creating new nodes with children
    let mut new_nodes: Vec<ExplorerNode> = Vec::with_capacity(nodes.len());
    
    // First, create all nodes without children
    for node in &nodes {
        new_nodes.push(ExplorerNode {
            path: node.path.clone(),
            name: node.name.clone(),
            is_dir: node.is_dir,
            children: Vec::new(),
        });
    }
    
    // Then, add children
    for (parent_idx, child_indices) in &children_by_parent {
        let mut children = Vec::new();
        for &child_idx in child_indices {
            children.push(new_nodes[child_idx].clone());
        }
        // Sort children
        children.sort_by(|a, b| {
            if a.is_dir != b.is_dir {
                b.is_dir.cmp(&a.is_dir)
            } else {
                a.name.to_lowercase().cmp(&b.name.to_lowercase())
            }
        });
        new_nodes[*parent_idx].children = children;
    }
    
    // Collect root nodes (nodes without parents)
    let mut root_nodes = Vec::new();
    for i in 0..new_nodes.len() {
        let node_path = &new_nodes[i].path;
        let has_parent = if let Some(parent_path) = node_path.parent() {
            let mut parent_str = parent_path.to_string_lossy().to_string();
            while parent_str.ends_with('/') || parent_str.ends_with('\\') {
                parent_str.pop();
            }
            path_to_index.contains_key(&parent_str)
        } else {
            false
        };
        
        if !has_parent {
            root_nodes.push(new_nodes[i].clone());
        }
    }
    
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

