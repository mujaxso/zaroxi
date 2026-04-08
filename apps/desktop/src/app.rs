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
    FileLoaded(Result<(String, String, TextBuffer), String>),
    TextEditorContentCreated(String), // Just the path
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

/// Create text editor content in a background task to avoid blocking the UI
async fn create_text_editor_content(path: String, _content: String) -> String {
    // This runs in a background thread
    // We don't actually create text_editor::Content here because it's not Send
    // Instead, we just process the content to keep the UI responsive
    // The actual content creation will happen on the main thread
    // But we can still yield control to prevent blocking
    tokio::task::yield_now().await;
    path
}

// Helper to extract edit information from text editor action
fn extract_edit_info(action: &text_editor::Action) -> Option<EditInfo> {
    match action {
        text_editor::Action::Edit(edit) => {
            // The Edit struct has a field `action` of type EditAction
            // We can access it directly
            // Use debug formatting to extract information
            let debug_str = format!("{:?}", edit);
            // The debug format looks like: "Edit { action: InsertText { char_idx: 5, text: "hello" }, .. }"
            // or "Edit { action: DeleteRange { start: 5, end: 10 }, .. }"
            if debug_str.contains("InsertText") {
                if let Some((char_idx, text)) = parse_insert_text(&debug_str) {
                    return Some(EditInfo::Insert { char_idx, text });
                }
            } else if debug_str.contains("DeleteRange") {
                if let Some((start, end)) = parse_delete_range(&debug_str) {
                    return Some(EditInfo::Delete { start, end });
                }
            }
        }
        _ => {}
    }
    None
}

// Parse InsertText from debug string
fn parse_insert_text(debug_str: &str) -> Option<(usize, String)> {
    // Example: "Edit { action: InsertText { char_idx: 5, text: \"hello\" }, .. }"
    // Find char_idx
    let char_idx_str = if let Some(idx) = debug_str.find("char_idx: ") {
        let rest = &debug_str[idx + 10..];
        if let Some(end) = rest.find(',') {
            rest[..end].trim().to_string()
        } else {
            String::new()
        }
    } else {
        String::new()
    };
    
    // Find text
    let text_str = if let Some(idx) = debug_str.find("text: \"") {
        let rest = &debug_str[idx + 7..];
        if let Some(end) = rest.find('\"') {
            rest[..end].to_string()
        } else {
            String::new()
        }
    } else {
        String::new()
    };
    
    if char_idx_str.is_empty() || text_str.is_empty() {
        return None;
    }
    
    let char_idx = char_idx_str.parse::<usize>().ok()?;
    Some((char_idx, text_str))
}

// Parse DeleteRange from debug string
fn parse_delete_range(debug_str: &str) -> Option<(usize, usize)> {
    // Example: "Edit { action: DeleteRange { start: 5, end: 10 }, .. }"
    // Find start
    let start_str = if let Some(idx) = debug_str.find("start: ") {
        let rest = &debug_str[idx + 7..];
        if let Some(end) = rest.find(',') {
            rest[..end].trim().to_string()
        } else {
            String::new()
        }
    } else {
        String::new()
    };
    
    // Find end
    let end_str = if let Some(idx) = debug_str.find("end: ") {
        let rest = &debug_str[idx + 5..];
        // Look for the closing brace of DeleteRange
        if let Some(end) = rest.find('}') {
            rest[..end].trim().to_string()
        } else {
            String::new()
        }
    } else {
        String::new()
    };
    
    if start_str.is_empty() || end_str.is_empty() {
        return None;
    }
    
    let start = start_str.parse::<usize>().ok()?;
    let end = end_str.parse::<usize>().ok()?;
    Some((start, end))
}

// Edit information enum
enum EditInfo {
    Insert { char_idx: usize, text: String },
    Delete { start: usize, end: usize },
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
    // Track if the current file is too large for the text editor
    is_file_too_large_for_editor: bool,
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
                        // Clear current editor content to show loading state
                        self.text_editor = text_editor::Content::new();
                        self.editor_buffer = None;
                        self.status_message = format!("Loading {}...", entry.name);
                        self.error_message = None;
                        
                        Command::perform(
                            async move {
                                // Clone path for use inside the async block
                                let path_clone = path.clone();
                                // Read file, create buffer, and text editor content in a blocking task
                                let result = tokio::task::spawn_blocking(move || {
                                    match files::read_file(&path_clone) {
                                        Ok(content) => {
                                            // Check file size before processing
                                            const MAX_FILE_SIZE: usize = 5_000_000; // 5MB
                                            if content.len() > MAX_FILE_SIZE {
                                                return Err(format!(
                                                    "File too large ({} MB). Maximum supported size is {} MB.",
                                                    content.len() / 1_000_000,
                                                    MAX_FILE_SIZE / 1_000_000
                                                ));
                                            }
                                            // Create buffer in the background thread
                                            let buffer = TextBuffer::new(content.clone());
                                            Ok((path, content, buffer))
                                        }
                                        Err(e) => Err(format!("Failed to read file: {}", e)),
                                    }
                                }).await;
                                
                                match result {
                                    Ok(Ok((path, content, buffer))) => {
                                        Message::FileLoaded(Ok((path, content, buffer)))
                                    }
                                    Ok(Err(e)) => Message::FileLoaded(Err(e)),
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
                    Ok((path, content, buffer)) => {
                        self.active_file_path = Some(path.clone());
                        self.editor_buffer = Some(buffer);
                        
                        // Check file size thresholds
                        let file_size = content.len();
                        const LARGE_THRESHOLD: usize = 100_000; // 100KB
                        const VERY_LARGE_THRESHOLD: usize = 5_000_000; // 5MB
                        
                        // For very large files, show a warning and potentially limit functionality
                        if file_size > VERY_LARGE_THRESHOLD {
                            self.status_message = format!(
                                "Very large file opened ({} MB) - editing may be limited",
                                file_size / 1_000_000
                            );
                            self.is_file_too_large_for_editor = true;
                        } else if file_size > LARGE_THRESHOLD {
                            self.status_message = format!(
                                "Large file opened ({} KB) - performance may be affected",
                                file_size / 1_000
                            );
                            self.is_file_too_large_for_editor = false;
                        } else {
                            self.status_message = format!("File loaded: {} ({} bytes)", path, file_size);
                            self.is_file_too_large_for_editor = false;
                        }
                        
                        // Initialize the text editor content from the buffer
                        // For large files, this is a one-time cost on file open
                        if let Some(ref buffer) = self.editor_buffer {
                            // Only load the full text if the file is not too large
                            if file_size <= VERY_LARGE_THRESHOLD {
                                let text = buffer.text();
                                self.text_editor = text_editor::Content::with_text(&text);
                            } else {
                                // For very large files, load an empty editor with a message
                                self.text_editor = text_editor::Content::with_text(
                                    &format!("// File too large to display ({} MB)\n// Open in external editor for full content", 
                                    file_size / 1_000_000)
                                );
                            }
                        }
                        
                        self.error_message = None;
                        self.is_dirty = false;
                        
                        // Update workspace state
                        {
                            let mut state = self.workspace_state.lock().unwrap();
                            state.open_buffer(&path, content);
                        }
                    }
                    Err(e) => {
                        self.error_message = Some(e);
                        self.status_message = "Failed to load file".to_string();
                    }
                }
                Command::none()
            }
            Message::EditorContentChanged(action) => {
                // Don't process edits if the file is too large for editor
                if self.is_file_too_large_for_editor {
                    self.status_message = "File is too large - editing disabled".to_string();
                    return Command::none();
                }
                
                // First, perform the action on the text editor
                self.text_editor.perform(action.clone());
                
                // Update the canonical buffer incrementally
                if let Some(ref mut buffer) = self.editor_buffer {
                    match extract_edit_info(&action) {
                        Some(EditInfo::Insert { char_idx, text }) => {
                            if buffer.insert_char_idx(char_idx, &text).is_ok() {
                                self.status_message = "Inserted text".to_string();
                            } else {
                                // Fall back to full update
                                let current_text = self.text_editor.text();
                                buffer.replace_all(&current_text);
                                self.status_message = "Fallback: full update after insert error".to_string();
                            }
                        }
                        Some(EditInfo::Delete { start, end }) => {
                            if buffer.delete_char_range(start, end).is_ok() {
                                self.status_message = "Deleted range".to_string();
                            } else {
                                // Fall back to full update
                                let current_text = self.text_editor.text();
                                buffer.replace_all(&current_text);
                                self.status_message = "Fallback: full update after delete error".to_string();
                            }
                        }
                        None => {
                            // For other actions, fall back to full update
                            let current_text = self.text_editor.text();
                            buffer.replace_all(&current_text);
                            self.status_message = "Full update for other action".to_string();
                        }
                    }
                    self.is_dirty = buffer.is_dirty();
                }
                Command::none()
            }
            Message::SaveFile => {
                if let Some(path) = &self.active_file_path {
                    if let Some(ref buffer) = self.editor_buffer {
                        let content = buffer.text();
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
                        self.status_message = "No buffer to save".to_string();
                        Command::none()
                    }
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
                        let error_msg = e.clone();
                        self.error_message = Some(e);
                        self.status_message = format!("Failed to save file: {}", error_msg);
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
            Message::TextEditorContentCreated(path) => {
                // This message is no longer needed since we handle loading directly in FileLoaded
                // But keep it for compatibility
                self.status_message = format!("Loaded: {}", path);
                self.error_message = None;
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
            self.is_dirty,
            &self.status_message,
            self.error_message.as_ref(),
            self.active_activity,
            self.ai_panel_visible,
            &self.prompt_input,
            &self.expanded_directories,
            &self.text_editor,
            self.editor_buffer.as_ref(),
            self.is_file_too_large_for_editor,
        )
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        iced::keyboard::on_key_press(|key, modifiers| {
            Some(Message::KeyPressed(key, modifiers))
        })
    }
}
