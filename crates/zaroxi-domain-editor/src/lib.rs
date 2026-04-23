//! Minimal editor core engine for Zaroxi Studio.
//!
//! This crate provides the domain model and editing operations for the Zaroxi editor,
//! separate from UI concerns. It is intentionally minimal to guarantee correctness.

pub mod document;
pub mod editor;
pub mod cursor;
pub mod viewport;
pub mod events;
pub mod selection;

// Re-export main types for convenience
pub use document::Document;
pub use document::LargeFileMode;
pub use editor::EditorState;
pub use cursor::{Cursor, CursorMovement};
pub use viewport::Viewport;
pub use events::EditorEvent;
pub use selection::Selection;
