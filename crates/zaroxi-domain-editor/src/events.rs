//! Editor events for notifications.

/// Events emitted by the editor.
#[derive(Debug, Clone)]
pub enum EditorEvent {
    DocumentChanged,
    CursorMoved,
    ViewportChanged,
}
