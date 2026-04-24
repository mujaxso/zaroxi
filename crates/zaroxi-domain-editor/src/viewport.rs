//! Simple viewport state for the minimal editor.

/// Viewport dimensions and visible‑line information.
#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    /// Line height in pixels (used for scroll calculations).
    pub line_height: f32,
    /// First visible line index.
    pub first_visible_line: usize,
    /// Number of visible lines.
    pub visible_line_count: usize,
    /// Total width of the viewport (in logical pixels).
    pub width: f32,
    /// Total height of the viewport (in logical pixels).
    pub height: f32,
}

impl Viewport {
    /// Create a viewport with default values (800x600, 22‑pixel line height).
    pub fn new() -> Self {
        Self {
            line_height: 22.0,
            first_visible_line: 0,
            visible_line_count: 0,
            width: 800.0,
            height: 600.0,
        }
    }

    /// Create a viewport with explicit dimensions. The line height remains 22 px.
    pub fn with_dimensions(width: f32, height: f32) -> Self {
        let line_height = 22.0;
        let visible_line_count = (height / line_height).ceil() as usize + 1;
        Self { line_height, first_visible_line: 0, visible_line_count, width, height }
    }

    /// Set the viewport dimensions and recompute the visible‑line count.
    pub fn set_dimensions(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
        self.visible_line_count = (height / self.line_height).ceil() as usize + 1;
    }

    /// Set the line height and recompute the visible‑line count.
    pub fn set_line_height(&mut self, line_height: f32) {
        self.line_height = line_height;
        self.visible_line_count = (self.height / line_height).ceil() as usize + 1;
    }

    /// Scroll to a specific line.
    pub fn scroll_to_line(&mut self, line: usize) {
        self.first_visible_line = line;
    }

    /// Scroll by a signed delta (positive = down, negative = up).
    pub fn scroll_by_lines(&mut self, delta: isize) {
        if delta > 0 {
            self.first_visible_line = self.first_visible_line.saturating_add(delta as usize);
        } else {
            self.first_visible_line = self.first_visible_line.saturating_sub((-delta) as usize);
        }
    }

    /// Compute the line index from a pixel offset.
    pub fn scroll_offset_to_line(&self, scroll_y: f32) -> usize {
        (scroll_y / self.line_height).floor() as usize
    }

    /// Compute the pixel offset for a given line index.
    pub fn line_to_scroll_offset(&self, line: usize) -> f32 {
        line as f32 * self.line_height
    }

    /// Return the `start..end` range of currently visible line indices.
    pub fn visible_line_range(&self) -> std::ops::Range<usize> {
        self.first_visible_line..(self.first_visible_line + self.visible_line_count)
    }
}

impl Default for Viewport {
    fn default() -> Self {
        Self::new()
    }
}
