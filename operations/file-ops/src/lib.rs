//! File operations for Zaroxi

use serde::{Deserialize, Serialize};

/// A file entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    /// Path
    pub path: String,
    /// Name
    pub name: String,
    /// Is directory
    pub is_dir: bool,
}

impl FileEntry {
    /// Create a new file entry
    pub fn new(path: String, name: String, is_dir: bool) -> Self {
        Self { path, name, is_dir }
    }
}
