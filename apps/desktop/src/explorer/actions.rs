use std::path::PathBuf;
use core_types::workspace::DirectoryEntry;

#[derive(Debug, Clone)]
pub enum ExplorerMessage {
    ToggleDirectory(PathBuf),
    SelectFile(PathBuf),
    Refresh,
    SetWorkspaceRoot(PathBuf),
    SetFileTree(Vec<DirectoryEntry>),
}
