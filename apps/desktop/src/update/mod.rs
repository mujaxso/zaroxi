//! Modular update architecture for Zaroxi Studio
//! 
//! Each domain/feature has its own update module that handles its specific messages.
//! The top-level update function coordinates between domains and handles cross-cutting concerns.

pub mod workspace;
pub mod explorer;
pub mod editor;
pub mod settings;
pub mod workbench;
pub mod assistant;
pub mod dialog;


use crate::message::Message;
use crate::state::App;
use iced::Command;

/// Main update entry point that routes messages to domain-specific handlers
pub fn update(app: &mut App, message: Message) -> Command<Message> {
    match message {
        // Workspace and file operations
        Message::OpenWorkspace
        | Message::WorkspaceLoaded(_)
        | Message::WorkspacePathChanged(_)
        | Message::RefreshWorkspace
        | Message::FileSelected(_)
        | Message::FileSelectedByPath(_)
        | Message::FileMetadataLoaded(_)
        | Message::ConfirmOpenLargeFile(_, _)
        | Message::OpenLargeFileReadOnly(_)
        | Message::FileLoaded(_)
        | Message::SaveFile
        | Message::FileSaved(_)
        | Message::SubmitManualWorkspacePath(_)
        | Message::WorkspaceDialogCancelled => {
            workspace::update(app, message)
        }
        
        // Explorer operations
        Message::Explorer(_)
        | Message::ExplorerHoverChanged(_)
        | Message::ToggleDirectory(_) => {
            explorer::update(app, message)
        }
        
        // Editor operations
        Message::EditorContentChanged(_)
        | Message::EditorInsertText(_)
        | Message::EditorDeleteBackward
        | Message::EditorDeleteForward
        | Message::EditorMoveCursor(_)
        | Message::EditorSetDocument(_)
        | Message::EditorUpdateState(_) => {
            editor::update(app, message)
        }
        
        // Settings and typography
        Message::ThemeChanged(_)
        | Message::FontFamilyChanged(_)
        | Message::FontSizeChanged(_)
        | Message::LineHeightChanged(_)
        | Message::LetterSpacingChanged(_)
        | Message::LigaturesToggled(_)
        | Message::IconModeChanged(_)
        | Message::PreferNerdFontsToggled(_)
        | Message::ZoomIn
        | Message::ZoomOut
        | Message::ResetZoom
        | Message::ResetTypographyToDefaults
        | Message::SaveTypographySettings
        | Message::TypographySettingsLoaded(_)
        | Message::FontLoaded
        | Message::FontLoadFailed => {
            settings::update(app, message)
        }
        
        // Workbench and activity
        Message::ToggleAiPanel
        | Message::ActivitySelected(_)
        | Message::ActivityHovered(_)
        | Message::WindowResized(_, _) => {
            workbench::update(app, message)
        }
        
        // Assistant operations
        Message::PromptInputChanged(_)
        | Message::SendPrompt => {
            assistant::update(app, message)
        }
        
        // Key presses and UI state
        Message::KeyPressed(_, _)
        | Message::ToggleCommandPalette => {
            // Handle key presses that may affect multiple domains
            handle_ui_actions(app, message)
        }
    }
}

/// Handle UI-level actions like key presses that may affect multiple domains
fn handle_ui_actions(app: &mut App, message: Message) -> Command<Message> {
    match message {
        Message::KeyPressed(key, modifiers) => {
            // Handle global shortcuts
            match key {
                iced::keyboard::Key::Character(c) if c == "s" && modifiers.control() => {
                    // Ctrl+S to save - delegate to workspace
                    workspace::update(app, Message::SaveFile)
                }
                iced::keyboard::Key::Character(c) if c == "r" && modifiers.control() => {
                    // Ctrl+R to refresh workspace
                    workspace::update(app, Message::RefreshWorkspace)
                }
                iced::keyboard::Key::Character(c) if c == "o" && modifiers.control() => {
                    // Ctrl+O to open workspace
                    workspace::update(app, Message::OpenWorkspace)
                }
                iced::keyboard::Key::Character(c) if c == "p" && modifiers.control() && modifiers.shift() => {
                    // Ctrl+Shift+P for command palette
                    app.status_message = "Command palette (Ctrl+Shift+P) - coming soon".to_string();
                    Command::none()
                }
                iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape) => {
                    // Escape to cancel inline editing - delegate to explorer
                    explorer::update(app, Message::KeyPressed(key, modifiers))
                }
                iced::keyboard::Key::Character(c) if c == "+" && modifiers.control() => {
                    // Ctrl++ to zoom in
                    settings::update(app, Message::ZoomIn)
                }
                iced::keyboard::Key::Character(c) if c == "-" && modifiers.control() => {
                    // Ctrl+- to zoom out
                    settings::update(app, Message::ZoomOut)
                }
                iced::keyboard::Key::Character(c) if c == "0" && modifiers.control() => {
                    // Ctrl+0 to reset zoom
                    settings::update(app, Message::ResetZoom)
                }
                // Arrow keys for cursor movement
                iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowLeft)
                | iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowRight)
                | iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowUp)
                | iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowDown) => {
                    editor::update(app, Message::KeyPressed(key, modifiers))
                }
                _ => Command::none(),
            }
        }
        Message::ToggleCommandPalette => {
            app.status_message = "Command palette (Ctrl+Shift+P) - coming soon".to_string();
            Command::none()
        }
        _ => Command::none(),
    }
}
