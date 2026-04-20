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
}

/// A simple document structure for file loading.
pub struct Document {
    /// The text content.
    pub text: String,
}

impl Document {
    /// Create a document from text.
    pub fn from_text(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }
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
