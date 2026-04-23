//! Centralized file‑size and pathological‑structure thresholds for the editor.

/// File classification categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileClass {
    /// Normal‑sized file; full editor features available.
    Normal,
    /// Medium‑sized file; consider deferring expensive features.
    Medium,
    /// Large file (many bytes, many lines, or extremely long lines);
    /// may be read‑only and uses a simplified rendering path.
    Large,
}

impl FileClass {
    /// Whether editing operations should be blocked for safety.
    pub fn is_read_only(&self) -> bool {
        *self == Self::Large
    }
}

// ------------------------------------------------------------------
// Configurable thresholds (easily tunable without touching rendering)
// ------------------------------------------------------------------

/// Maximum bytes for a normal file (1 MiB).
pub const NORMAL_BYTE_LIMIT: u64 = 1_000_000;

/// Maximum bytes for a medium file (10 MiB).
pub const MEDIUM_BYTE_LIMIT: u64 = 10_000_000;

/// Maximum lines for a normal file.
pub const NORMAL_LINE_LIMIT: usize = 10_000;

/// Maximum lines for a medium file.
pub const MEDIUM_LINE_LIMIT: usize = 100_000;

/// Maximum characters in a single line for a normal file.
pub const NORMAL_MAX_LINE_LEN: usize = 2_000;

/// Maximum characters in a single line for a medium file.
pub const MEDIUM_MAX_LINE_LEN: usize = 20_000;

/// Classify a file based on byte size, line count, and maximum line length.
pub fn classify_file(
    byte_size: u64,
    line_count: usize,
    max_line_len: usize,
) -> FileClass {
    // Large threshold checks (any metric beyond medium makes it Large)
    if byte_size > MEDIUM_BYTE_LIMIT
        || line_count > MEDIUM_LINE_LIMIT
        || max_line_len > MEDIUM_MAX_LINE_LEN
    {
        return FileClass::Large;
    }

    // Medium threshold checks
    if byte_size > NORMAL_BYTE_LIMIT
        || line_count > NORMAL_LINE_LIMIT
        || max_line_len > NORMAL_MAX_LINE_LEN
    {
        return FileClass::Medium;
    }

    FileClass::Normal
}
