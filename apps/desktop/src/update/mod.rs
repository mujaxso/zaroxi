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
        
        // Tab management messages
        Message::ActivateTab(tab_id) => {
            if app.tab_manager.activate_tab(tab_id) {
                // Update active file path
                if let Some(tab) = app.tab_manager.get_active_tab() {
                    let path = tab.file_path.clone();
                    app.active_file_path = Some(path.clone());
                    
                    // Load buffer from cache
                    if let Some(buffer) = app.editor_buffers.get(&path) {
                        // Update syntax highlighting state
                        app.syntax_highlight_spans = buffer.syntax_highlight_spans.clone();
                        app.syntax_highlight_cache = buffer.syntax_highlight_cache.clone();
                        app.syntax_cache_version = buffer.syntax_cache_version;
                        app.syntax_highlight_span_count = buffer.syntax_highlight_span_count;
                        
                        // Set the editor state
                        app.editor_state = Some(editor_core::EditorState::from_document(buffer.document.clone()));
                        app.is_dirty = buffer.is_dirty;
                        app.is_file_read_only = false;
                        app.is_file_too_large_for_editor = false;
                        
                        // Update status
                        app.status_message = format!("Switched to {}", tab.display_name);
                    }
                }
            }
            Command::none()
        }
        Message::CloseTab(tab_id) => {
            if let Some(closed_path) = app.tab_manager.close_tab(tab_id) {
                // If this was the last tab, clear the editor state
                if app.tab_manager.tabs.is_empty() {
                    app.active_file_path = None;
                    app.editor_state = None;
                    app.text_editor = iced::widget::text_editor::Content::new();
                    app.is_dirty = false;
                    app.status_message = "All tabs closed".to_string();
                } else {
                    // Activate the new active tab
                    if let Some(tab) = app.tab_manager.get_active_tab() {
                        let path = tab.file_path.clone();
                        app.active_file_path = Some(path.clone());
                        
                        // Load buffer from cache
                        if let Some(buffer) = app.editor_buffers.get(&path) {
                            // Update syntax highlighting state
                            app.syntax_highlight_spans = buffer.syntax_highlight_spans.clone();
                            app.syntax_highlight_cache = buffer.syntax_highlight_cache.clone();
                            app.syntax_cache_version = buffer.syntax_cache_version;
                            app.syntax_highlight_span_count = buffer.syntax_highlight_span_count;
                            
                            // Set the editor state
                            app.editor_state = Some(editor_core::EditorState::from_document(buffer.document.clone()));
                            app.is_dirty = buffer.is_dirty;
                            app.is_file_read_only = false;
                            app.is_file_too_large_for_editor = false;
                            
                            // Update status
                            app.status_message = format!("Switched to {}", tab.display_name);
                        }
                    }
                }
            }
            Command::none()
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
