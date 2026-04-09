use std::path::PathBuf;
use core_types::workspace::DirectoryEntry;

#[derive(Debug, Clone)]
pub struct ExplorerNode {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub children: Vec<ExplorerNode>,
}

impl ExplorerNode {
    pub fn from_directory_entry(entry: &DirectoryEntry, all_entries: &[DirectoryEntry]) -> Self {
        let path = PathBuf::from(&entry.path);
        let name = entry.name.clone();
        let is_dir = entry.is_dir;
        
        // Collect children
        let mut children = Vec::new();
        if is_dir {
            // First, collect all potential children
            let mut child_entries: Vec<&DirectoryEntry> = all_entries
                .iter()
                .filter(|child_entry| {
                    let child_path = PathBuf::from(&child_entry.path);
                    if let Some(parent) = child_path.parent() {
                        // Check if this child is directly inside the current directory
                        parent == path
                    } else {
                        false
                    }
                })
                .collect();
            
            // Sort children: directories first, then files, both alphabetically
            child_entries.sort_by(|a, b| {
                // Directories first
                if a.is_dir != b.is_dir {
                    b.is_dir.cmp(&a.is_dir) // true (dir) comes before false (file)
                } else {
                    // Then alphabetically
                    a.name.to_lowercase().cmp(&b.name.to_lowercase())
                }
            });
            
            // Recursively build child nodes
            for child_entry in child_entries {
                children.push(ExplorerNode::from_directory_entry(child_entry, all_entries));
            }
        }
        
        ExplorerNode {
            path,
            name,
            is_dir,
            children,
        }
    }
}

pub fn build_explorer_tree(entries: &[DirectoryEntry]) -> Vec<ExplorerNode> {
    if entries.is_empty() {
        return Vec::new();
    }
    
    // Find workspace root (the shortest path)
    let workspace_root = entries
        .iter()
        .map(|e| PathBuf::from(&e.path))
        .min_by_key(|p| p.components().count())
        .unwrap_or_else(|| PathBuf::from(""));
    
    // Find root entries (those whose parent is the workspace root or don't have a parent in the list)
    let mut root_entries: Vec<&DirectoryEntry> = Vec::new();
    
    for entry in entries {
        let path = PathBuf::from(&entry.path);
        if let Some(parent) = path.parent() {
            if parent == workspace_root {
                root_entries.push(entry);
            } else {
                // Check if parent exists in entries
                let parent_exists = entries.iter().any(|e| {
                    PathBuf::from(&e.path) == parent
                });
                if !parent_exists {
                    root_entries.push(entry);
                }
            }
        } else {
            // Entry has no parent (shouldn't happen with proper paths)
            root_entries.push(entry);
        }
    }
    
    // Sort root entries: directories first, then files, both alphabetically
    root_entries.sort_by(|a, b| {
        // Directories first
        if a.is_dir != b.is_dir {
            b.is_dir.cmp(&a.is_dir) // true (dir) comes before false (file)
        } else {
            // Then alphabetically
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        }
    });
    
    // Build tree from root entries
    let mut root_nodes = Vec::new();
    for entry in root_entries {
        root_nodes.push(ExplorerNode::from_directory_entry(entry, entries));
    }
    
    root_nodes
}
