use iced::widget::text_editor;
use core_types::workspace::DirectoryEntry;
use editor_buffer::buffer::TextBuffer;
use crate::state::{Activity, FileMetadata};

#[derive(Debug, Clone)]
pub enum Message {
    WorkspacePathChanged(String),
    OpenWorkspace,
    WorkspaceLoaded(Result<Vec<DirectoryEntry>, String>),
    FileSelected(usize),
    FileSelectedByPath(String),
    // Metadata loaded (size, etc.)
    FileMetadataLoaded(Result<FileMetadata, String>),
    // File content loaded
    FileLoaded(Result<(String, String, TextBuffer), String>),
    // Confirm opening a large file
    ConfirmOpenLargeFile(String, u64),
    // Open in read-only mode
    OpenLargeFileReadOnly(String),
    EditorContentChanged(text_editor::Action),
    SaveFile,
    FileSaved(Result<(), String>),
    RefreshWorkspace,
    ToggleAiPanel,
    ActivitySelected(Activity),
    PromptInputChanged(String),
    SendPrompt,
    KeyPressed(iced::keyboard::Key, iced::keyboard::Modifiers),
    ToggleDirectory(String),
    ToggleCommandPalette,
}
