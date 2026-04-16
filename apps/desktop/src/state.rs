use std::sync::{Arc, Mutex};
use std::env;
use workspace_model::state::WorkspaceState;
use core_types::workspace::DirectoryEntry;
use editor_core::EditorState;
use iced::widget::text_editor;
use iced;

use crate::theme::ZaroxiTheme;
use crate::explorer::state::ExplorerState;
use crate::settings::editor::EditorTypographySettings;
use syntax_core;

#[derive(Debug, Clone)]
pub struct EditorTab {
    pub id: usize,
    pub file_path: String,
    pub display_name: String,
    pub is_dirty: bool,
    pub is_active: bool,
    pub is_pinned: bool,
}

impl EditorTab {
    pub fn new(id: usize, file_path: String) -> Self {
        println!("DEBUG EditorTab::new: id = {}, file_path = {}", id, file_path);
        let file_name = std::path::Path::new(&file_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(&file_path)
            .to_string();
        
        // For now, use just the file name
        // In a more advanced implementation, we could add parent directory
        // if there are duplicate file names in different directories
        let display_name = file_name.clone();
        
        println!("DEBUG EditorTab::new: display_name = {}", display_name);
        
        Self {
            id,
            file_path,
            display_name,
            is_dirty: false,
            is_active: false,
            is_pinned: false,
        }
    }
    
    pub fn set_dirty(&mut self, dirty: bool) {
        self.is_dirty = dirty;
    }
    
    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }
}

#[derive(Debug, Clone)]
pub struct TabManager {
    pub tabs: Vec<EditorTab>,
    pub next_tab_id: usize,
    pub active_tab_id: Option<usize>,
}

impl TabManager {
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            next_tab_id: 0,
            active_tab_id: None,
        }
    }
    
    pub fn open_or_activate_tab(&mut self, file_path: String) -> usize {
        println!("DEBUG TabManager::open_or_activate_tab: file_path = {}", file_path);
        println!("DEBUG TabManager::open_or_activate_tab: current tabs count = {}", self.tabs.len());
        
        // Check if tab already exists for this file
        for tab in &self.tabs {
            if tab.file_path == file_path {
                println!("DEBUG TabManager::open_or_activate_tab: tab already exists, id = {}", tab.id);
                // Get the tab id before calling activate_tab which borrows self mutably
                let tab_id = tab.id;
                // Activate this tab
                self.activate_tab(tab_id);
                return tab_id;
            }
        }
        
        println!("DEBUG TabManager::open_or_activate_tab: creating new tab");
        // Create new tab
        let tab_id = self.next_tab_id;
        self.next_tab_id += 1;
        
        let mut new_tab = EditorTab::new(tab_id, file_path.clone());
        new_tab.set_active(true);
        
        println!("DEBUG TabManager::open_or_activate_tab: new tab id = {}, path = {}", tab_id, file_path);
        
        // Deactivate all other tabs
        for tab in &mut self.tabs {
            tab.set_active(false);
        }
        
        self.tabs.push(new_tab);
        self.active_tab_id = Some(tab_id);
        
        println!("DEBUG TabManager::open_or_activate_tab: after push, tabs count = {}", self.tabs.len());
        
        // Update display names to handle duplicates
        self.update_tab_display_names();
        
        tab_id
    }
    
    pub fn activate_tab(&mut self, tab_id: usize) -> bool {
        if self.tabs.iter().any(|t| t.id == tab_id) {
            // Deactivate all tabs
            for tab in &mut self.tabs {
                tab.set_active(tab.id == tab_id);
            }
            self.active_tab_id = Some(tab_id);
            true
        } else {
            false
        }
    }
    
    pub fn close_tab(&mut self, tab_id: usize) -> Option<String> {
        let position = self.tabs.iter().position(|t| t.id == tab_id)?;
        let closed_tab = self.tabs.remove(position);
        
        // If we closed the active tab, activate another one
        if self.active_tab_id == Some(tab_id) {
            self.active_tab_id = if !self.tabs.is_empty() {
                // Try to activate the tab to the right, or the last tab
                let new_active_id = if position < self.tabs.len() {
                    self.tabs[position].id
                } else if !self.tabs.is_empty() {
                    self.tabs[self.tabs.len() - 1].id
                } else {
                    return Some(closed_tab.file_path);
                };
                self.activate_tab(new_active_id);
                Some(new_active_id)
            } else {
                None
            };
        }
        
        // Update display names in case duplicates are resolved
        self.update_tab_display_names();
        
        Some(closed_tab.file_path)
    }
    
    pub fn get_active_tab(&self) -> Option<&EditorTab> {
        self.active_tab_id.and_then(|id| self.tabs.iter().find(|t| t.id == id))
    }
    
    pub fn get_active_file_path(&self) -> Option<String> {
        self.get_active_tab().map(|t| t.file_path.clone())
    }
    
    pub fn set_tab_dirty(&mut self, tab_id: usize, dirty: bool) -> bool {
        if let Some(tab) = self.tabs.iter_mut().find(|t| t.id == tab_id) {
            tab.set_dirty(dirty);
            true
        } else {
            false
        }
    }
    
    pub fn find_tab_by_path(&self, file_path: &str) -> Option<&EditorTab> {
        self.tabs.iter().find(|t| t.file_path == file_path)
    }
    
    pub fn has_tab_for_path(&self, file_path: &str) -> bool {
        self.find_tab_by_path(file_path).is_some()
    }
    
    fn update_tab_display_names(&mut self) {
        // Count occurrences of each file name
        use std::collections::HashMap;
        let mut name_counts: HashMap<String, usize> = HashMap::new();
        
        for tab in &self.tabs {
            let name = tab.display_name.clone();
            *name_counts.entry(name).or_insert(0) += 1;
        }
        
        // If any file name appears more than once, add parent directory
        for tab in &mut self.tabs {
            if let Some(&count) = name_counts.get(&tab.display_name) {
                if count > 1 {
                    // Add parent directory to disambiguate
                    if let Some(parent) = std::path::Path::new(&tab.file_path).parent() {
                        if let Some(parent_name) = parent.file_name() {
                            if let Some(parent_str) = parent_name.to_str() {
                                tab.display_name = format!("{}/{}", parent_str, tab.display_name);
                            }
                        }
                    }
                }
            }
        }
    }
}

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
    // Theme preference
    pub theme_preference: ZaroxiTheme,
    // Current resolved theme (based on preference and system)
    pub current_theme: ZaroxiTheme,
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
    // Version counter to force UI updates when cache changes
    pub syntax_cache_version: u32,
    // Whether syntax highlighting should be enabled for the current file
    pub syntax_highlighting_enabled: bool,
    // File cache to speed up reopening files (max 10 files to avoid memory issues)
    pub file_cache: std::collections::HashMap<String, (String, editor_core::Document)>,
    // Tab management for editor
    pub tab_manager: TabManager,
}

impl App {
    pub fn new() -> (Self, iced::Command<crate::message::Message>) {
        // Ensure ZAROXI_STUDIO_RUNTIME is set to find Tree‑sitter resources
        if env::var(crate::brand::RUNTIME_ENV_VAR).is_err() {
            // Try to locate runtime relative to the executable
            if let Ok(exe_path) = env::current_exe() {
                if let Some(exe_dir) = exe_path.parent() {
                    let runtime_candidate = exe_dir.join("../runtime/treesitter");
                    if runtime_candidate.exists() {
                        unsafe {
                            env::set_var(crate::brand::RUNTIME_ENV_VAR, runtime_candidate.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }

        // Load settings from disk
        let (editor_typography, theme_preference) = match crate::settings::persistence::load_settings() {
            Ok(settings) => settings,
            Err(e) => {
                eprintln!("Failed to load settings: {}", e);
                (EditorTypographySettings::default(), ZaroxiTheme::System)
            }
        };

        let mut app = App {
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
            theme_preference,
            current_theme: ZaroxiTheme::Dark, // Will be updated by update_current_theme
            window_width: 1200,
            window_height: 800,
            layout_mode: LayoutMode::Wide,
            editor_typography,
            syntax_manager: Arc::new(Mutex::new(syntax_core::SyntaxManager::new())),
            syntax_highlight_span_count: 0,
            syntax_highlight_spans: Vec::new(),
            syntax_highlight_cache: Vec::new(),
            syntax_cache_version: 0,
            syntax_highlighting_enabled: true,
            file_cache: std::collections::HashMap::new(),
            tab_manager: TabManager::new(),
        };
        
        // Update current theme based on preference
        app.update_current_theme();
        
        (
            app,
            iced::Command::none(),
        )
    }

    pub fn subscription(&self) -> iced::Subscription<crate::message::Message> {
        use crate::message::Message;
        
        iced::Subscription::batch(vec![
            iced::keyboard::on_key_press(|key, modifiers| {
                Some(Message::KeyPressed(key, modifiers))
            }),
            iced::event::listen_with(|event, _status| {
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
    
    /// Update the current theme based on preference and system detection
    pub fn update_current_theme(&mut self) {
        self.current_theme = match self.theme_preference {
            ZaroxiTheme::System => {
                // For now, default to Dark for System mode
                // In a real implementation, we'd detect system preference
                ZaroxiTheme::Dark
            }
            ZaroxiTheme::Light => ZaroxiTheme::Light,
            ZaroxiTheme::Dark => ZaroxiTheme::Dark,
        };
    }
}
