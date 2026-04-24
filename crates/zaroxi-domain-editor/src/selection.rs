//! Text selection model.

/// A text selection range (inclusive start, exclusive end).
#[derive(Debug, Clone, Copy)]
pub struct Selection {
    start: usize,
    end: usize,
}

impl Selection {
    /// Create a new selection.
    pub fn new(start: usize, end: usize) -> Self {
        let (start, end) = if start <= end { (start, end) } else { (end, start) };
        Self { start, end }
    }

    /// Get the start position (inclusive).
    pub fn start(&self) -> usize {
        self.start
    }

    /// Get the end position (exclusive).
    pub fn end(&self) -> usize {
        self.end
    }

    /// Check if the selection is empty (start == end).
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Get the length of the selection in characters.
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Check if a position is within the selection.
    pub fn contains(&self, position: usize) -> bool {
        position >= self.start && position < self.end
    }
}
