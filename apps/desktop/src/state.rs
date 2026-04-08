use std::sync::{Arc, Mutex};
use workspace_model::state::WorkspaceState;
use core_types::workspace::DirectoryEntry;
use editor_buffer::buffer::TextBuffer;
use iced::widget::text_editor;
use iced;
use iced_futures::SubscriptionExt;

use crate::theme::NeoteTheme;

#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub path: String,
    pub size: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Activity {
    Explorer,
    Search,
    Ai,
    SourceControl,
    Settings,
}

// File loading states
#[derive(Debug, Clone)]
pub enum FileLoadingState {
    Idle,
    LoadingMetadata { path: String },
    LoadingContent { path: String, size: u64 },
    LargeFileWarning { path: String, size: u64 },
    VeryLargeFileWarning { path: String, size: u64 },
    ReadOnlyPreview { path: String, size: u64 },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayoutMode {
    Wide,
    Medium,
    Narrow,
}

pub struct App {
    pub workspace_path: String,
    pub file_entries: Vec<DirectoryEntry>,
    pub active_file_path: Option<String>,
    pub editor_buffer: Option<TextBuffer>,
    pub is_dirty: bool,
    pub status_message: String,
    pub error_message: Option<String>,
    pub workspace_state: Arc<Mutex<WorkspaceState>>,
    pub active_activity: Activity,
    pub ai_panel_visible: bool,
    pub prompt_input: String,
    pub expanded_directories: std::collections::HashSet<String>,
    pub text_editor: text_editor::Content,
    // Track if the current file is too large for the text editor
    pub is_file_too_large_for_editor: bool,
    // New: File loading state
    pub file_loading_state: FileLoadingState,
    // Track if current file was loaded in read-only mode
    pub is_file_read_only: bool,
    // Theme
    pub theme: NeoteTheme,
    // Window dimensions for responsive layout
    pub window_width: u32,
    pub window_height: u32,
    pub layout_mode: LayoutMode,
}

impl App {
    pub fn new() -> (Self, iced::Command<crate::message::Message>) {
        (
            App {
                workspace_path: String::new(),
                file_entries: Vec::new(),
                active_file_path: None,
                editor_buffer: None,
                is_dirty: false,
                status_message: "Ready".to_string(),
                error_message: None,
                workspace_state: Arc::new(Mutex::new(WorkspaceState::new(""))),
                active_activity: Activity::Explorer,
                ai_panel_visible: true,
                prompt_input: String::new(),
                expanded_directories: std::collections::HashSet::new(),
                text_editor: text_editor::Content::new(),
                is_file_too_large_for_editor: false,
                file_loading_state: FileLoadingState::Idle,
                is_file_read_only: false,
                theme: NeoteTheme::Dark, // Always use premium dark theme
                window_width: 1200,
                window_height: 800,
                layout_mode: LayoutMode::Wide,
            },
            iced::Command::none(),
        )
    }

    pub fn subscription(&self) -> iced::Subscription<crate::message::Message> {
        use crate::message::Message;
        
        iced::Subscription::batch(vec![
            iced::keyboard::on_key_press(|key, modifiers| {
                Some(Message::KeyPressed(key, modifiers))
            }),
            iced::event::listen().filter_map(|event| {
                match event {
                    iced::Event::Window(_id, iced::window::Event::Resized { width, height }) => {
                        Some(Message::WindowResized(width, height))
                    }
                    _ => None,
                }
            }),
        ])
    }

    pub fn update_layout_mode(&mut self) {
        self.layout_mode = if self.window_width >= 1200 {
            LayoutMode::Wide
        } else if self.window_width >= 800 {
            LayoutMode::Medium
        } else {
            LayoutMode::Narrow
        };
    }
}
