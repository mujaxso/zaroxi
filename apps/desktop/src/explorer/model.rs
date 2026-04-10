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
    
    // Create a map from path to node index
    let mut nodes: Vec<ExplorerNode> = Vec::new();
    let mut path_to_index: HashMap<String, usize> = HashMap::new();
    
    // First pass: create all nodes without children
    for (i, entry) in sorted_entries.iter().enumerate() {
        let path_str = entry.path.clone();
        let node = ExplorerNode::new(entry);
        nodes.push(node);
        path_to_index.insert(path_str, i);
    }
    
    // Second pass: build parent-child relationships
    // We need to collect parent-child relationships first to avoid borrowing issues
    let mut children_by_parent: HashMap<usize, Vec<usize>> = HashMap::new();
    
    for i in 0..nodes.len() {
        let node_path = &nodes[i].path;
        if let Some(parent_path) = node_path.parent() {
            let parent_str = parent_path.to_string_lossy().to_string();
            if let Some(&parent_idx) = path_to_index.get(&parent_str) {
                children_by_parent.entry(parent_idx)
                    .or_insert_with(Vec::new)
                    .push(i);
            }
        }
    }
    
    // Third pass: build the tree by creating new nodes with children
    // We need to create a new tree to avoid ownership issues
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
    
    // Then, add children to their parents
    // We need to process in a topological order (parents before children)
    // But since we're using indices, we can just add children
    for (parent_idx, child_indices) in children_by_parent {
        // Sort children: directories first, then files, alphabetically
        let mut children: Vec<ExplorerNode> = child_indices
            .iter()
            .map(|&idx| new_nodes[idx].clone())
            .collect();
        
        children.sort_by(|a, b| {
            if a.is_dir != b.is_dir {
                b.is_dir.cmp(&a.is_dir)
            } else {
                a.name.to_lowercase().cmp(&b.name.to_lowercase())
            }
        });
        
        new_nodes[parent_idx].children = children;
    }
    
    // Collect root nodes (nodes without parents in the entries)
    let mut root_nodes = Vec::new();
    for i in 0..new_nodes.len() {
        let node_path = &new_nodes[i].path;
        let has_parent = if let Some(parent_path) = node_path.parent() {
            let parent_str = parent_path.to_string_lossy().to_string();
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

