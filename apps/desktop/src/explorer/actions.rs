use std::path::PathBuf;
use core_types::workspace::DirectoryEntry;

#[derive(Debug, Clone)]
pub enum ExplorerMessage {
    ToggleDirectory(PathBuf),
    SelectFile(PathBuf),
    Refresh,
    SetWorkspaceRoot(PathBuf),
    SetFileTree(Vec<DirectoryEntry>),
    // New file/folder creation
    CreateFileRequested,
    CreateFolderRequested,
    // Inline editing
    RenameRequested(PathBuf),
    DeleteRequested(PathBuf),
    // Inline edit state changes
    InlineEditNameChanged(String),
    InlineEditConfirmed,
    InlineEditCancelled,
    // Context menu actions
    ShowContextMenu(PathBuf),
    HideContextMenu,
}
