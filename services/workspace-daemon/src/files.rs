use std::fs;
use std::path::Path;

use core_types::workspace::*;

#[allow(dead_code)]
pub fn list_directory(path: &str) -> Result<Vec<DirectoryEntry>, String> {
    let dir_path = Path::new(path);
    if !dir_path.exists() {
        return Err(format!("Path does not exist: {}", path));
    }
    if !dir_path.is_dir() {
        return Err(format!("Path is not a directory: {}", path));
    }

    let mut entries = Vec::new();
    // Use a stack for depth-first traversal
    let mut stack = vec![dir_path.to_path_buf()];
    
    while let Some(current_path) = stack.pop() {
        // Read the current directory
        for entry in fs::read_dir(&current_path).map_err(|e| format!("Failed to read directory: {}", e))? {
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
            
            // If it's a directory, add to stack for further traversal
            if is_dir {
                stack.push(path);
            }
        }
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

#[allow(dead_code)]
pub fn read_file(path: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))
}

#[allow(dead_code)]
pub fn write_file(path: &str, content: &str) -> Result<(), String> {
    fs::write(path, content).map_err(|e| format!("Failed to write file: {}", e))
}

#[allow(dead_code)]
pub fn get_file_metadata(path: &str) -> Result<(u64, bool), String> {
    let metadata = fs::metadata(path).map_err(|e| format!("Failed to get file metadata: {}", e))?;
    let size = metadata.len();
    
    // Simple binary detection: check first 1KB for null bytes
    let is_binary = match fs::read(path) {
        Ok(bytes) => bytes.iter().take(1024).any(|&b| b == 0),
        Err(_) => false,
    };
    
    Ok((size, is_binary))
}
