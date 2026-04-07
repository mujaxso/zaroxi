use std::fs;
use std::path::Path;

use core_types::workspace::*;

pub fn list_directory(path: &str) -> Result<Vec<DirectoryEntry>, String> {
    let dir_path = Path::new(path);
    if !dir_path.exists() {
        return Err(format!("Path does not exist: {}", path));
    }
    if !dir_path.is_dir() {
        return Err(format!("Path is not a directory: {}", path));
    }

    let mut entries = Vec::new();
    for entry in fs::read_dir(dir_path).map_err(|e| format!("Failed to read directory: {}", e))? {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let path = entry.path();
        let name = entry
            .file_name()
            .into_string()
            .map_err(|_| "Invalid UTF-8 in filename".to_string())?;
        
        let is_dir = path.is_dir();
        let path_str = path.to_string_lossy().to_string();
        
        entries.push(DirectoryEntry {
            path: path_str,
            name,
            is_dir,
        });
    }
    
    // Sort directories first, then files
    entries.sort_by(|a, b| {
        if a.is_dir && !b.is_dir {
            std::cmp::Ordering::Less
        } else if !a.is_dir && b.is_dir {
            std::cmp::Ordering::Greater
        } else {
            a.name.cmp(&b.name)
        }
    });
    
    Ok(entries)
}

pub fn read_file(path: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))
}

pub fn write_file(path: &str, content: &str) -> Result<(), String> {
    fs::write(path, content).map_err(|e| format!("Failed to write file: {}", e))
}
