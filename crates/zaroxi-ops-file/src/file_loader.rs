use std::fs;
use std::io::Read;
use thiserror::Error;

use crate::metadata::FileMetadata;

#[derive(Debug, Error)]
pub enum FileLoadError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("File too large: {0}")]
    TooLarge(String),
    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),
}

/// Strategy for loading a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileLoadStrategy {
    /// Load entire file into memory.
    Memory,
    /// Load only a preview (first N bytes).
    Preview(usize),
}

impl FileLoadStrategy {
    /// Determine the appropriate strategy based on file size.
    pub fn for_size(size: u64) -> Self {
        if size <= 10 * 1024 * 1024 {
            FileLoadStrategy::Memory
        } else {
            // For very large files we still read the whole content,
            // but the low‑level editor marks it as read‑only.
            FileLoadStrategy::Memory
        }
    }
}

pub struct FileLoader;

impl FileLoader {
    pub fn load_metadata(path: &str) -> Result<FileMetadata, FileLoadError> {
        let metadata = fs::metadata(path)?;
        Ok(FileMetadata::new(path.to_string(), metadata.len()))
    }

    /// Load a file using the appropriate strategy based on file size.
    pub fn load_file(path: &str) -> Result<(String, u64), FileLoadError> {
        let metadata = fs::metadata(path)?;
        let size = metadata.len();
        let strategy = FileLoadStrategy::for_size(size);
        Self::load_file_with_strategy(path, strategy)
    }

    /// Load a file with a specific strategy.
    pub fn load_file_with_strategy(path: &str, strategy: FileLoadStrategy) -> Result<(String, u64), FileLoadError> {
        let metadata = fs::metadata(path)?;
        let size = metadata.len();

        match strategy {
            FileLoadStrategy::Memory => {
                let content = fs::read_to_string(path)?;
                Ok((content, size))
            }
            FileLoadStrategy::Preview(max_bytes) => {
                let mut file = fs::File::open(path)?;
                let mut buffer = vec![0; max_bytes];
                let bytes_read = file.read(&mut buffer)?;
                let content = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
                Ok((content, size))
            }
        }
    }
}
