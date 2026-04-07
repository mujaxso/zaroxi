use std::sync::{Arc, Mutex};
use workspace_daemon::files;
use workspace_model::state::WorkspaceState;
use core_types::workspace::DirectoryEntry;
use editor_buffer::buffer::TextBuffer;
use iced::{Element, Command};
use iced::widget::text_editor;

#[derive(Debug, Clone)]
pub enum Message {
    WorkspacePathChanged(String),
    OpenWorkspace,
    WorkspaceLoaded(Result<Vec<DirectoryEntry>, String>),
    FileSelected(usize),
    FileLoaded(Result<(String, String), String>),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Activity {
    Explorer,
    Search,
    Ai,
    SourceControl,
    Settings,
}

pub struct App {
    workspace_path: String,
    file_entries: Vec<DirectoryEntry>,
    active_file_path: Option<String>,
    editor_content: String,
    editor_buffer: Option<TextBuffer>,
    is_dirty: bool,
    status_message: String,
    error_message: Option<String>,
    workspace_state: Arc<Mutex<WorkspaceState>>,
    active_activity: Activity,
    ai_panel_visible: bool,
    prompt_input: String,
    expanded_directories: std::collections::HashSet<String>,
    text_editor: text_editor::Content,
}

impl iced::Application for App {
    type Message = Message;
    type Theme = iced::Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            App {
                workspace_path: String::new(),
                file_entries: Vec::new(),
                active_file_path: None,
                editor_content: String::new(),
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
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Neote")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::WorkspacePathChanged(path) => {
                self.workspace_path = path;
                Command::none()
            }
            Message::OpenWorkspace => {
                if self.workspace_path.is_empty() {
                    self.status_message = "Please enter a workspace path".to_string();
                    return Command::none();
                }
                
                let path = self.workspace_path.clone();
                Command::perform(
                    async move {
                        match files::list_directory(&path) {
                            Ok(entries) => Message::WorkspaceLoaded(Ok(entries)),
                            Err(e) => Message::WorkspaceLoaded(Err(format!("Failed to open workspace: {}", e))),
                        }
                    },
                    |result| result,
                )
            }
            Message::WorkspaceLoaded(result) => {
                match result {
                    Ok(entries) => {
                        self.file_entries = entries;
                        self.status_message = format!("Workspace loaded: {} files", self.file_entries.len());
                        self.error_message = None;
                        
                        let mut state = self.workspace_state.lock().unwrap();
                        state.set_workspace_root(&self.workspace_path);
                        state.set_file_tree(self.file_entries.clone());
                    }
                    Err(e) => {
                        self.error_message = Some(e);
                        self.status_message = "Failed to load workspace".to_string();
                    }
                }
                Command::none()
            }
            Message::FileSelected(index) => {
                if index < self.file_entries.len() {
                    let entry = &self.file_entries[index];
                    if !entry.is_dir {
                        let path = entry.path.clone();
                        self.active_file_path = Some(path.clone());
                        self.status_message = format!("Loading {}... (this may take a moment for large files)", entry.name);
                        
                        Command::perform(
                            async move {
                                // Clone path for use inside the async block
                                let path_clone = path.clone();
                                // Read file in a blocking task to avoid freezing the UI
                                let content_result = tokio::task::spawn_blocking(move || {
                                    files::read_file(&path_clone)
                                }).await;
                                
                                match content_result {
                                    Ok(Ok(content)) => Message::FileLoaded(Ok((path, content))),
                                    Ok(Err(e)) => Message::FileLoaded(Err(format!("Failed to read file: {}", e))),
                                    Err(join_err) => Message::FileLoaded(Err(format!("Failed to join task: {}", join_err))),
                                }
                            },
                            |result| result,
                        )
                    } else {
                        Command::none()
                    }
                } else {
                    Command::none()
                }
            }
            Message::FileLoaded(result) => {
                match result {
                    Ok((path, content)) => {
                        // Check file size to determine handling mode
                        let file_size = content.len();
                        const WARNING_THRESHOLD: usize = 1_000_000; // 1MB
                        const READ_ONLY_THRESHOLD: usize = 10_000_000; // 10MB
                        
                        if file_size > READ_ONLY_THRESHOLD {
                            self.status_message = format!(
                                "Opened very large file ({} MB) in read-only mode. Editing disabled.",
                                file_size / 1_000_000
                            );
                            // Mark as read-only by not creating an editor buffer
                            // But we still want to show the content
                            self.text_editor = text_editor::Content::with_text(&content);
                            self.editor_content = content.clone();
                            self.editor_buffer = None;
                            self.is_dirty = false;
                            self.error_message = Some("File is very large - opened in read-only mode".to_string());
                            return Command::none();
                        } else if file_size > WARNING_THRESHOLD {
                            self.status_message = format!(
                                "Opened large file ({} MB). Performance may be affected.",
                                file_size / 1_000_000
                            );
                            self.error_message = Some("Large file - editing may be slow".to_string());
                        } else {
                            self.status_message = format!("Loaded: {} ({} bytes)", path, file_size);
                            self.error_message = None;
                        }
                        
                        // We'll update the UI immediately with a loading message
                        self.status_message = format!("Processing file content... ({} bytes)", file_size);
                        
                        // Update text editor content
                        // This may still block, but with the 1MB limit it should be manageable
                        self.text_editor = text_editor::Content::with_text(&content);
                        self.editor_content = content.clone();
                        
                        // Create buffer
                        let buffer = TextBuffer::new(content.clone());
                        self.editor_buffer = Some(buffer);
                        self.is_dirty = false;
                        
                        // Update workspace state quickly
                        {
                            let mut state = self.workspace_state.lock().unwrap();
                            state.open_buffer(&path, self.editor_content.clone());
                        }
                        
                        if file_size > 1_000_000 { // 1MB
                            self.status_message = format!("Loaded large file: {} ({} MB).", 
                                path, file_size / 1_000_000);
                        } else {
                            self.status_message = format!("Loaded: {} ({} bytes)", path, file_size);
                        }
                        self.error_message = None;
                    }
                    Err(e) => {
                        self.error_message = Some(e);
                        self.status_message = "Failed to load file".to_string();
                    }
                }
                Command::none()
            }
            Message::EditorContentChanged(action) => {
                // First, perform the action on the text editor
                self.text_editor.perform(action);
                
                // Try to update the buffer incrementally
                let mut updated_incrementally = false;
                let mut is_very_large_file = false;
                if let Some(buffer) = &mut self.editor_buffer {
                    // Check if the buffer is too large for incremental updates
                    if buffer.is_very_large() {
                        is_very_large_file = true;
                        // For very large files, we might want to mark them as read-only
                        // But for now, we'll just update with a full replacement
                        buffer.replace_all(self.text_editor.text());
                        self.is_dirty = buffer.is_dirty();
                    } else {
                        // Try to apply the action incrementally
                        match buffer.apply_iced_action(&action) {
                            Ok(_) => {
                                updated_incrementally = true;
                                self.is_dirty = buffer.is_dirty();
                            }
                            Err(_) => {
                                // Fall back to full replacement
                                buffer.replace_all(self.text_editor.text());
                                self.is_dirty = buffer.is_dirty();
                            }
                        }
                    }
                }
                
                // Update the editor content string only if needed
                if !updated_incrementally {
                    self.editor_content = self.text_editor.text().to_string();
                }
                
                // Set status message
                if is_very_large_file {
                    self.status_message = "Very large file - editing may be slow".to_string();
                } else {
                    self.status_message = if self.is_dirty {
                        "File has unsaved changes".to_string()
                    } else {
                        "All changes saved".to_string()
                    };
                }
                Command::none()
            }
            Message::SaveFile => {
                if let Some(path) = &self.active_file_path {
                    let content = self.editor_content.clone();
                    let path_clone = path.clone();
                    
                    Command::perform(
                        async move {
                            match files::write_file(&path_clone, &content) {
                                Ok(_) => Message::FileSaved(Ok(())),
                                Err(e) => Message::FileSaved(Err(format!("Failed to save file: {}", e))),
                            }
                        },
                        |result| result,
                    )
                } else {
                    self.status_message = "No file selected to save".to_string();
                    Command::none()
                }
            }
            Message::FileSaved(result) => {
                match result {
                    Ok(_) => {
                        if let Some(buffer) = &mut self.editor_buffer {
                            buffer.mark_saved();
                            self.is_dirty = buffer.is_dirty();
                        }
                        self.status_message = "File saved successfully".to_string();
                        self.error_message = None;
                    }
                    Err(e) => {
                        self.error_message = Some(e);
                        self.status_message = "Failed to save file".to_string();
                    }
                }
                Command::none()
            }
            Message::RefreshWorkspace => {
                if !self.workspace_path.is_empty() {
                    let path = self.workspace_path.clone();
                    Command::perform(
                        async move {
                            match files::list_directory(&path) {
                                Ok(entries) => Message::WorkspaceLoaded(Ok(entries)),
                                Err(e) => Message::WorkspaceLoaded(Err(format!("Failed to refresh workspace: {}", e))),
                            }
                        },
                        |result| result,
                    )
                } else {
                    Command::none()
                }
            }
            Message::ToggleAiPanel => {
                self.ai_panel_visible = !self.ai_panel_visible;
                // When toggling the AI panel, also set the active activity to AI
                self.active_activity = Activity::Ai;
                Command::none()
            }
            Message::ActivitySelected(activity) => {
                self.active_activity = activity;
                // If the selected activity is AI, ensure the AI panel is visible
                if activity == Activity::Ai {
                    self.ai_panel_visible = true;
                }
                Command::none()
            }
            Message::PromptInputChanged(text) => {
                self.prompt_input = text;
                Command::none()
            }
            Message::SendPrompt => {
                // Placeholder for AI prompt
                self.status_message = "AI feature coming soon".to_string();
                self.prompt_input.clear();
                Command::none()
            }
            Message::KeyPressed(key, modifiers) => {
                match key {
                    iced::keyboard::Key::Character(c) if c == "s" && modifiers.control() => {
                        // Ctrl+S to save
                        self.update(Message::SaveFile)
                    }
                    iced::keyboard::Key::Character(c) if c == "r" && modifiers.control() => {
                        // Ctrl+R to refresh workspace
                        self.update(Message::RefreshWorkspace)
                    }
                    iced::keyboard::Key::Character(c) if c == "o" && modifiers.control() => {
                        // Ctrl+O to open workspace
                        self.update(Message::OpenWorkspace)
                    }
                    iced::keyboard::Key::Character(c) if c == "p" && modifiers.control() && modifiers.shift() => {
                        // Ctrl+Shift+P for command palette
                        self.update(Message::ToggleCommandPalette)
                    }
                    _ => Command::none(),
                }
            }
            Message::ToggleCommandPalette => {
                // For now, just show a status message
                self.status_message = "Command palette (Ctrl+Shift+P) - coming soon".to_string();
                Command::none()
            }
            Message::ToggleDirectory(path) => {
                if self.expanded_directories.contains(&path) {
                    self.expanded_directories.remove(&path);
                } else {
                    self.expanded_directories.insert(path);
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        crate::ui::layout::ide_layout(
            &self.workspace_path,
            &self.file_entries,
            self.active_file_path.as_ref(),
            &self.editor_content,
            self.is_dirty,
            &self.status_message,
            self.error_message.as_ref(),
            self.active_activity,
            self.ai_panel_visible,
            &self.prompt_input,
            &self.expanded_directories,
            &self.text_editor,
            self.editor_buffer.as_ref(),
        )
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        iced::keyboard::on_key_press(|key, modifiers| {
            Some(Message::KeyPressed(key, modifiers))
        })
    }
}
