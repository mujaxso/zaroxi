use std::fs;
use std::io::Read;
use std::path::Path;
use thiserror::Error;
use memmap2::Mmap;

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

/// Source of file content for the editor.
#[derive(Debug, Clone)]
pub enum FileSource {
    /// File loaded entirely into memory (small files).
    Memory(String),
    /// File memory-mapped (large files).
    Mmap(Mmap),
}

impl FileSource {
    /// Get the content as a string slice.
    pub fn as_str(&self) -> &str {
        match self {
            FileSource::Memory(s) => s.as_str(),
            FileSource::Mmap(m) => {
                // SAFETY: The mmap is valid for the lifetime of the FileSource.
                // We assume the file is valid UTF-8.
                unsafe { std::str::from_utf8_unchecked(m) }
            }
        }
    }

    /// Get the length in bytes.
    pub fn len(&self) -> usize {
        match self {
            FileSource::Memory(s) => s.len(),
            FileSource::Mmap(m) => m.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
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

/// Thresholds for large file handling (in bytes).
pub const LARGE_FILE_THRESHOLD: u64 = 10 * 1024 * 1024; // 10 MB
pub const VERY_LARGE_FILE_THRESHOLD: u64 = 100 * 1024 * 1024; // 100 MB

/// Strategy for loading a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileLoadStrategy {
    /// Load entire file into memory.
    Memory,
    /// Memory-map the file.
    Mmap,
    /// Load only a preview (first N bytes).
    Preview(usize),
}

impl FileLoadStrategy {
    /// Determine the appropriate strategy based on file size.
    pub fn for_size(size: u64) -> Self {
        if size <= LARGE_FILE_THRESHOLD {
            FileLoadStrategy::Memory
        } else if size <= VERY_LARGE_FILE_THRESHOLD {
            FileLoadStrategy::Mmap
        } else {
            // For very large files, we still mmap but may need to limit features
            FileLoadStrategy::Mmap
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
    pub fn load_file(path: &str) -> Result<(FileSource, Document), FileLoadError> {
        let metadata = fs::metadata(path)?;
        let size = metadata.len();
        let strategy = FileLoadStrategy::for_size(size);
        Self::load_file_with_strategy(path, strategy)
    }

    /// Load a file with a specific strategy.
    pub fn load_file_with_strategy(path: &str, strategy: FileLoadStrategy) -> Result<(FileSource, Document), FileLoadError> {
        match strategy {
            FileLoadStrategy::Memory => {
                let content = fs::read_to_string(path)?;
                let source = FileSource::Memory(content.clone());
                let document = Document::from_text(&content);
                Ok((source, document))
            }
            FileLoadStrategy::Mmap => {
                let file = fs::File::open(path)?;
                let mmap = unsafe { Mmap::map(&file)? };
                // Validate UTF-8
                let _ = std::str::from_utf8(&mmap)?;
                let source = FileSource::Mmap(mmap);
                // For mmap, we create a Document with empty text initially
                // The actual content is accessed via the FileSource
                let document = Document::from_text("");
                Ok((source, document))
            }
            FileLoadStrategy::Preview(max_bytes) => {
                Self::load_file_preview(path, max_bytes)
            }
        }
    }

    pub fn load_file_preview(path: &str, max_bytes: usize) -> Result<(FileSource, Document), FileLoadError> {
        let mut file = fs::File::open(path)?;
        let mut buffer = vec![0; max_bytes];
        let bytes_read = file.read(&mut buffer)?;
        
        let content = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
        let source = FileSource::Memory(content.clone());
        let document = Document::from_text(&content);
        Ok((source, document))
    }
}
