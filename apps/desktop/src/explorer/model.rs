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
            for child_entry in all_entries {
                let child_path = PathBuf::from(&child_entry.path);
                if let Some(parent) = child_path.parent() {
                    if parent == path {
                        children.push(ExplorerNode::from_directory_entry(child_entry, all_entries));
                    }
                }
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
    let mut root_nodes = Vec::new();
    
    // First, find all entries that don't have a parent in the list (i.e., are at root level)
    for entry in entries {
        let path = PathBuf::from(&entry.path);
        let mut has_parent_in_list = false;
        
        for other_entry in entries {
            if other_entry.path == entry.path {
                continue;
            }
            let other_path = PathBuf::from(&other_entry.path);
            if path.parent() == Some(&other_path) {
                has_parent_in_list = true;
                break;
            }
        }
        
        if !has_parent_in_list {
            root_nodes.push(ExplorerNode::from_directory_entry(entry, entries));
        }
    }
    
    root_nodes
}
