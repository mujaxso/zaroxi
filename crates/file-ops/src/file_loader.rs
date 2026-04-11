use std::fs;
use std::io::Read;
use thiserror::Error;
use editor_core::Document;

use crate::metadata::FileMetadata;

#[derive(Debug, Error)]
pub enum FileLoadError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("File too large: {0}")]
    TooLarge(String),
}

pub struct FileLoader;

impl FileLoader {
    pub fn load_metadata(path: &str) -> Result<FileMetadata, FileLoadError> {
        let metadata = fs::metadata(path)?;
        Ok(FileMetadata::new(path.to_string(), metadata.len()))
    }

    pub fn load_file(path: &str) -> Result<(String, Document), FileLoadError> {
        let content = fs::read_to_string(path)?;
        let document = Document::from_text(&content);
        Ok((content, document))
    }

    pub fn load_file_preview(path: &str, max_bytes: usize) -> Result<(String, Document), FileLoadError> {
        let mut file = fs::File::open(path)?;
        let mut buffer = vec![0; max_bytes];
        let bytes_read = file.read(&mut buffer)?;
        
        let content = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
        let document = Document::from_text(&content);
        Ok((content, document))
    }
}
