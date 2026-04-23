//! Viewport state for visible editor area.

/// Viewport dimensions and state.
#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    width: f32,
    height: f32,
    /// Line height in pixels (used for scroll calculations).
    line_height: f32,
    /// First visible line index.
    first_visible_line: usize,
    /// Number of visible lines.
    visible_line_count: usize,
}

impl Viewport {
    /// Create a new viewport with default dimensions.
    pub fn new() -> Self {
        Self {
            width: 800.0,
            height: 600.0,
            line_height: 22.0,
            first_visible_line: 0,
            visible_line_count: 0,
        }
    }

    /// Create a viewport with specific dimensions.
    pub fn with_dimensions(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            line_height: 22.0,
            first_visible_line: 0,
            visible_line_count: (height / 22.0).ceil() as usize + 1,
        }
    }

    /// Get the viewport width.
    pub fn width(&self) -> f32 {
        self.width
    }

    /// Get the viewport height.
    pub fn height(&self) -> f32 {
        self.height
    }

    /// Set the viewport dimensions.
    pub fn set_dimensions(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
        self.visible_line_count = (height / self.line_height).ceil() as usize + 1;
    }

    /// Get the line height.
    pub fn line_height(&self) -> f32 {
        self.line_height
    }

    /// Set the line height.
    pub fn set_line_height(&mut self, line_height: f32) {
        self.line_height = line_height;
        self.visible_line_count = (self.height / line_height).ceil() as usize + 1;
    }

    /// Get the first visible line index.
    pub fn first_visible_line(&self) -> usize {
        self.first_visible_line
    }

    /// Set the first visible line index.
    pub fn set_first_visible_line(&mut self, line: usize) {
        self.first_visible_line = line;
    }

    /// Get the number of visible lines.
    pub fn visible_line_count(&self) -> usize {
        self.visible_line_count
    }

    /// Get the range of visible lines (inclusive start, exclusive end).
    pub fn visible_line_range(&self) -> std::ops::Range<usize> {
        self.first_visible_line..(self.first_visible_line + self.visible_line_count)
    }

    /// Scroll to a specific line.
    pub fn scroll_to_line(&mut self, line: usize) {
        self.first_visible_line = line;
    }

    /// Scroll by a number of lines (positive = down, negative = up).
    pub fn scroll_by_lines(&mut self, delta: isize) {
        if delta > 0 {
            self.first_visible_line = self.first_visible_line.saturating_add(delta as usize);
        } else {
            self.first_visible_line = self.first_visible_line.saturating_sub((-delta) as usize);
        }
    }

    /// Convert a scroll offset in pixels to a line index.
    pub fn scroll_offset_to_line(&self, scroll_y: f32) -> usize {
        (scroll_y / self.line_height).floor() as usize
    }

    /// Convert a line index to a scroll offset in pixels.
    pub fn line_to_scroll_offset(&self, line: usize) -> f32 {
        line as f32 * self.line_height
    }
}

impl Default for Viewport {
    fn default() -> Self {
        Self::new()
    }
}
