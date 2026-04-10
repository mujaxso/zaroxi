use iced::widget::text_editor;
use core_types::workspace::DirectoryEntry;
use editor_buffer::buffer::TextBuffer;
use crate::state::{Activity, FileMetadata};
use crate::explorer::actions::ExplorerMessage;
use crate::settings::editor::FontFamily;

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
    WindowResized(u32, u32),
    // Explorer messages
    Explorer(ExplorerMessage),
    // New messages for explorer actions
    ExplorerHoverChanged(Option<std::path::PathBuf>),
    // Activity bar hover state
    ActivityHovered(Option<Activity>),
    // Font loading messages
    FontLoaded,
    FontLoadFailed,
    // Editor typography settings messages
    FontFamilyChanged(FontFamily),
    FontSizeChanged(u16),
    LineHeightChanged(f32),
    LetterSpacingChanged(f32),
    LigaturesToggled(bool),
    // Icon settings
    IconModeChanged(crate::settings::editor::IconMode),
    PreferNerdFontsToggled(bool),
    // Zoom controls
    ZoomIn,
    ZoomOut,
    ResetZoom,
    // Reset all typography settings to defaults
    ResetTypographyToDefaults,
    // Save typography settings
    SaveTypographySettings,
    // Load typography settings
    TypographySettingsLoaded(Result<crate::settings::editor::EditorTypographySettings, String>),
}
