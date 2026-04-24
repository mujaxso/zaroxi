//! File tree representation for workspaces.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// A node in the file tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileTreeNode {
    /// A directory containing other nodes.
    Directory {
        /// Path to the directory.
        path: PathBuf,
        /// Name of the directory.
        name: String,
        /// Children nodes.
        children: Vec<FileTreeNode>,
    },
    /// A file.
    File {
        /// Path to the file.
        path: PathBuf,
        /// Name of the file.
        name: String,
        /// File extension, if any.
        extension: Option<String>,
        /// File size in bytes.
        size: u64,
    },
}

impl FileTreeNode {
    /// Create a directory node.
    pub fn directory(path: PathBuf) -> Self {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "".to_string());
        Self::Directory { path, name, children: Vec::new() }
    }

    /// Create a file node.
    pub fn file(path: PathBuf, size: u64) -> Self {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "".to_string());
        let extension = path.extension().and_then(|e| e.to_str()).map(|s| s.to_string());
        Self::File { path, name, extension, size }
    }

    /// Get the path of the node.
    pub fn path(&self) -> &Path {
        match self {
            FileTreeNode::Directory { path, .. } => path,
            FileTreeNode::File { path, .. } => path,
        }
    }

    /// Get the name of the node.
    pub fn name(&self) -> &str {
        match self {
            FileTreeNode::Directory { name, .. } => name,
            FileTreeNode::File { name, .. } => name,
        }
    }
}

/// A file tree representing the structure of a workspace.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FileTree {
    /// The root node of the tree.
    pub root: Option<FileTreeNode>,
}

impl FileTree {
    /// Create a new empty file tree.
    pub fn new() -> Self {
        Self { root: None }
    }

    /// Set the root node.
    pub fn set_root(&mut self, root: FileTreeNode) {
        self.root = Some(root);
    }
}
