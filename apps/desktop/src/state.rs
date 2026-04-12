use std::sync::{Arc, Mutex};
use std::env;
use workspace_model::state::WorkspaceState;
use core_types::workspace::DirectoryEntry;
use editor_core::EditorState;
use iced::widget::text_editor;
use iced;

use crate::theme::NeoteTheme;
use crate::explorer::state::ExplorerState;
use crate::settings::editor::EditorTypographySettings;
use syntax_core;

#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub path: String,
    pub size: u64,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimarySidebarView {
    Explorer,
    Search,
    SourceControl,
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuxiliaryView {
    AiAssistant,
    // Future: Debug, Terminal, etc.
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Activity {
    Primary(PrimarySidebarView),
    Auxiliary(AuxiliaryView),
}

impl Activity {
    pub fn explorer() -> Self {
        Activity::Primary(PrimarySidebarView::Explorer)
    }
    
    pub fn search() -> Self {
        Activity::Primary(PrimarySidebarView::Search)
    }
    
    pub fn source_control() -> Self {
        Activity::Primary(PrimarySidebarView::SourceControl)
    }
    
    pub fn settings() -> Self {
        Activity::Primary(PrimarySidebarView::Settings)
    }
    
    pub fn ai_assistant() -> Self {
        Activity::Auxiliary(AuxiliaryView::AiAssistant)
    }
    
    pub fn is_primary(&self) -> bool {
        matches!(self, Activity::Primary(_))
    }
    
    pub fn is_auxiliary(&self) -> bool {
        matches!(self, Activity::Auxiliary(_))
    }
    
    pub fn primary_view(&self) -> Option<PrimarySidebarView> {
        match self {
            Activity::Primary(view) => Some(*view),
            _ => None,
        }
    }
    
    pub fn auxiliary_view(&self) -> Option<AuxiliaryView> {
        match self {
            Activity::Auxiliary(view) => Some(*view),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WorkbenchLayoutState {
    // Primary sidebar state
    pub primary_sidebar_visible: bool,
    pub active_primary_view: PrimarySidebarView,
    
    // Auxiliary sidebar state (like AI Assistant)
    pub auxiliary_sidebar_visible: bool,
    pub active_auxiliary_view: Option<AuxiliaryView>,
    
    // Activity bar hover state
    pub hovered_activity: Option<Activity>,
}

impl Default for WorkbenchLayoutState {
    fn default() -> Self {
        Self {
            primary_sidebar_visible: true,
            active_primary_view: PrimarySidebarView::Explorer,
            auxiliary_sidebar_visible: true,
            active_auxiliary_view: Some(AuxiliaryView::AiAssistant),
            hovered_activity: None,
        }
    }
}

impl WorkbenchLayoutState {
    pub fn set_active_primary_view(&mut self, view: PrimarySidebarView) {
        self.active_primary_view = view;
        // For Settings, we want to hide the primary sidebar and show settings in main area
        // For other views, show the primary sidebar
        if view == PrimarySidebarView::Settings {
            self.primary_sidebar_visible = false;
        } else {
            self.primary_sidebar_visible = true;
        }
    }
    
    pub fn toggle_auxiliary_sidebar(&mut self) {
        self.auxiliary_sidebar_visible = !self.auxiliary_sidebar_visible;
    }
    
    pub fn set_auxiliary_view(&mut self, view: AuxiliaryView) {
        self.active_auxiliary_view = Some(view);
        self.auxiliary_sidebar_visible = true;
    }
    
    pub fn is_primary_view_active(&self, view: PrimarySidebarView) -> bool {
        // For Settings, it's active even if primary_sidebar_visible is false
        if view == PrimarySidebarView::Settings {
            self.active_primary_view == view
        } else {
            self.primary_sidebar_visible && self.active_primary_view == view
        }
    }
    
    pub fn is_auxiliary_view_active(&self, view: AuxiliaryView) -> bool {
        self.auxiliary_sidebar_visible && self.active_auxiliary_view == Some(view)
    }
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
    pub explorer_state: ExplorerState,
    pub active_file_path: Option<String>,
    pub editor_state: Option<EditorState>,
    pub is_dirty: bool,
    pub status_message: String,
    pub error_message: Option<String>,
    pub workspace_state: Arc<Mutex<WorkspaceState>>,
    pub workbench_layout: WorkbenchLayoutState,
    pub prompt_input: String,
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
    // Editor typography settings
    pub editor_typography: EditorTypographySettings,
    // Syntax manager for Tree-sitter integration
    pub syntax_manager: Arc<Mutex<syntax_core::SyntaxManager>>,
    // Diagnostic: number of highlight spans produced for the active document
    pub syntax_highlight_span_count: usize,
    // Actual highlight spans for the active document (for UI rendering)
    pub syntax_highlight_spans: Vec<syntax_core::HighlightSpan>,
    // Per‑line cache of highlight ranges for the real editor widget
    pub syntax_highlight_cache: Vec<Vec<(std::ops::Range<usize>, iced::Color)>>,
}

impl App {
    pub fn new() -> (Self, iced::Command<crate::message::Message>) {
        // Ensure NEOTE_RUNTIME is set to find Tree‑sitter resources
        if env::var("NEOTE_RUNTIME").is_err() {
            // Try to locate runtime relative to the executable
            if let Ok(exe_path) = env::current_exe() {
                if let Some(exe_dir) = exe_path.parent() {
                    let runtime_candidate = exe_dir.join("../runtime/treesitter");
                    if runtime_candidate.exists() {
                        unsafe {
                            env::set_var("NEOTE_RUNTIME", runtime_candidate.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }

        (
            App {
                workspace_path: String::new(),
                file_entries: Vec::new(),
                explorer_state: ExplorerState::new(),
                active_file_path: None,
                editor_state: None,
                is_dirty: false,
                status_message: "Ready".to_string(),
                error_message: None,
                workspace_state: Arc::new(Mutex::new(WorkspaceState::new(""))),
                workbench_layout: WorkbenchLayoutState::default(),
                prompt_input: String::new(),
                text_editor: text_editor::Content::new(),
                is_file_too_large_for_editor: false,
                file_loading_state: FileLoadingState::Idle,
                is_file_read_only: false,
                theme: NeoteTheme::Dark, // Always use premium dark theme
                window_width: 1200,
                window_height: 800,
                layout_mode: LayoutMode::Wide,
                editor_typography: EditorTypographySettings::default(),
                syntax_manager: Arc::new(Mutex::new(syntax_core::SyntaxManager::new())),
                syntax_highlight_span_count: 0,
                syntax_highlight_spans: Vec::new(),
                syntax_highlight_cache: Vec::new(),
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
