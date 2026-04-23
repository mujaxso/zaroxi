//! Text document model with rope-based storage.

use ropey::Rope;
use std::borrow::Cow;
use std::path::PathBuf;
use memmap2::Mmap;

/// Source of file content for the editor.
#[derive(Debug)]
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

/// Large file mode indicator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LargeFileMode {
    /// Normal mode (file fits in memory).
    Normal,
    /// Large file mode (file is memory-mapped, some features may be reduced).
    Large,
    /// Very large file mode (file is memory-mapped, read-only recommended).
    VeryLarge,
}

impl LargeFileMode {
    /// Determine the mode based on file size in bytes.
    pub fn from_size(size: u64) -> Self {
        if size <= 10_000_000 {
            LargeFileMode::Normal
        } else if size <= 100_000_000 {
            LargeFileMode::Large
        } else {
            LargeFileMode::VeryLarge
        }
    }

    /// Whether syntax highlighting should be disabled.
    pub fn disable_syntax_highlighting(&self) -> bool {
        matches!(self, LargeFileMode::Large | LargeFileMode::VeryLarge)
    }

    /// Whether the document should be read-only.
    pub fn is_read_only(&self) -> bool {
        matches!(self, LargeFileMode::VeryLarge)
    }
}

/// A text document with efficient editing operations.
#[derive(Debug)]
pub struct Document {
    rope: Rope,
    version: u64,
    dirty: bool,
    path: Option<PathBuf>,
    file_source: Option<FileSource>,
    large_file_mode: LargeFileMode,
}

impl Document {
    /// Create a new empty document.
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            version: 0,
            dirty: false,
            path: None,
            file_source: None,
            large_file_mode: LargeFileMode::Normal,
        }
    }

    /// Create a document from text.
    pub fn from_text(text: &str) -> Self {
        Self {
            rope: Rope::from_str(text),
            version: 0,
            dirty: false,
            path: None,
            file_source: None,
            large_file_mode: LargeFileMode::Normal,
        }
    }

    /// Create a document from text with a file path.
    pub fn from_text_with_path(text: &str, path: String) -> Self {
        Self {
            rope: Rope::from_str(text),
            version: 0,
            dirty: false,
            path: Some(PathBuf::from(path)),
            file_source: None,
            large_file_mode: LargeFileMode::Normal,
        }
    }

    /// Create a document from a memory-mapped file.
    pub fn from_mmap(mmap: Mmap, path: String, size: u64) -> Self {
        let mode = LargeFileMode::from_size(size);
        // For large files, we don't load into rope; we keep the mmap as source.
        // The rope will be populated lazily as needed.
        let rope = if mode == LargeFileMode::Normal {
            // SAFETY: We validated UTF-8 when creating the mmap.
            let text = unsafe { std::str::from_utf8_unchecked(&mmap) };
            Rope::from_str(text)
        } else {
            Rope::new()
        };

        Self {
            rope,
            version: 0,
            dirty: false,
            path: Some(PathBuf::from(path)),
            file_source: Some(FileSource::Mmap(mmap)),
            large_file_mode: mode,
        }
    }

    /// Get the document's text as a string.
    /// For large files, this may be expensive; prefer using `get_line` or `slice`.
    pub fn text(&self) -> String {
        if let Some(source) = &self.file_source {
            source.as_str().to_string()
        } else {
            self.rope.to_string()
        }
    }

    /// Get the number of characters in the document.
    pub fn len_chars(&self) -> usize {
        if let Some(source) = &self.file_source {
            source.as_str().chars().count()
        } else {
            self.rope.len_chars()
        }
    }

    /// Get the number of lines in the document.
    pub fn len_lines(&self) -> usize {
        if let Some(source) = &self.file_source {
            // Count newlines efficiently using byte iteration
            let text = source.as_str();
            let count = text.bytes().filter(|&b| b == b'\n').count();
            // If the file doesn't end with a newline, add one for the last line
            if text.is_empty() {
                0
            } else if text.as_bytes().last() == Some(&b'\n') {
                count
            } else {
                count + 1
            }
        } else {
            self.rope.len_lines()
        }
    }

    /// Check if the document is empty.
    pub fn is_empty(&self) -> bool {
        self.len_chars() == 0
    }

    /// Get a line by index (0-based).
    /// Returns `None` if the line index is out of bounds.
    pub fn line(&self, line_idx: usize) -> Option<String> {
        if let Some(source) = &self.file_source {
            let text = source.as_str();
            let mut start = 0usize;
            let mut current_line = 0usize;
            for (_i, ch) in text.char_indices() {
                if ch == '\n' {
                    if current_line == line_idx {
                        return Some(text[start.._i].to_string());
                    }
                    start = _i + 1;
                    current_line += 1;
                }
            }
            // Handle last line without trailing newline
            if current_line == line_idx {
                return Some(text[start..].to_string());
            }
            None
        } else {
            if line_idx >= self.rope.len_lines() {
                return None;
            }
            Some(self.rope.line(line_idx).to_string())
        }
    }

    /// Get a line by index (0-based) as a `Cow<str>` to avoid allocation when possible.
    pub fn line_cow(&self, line_idx: usize) -> Option<Cow<'_, str>> {
        if let Some(source) = &self.file_source {
            let text = source.as_str();
            let mut start = 0usize;
            let mut current_line = 0usize;
            for (_i, ch) in text.char_indices() {
                if ch == '\n' {
                    if current_line == line_idx {
                        return Some(Cow::Borrowed(&text[start.._i]));
                    }
                    start = _i + 1;
                    current_line += 1;
                }
            }
            if current_line == line_idx {
                return Some(Cow::Borrowed(&text[start..]));
            }
            None
        } else {
            if line_idx >= self.rope.len_lines() {
                return None;
            }
            Some(Cow::Owned(self.rope.line(line_idx).to_string()))
        }
    }

    /// Get the character index for a line and column.
    pub fn line_col_to_char(&self, line: usize, col: usize) -> Option<usize> {
        if let Some(source) = &self.file_source {
            let text = source.as_str();
            let mut current_line = 0usize;
            let mut char_idx = 0usize;
            for (_i, ch) in text.char_indices() {
                if current_line == line {
                    if col == 0 {
                        return Some(char_idx);
                    }
                    if ch == '\n' {
                        return None;
                    }
                    char_idx += 1;
                }
                if ch == '\n' {
                    current_line += 1;
                    if current_line > line {
                        break;
                    }
                }
            }
            None
        } else {
            if line >= self.rope.len_lines() {
                return None;
            }
            let line_start = self.rope.line_to_char(line);
            let line_len = self.rope.line(line).len_chars();
            if col > line_len {
                return None;
            }
            Some(line_start + col)
        }
    }

    /// Get the line and column for a character index.
    pub fn char_to_line_col(&self, char_idx: usize) -> Option<(usize, usize)> {
        if let Some(source) = &self.file_source {
            let text = source.as_str();
            let mut line = 0usize;
            let mut col = 0usize;
            for (i, ch) in text.char_indices() {
                if i >= char_idx {
                    break;
                }
                if ch == '\n' {
                    line += 1;
                    col = 0;
                } else {
                    col += 1;
                }
            }
            Some((line, col))
        } else {
            if char_idx > self.rope.len_chars() {
                return None;
            }
            let line = self.rope.char_to_line(char_idx);
            let line_start = self.rope.line_to_char(line);
            let col = char_idx - line_start;
            Some((line, col))
        }
    }

    /// Get the character index for the start of a line.
    pub fn line_to_char(&self, line: usize) -> usize {
        if let Some(source) = &self.file_source {
            let text = source.as_str();
            let mut current_line = 0usize;
            let mut char_idx = 0usize;
            for (_i, ch) in text.char_indices() {
                if current_line == line {
                    return char_idx;
                }
                if ch == '\n' {
                    current_line += 1;
                }
                char_idx += 1;
            }
            // If line is beyond the last line, return the end
            text.chars().count()
        } else {
            self.rope.line_to_char(line)
        }
    }

    /// Insert text at a character position.
    pub fn insert(&mut self, char_idx: usize, text: &str) -> Result<(), String> {
        if self.large_file_mode.is_read_only() {
            return Err("Document is read-only (very large file)".to_string());
        }
        // For large files, we need to load the relevant portion into the rope
        self.ensure_rope_loaded();
        if char_idx > self.rope.len_chars() {
            return Err(format!("Char index {} out of bounds", char_idx));
        }
        self.rope.insert(char_idx, text);
        self.version += 1;
        self.dirty = true;
        Ok(())
    }

    /// Delete a range of characters.
    pub fn delete(&mut self, start: usize, end: usize) -> Result<(), String> {
        if self.large_file_mode.is_read_only() {
            return Err("Document is read-only (very large file)".to_string());
        }
        self.ensure_rope_loaded();
        if start > end {
            return Err(format!("Start {} greater than end {}", start, end));
        }
        if end > self.rope.len_chars() {
            return Err(format!("End {} out of bounds", end));
        }
        self.rope.remove(start..end);
        self.version += 1;
        self.dirty = true;
        Ok(())
    }

    /// Replace the entire document content.
    pub fn replace_all(&mut self, text: &str) {
        if self.large_file_mode.is_read_only() {
            return;
        }
        self.rope = Rope::from_str(text);
        self.version += 1;
        self.dirty = true;
    }

    /// Get a slice of the document.
    pub fn slice(&self, start: usize, end: usize) -> Result<String, String> {
        if let Some(source) = &self.file_source {
            let text = source.as_str();
            if start > end {
                return Err(format!("Start {} greater than end {}", start, end));
            }
            if end > text.len() {
                return Err(format!("End {} out of bounds", end));
            }
            Ok(text[start..end].to_string())
        } else {
            if start > end {
                return Err(format!("Start {} greater than end {}", start, end));
            }
            if end > self.rope.len_chars() {
                return Err(format!("End {} out of bounds", end));
            }
            Ok(self.rope.slice(start..end).to_string())
        }
    }

    /// Mark the document as saved (clears dirty flag).
    pub fn mark_saved(&mut self) {
        self.dirty = false;
    }

    /// Check if the document has unsaved changes.
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Get the document version (increments on each change).
    pub fn version(&self) -> u64 {
        self.version
    }

    /// Get the document's file path, if any.
    pub fn path(&self) -> Option<&std::path::Path> {
        self.path.as_deref()
    }

    /// Set the document's file path.
    pub fn set_path(&mut self, path: Option<String>) {
        self.path = path.map(PathBuf::from);
    }

    /// Get the large file mode.
    pub fn large_file_mode(&self) -> LargeFileMode {
        self.large_file_mode
    }

    /// Check if the document is considered large (for performance considerations).
    pub fn is_large(&self) -> bool {
        self.large_file_mode == LargeFileMode::Large
    }

    /// Check if the document is considered very large (read-only recommended).
    pub fn is_very_large(&self) -> bool {
        self.large_file_mode == LargeFileMode::VeryLarge
    }

    /// Get byte offset for character position
    pub fn char_to_byte(&self, char_idx: usize) -> usize {
        if let Some(source) = &self.file_source {
            let text = source.as_str();
            // Approximate: count bytes up to char_idx
            let mut byte_idx = 0usize;
            for (i, _) in text.char_indices() {
                if i >= char_idx {
                    break;
                }
                byte_idx = i;
            }
            byte_idx
        } else {
            self.rope.char_to_byte(char_idx)
        }
    }

    /// Get character offset for byte position
    pub fn byte_to_char(&self, byte_idx: usize) -> usize {
        if let Some(source) = &self.file_source {
            let text = source.as_str();
            // Count characters up to byte_idx
            let mut char_count = 0usize;
            for (i, _) in text.char_indices() {
                if i >= byte_idx {
                    break;
                }
                char_count += 1;
            }
            char_count
        } else {
            self.rope.byte_to_char(byte_idx)
        }
    }

    /// Get the file source, if any.
    pub fn file_source(&self) -> Option<&FileSource> {
        self.file_source.as_ref()
    }

    /// Ensure the rope is loaded from the file source (for editing operations).
    fn ensure_rope_loaded(&mut self) {
        if self.rope.len_chars() == 0 {
            if let Some(source) = &self.file_source {
                self.rope = Rope::from_str(source.as_str());
            }
        }
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}
