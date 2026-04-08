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
    // New: Request to open a file with metadata check first
    FileOpenRequested(String),
    // New: Metadata loaded (size, etc.)
    FileMetadataLoaded(Result<FileMetadata, String>),
    // New: File content loaded
    FileLoaded(Result<(String, String, TextBuffer), String>),
    // New: Confirm opening a large file
    ConfirmOpenLargeFile(String, u64),
    // New: Open in read-only mode
    OpenLargeFileReadOnly(String),
    // New: Cancel file open
    CancelOpenFile,
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

// New: File metadata structure
#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub path: String,
    pub size: u64,
    pub is_binary: bool,
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
    // New: File loading state
    file_loading_state: FileLoadingState,
}

// New: File loading states
#[derive(Debug, Clone)]
pub enum FileLoadingState {
    Idle,
    LoadingMetadata { path: String },
    LoadingContent { path: String, size: u64 },
    LargeFileWarning { path: String, size: u64 },
    VeryLargeFileWarning { path: String, size: u64 },
    ReadOnlyPreview { path: String, size: u64 },
}

// New: File size thresholds
const SMALL_FILE_THRESHOLD: u64 = 100 * 1024; // 100KB
const LARGE_FILE_THRESHOLD: u64 = 5 * 1024 * 1024; // 5MB
const VERY_LARGE_FILE_THRESHOLD: u64 = 50 * 1024 * 1024; // 50MB

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
                file_loading_state: FileLoadingState::Idle,
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
                        // Start by loading metadata first
                        self.file_loading_state = FileLoadingState::LoadingMetadata { 
                            path: path.clone() 
                        };
                        self.status_message = format!("Checking {}...", entry.name);
                        self.error_message = None;
                        
                        // Request metadata check by returning a command that will trigger FileOpenRequested
                        // We need to return a Command, not a Message directly
                        Command::perform(
                            async move {
                                // Small delay to ensure UI updates first
                                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                                Message::FileOpenRequested(path)
                            },
                            |msg| msg,
                        )
                    } else {
                        // For directories, toggle expansion
                        let path = entry.path.clone();
                        self.expanded_directories.insert(path.clone());
                        Command::perform(
                            async move {
                                Message::ToggleDirectory(path)
                            },
                            |msg| msg,
                        )
                    }
                } else {
                    Command::none()
                }
            }
            Message::FileOpenRequested(path) => {
                // This message is now handled directly in FileSelected
                // But keep it for compatibility
                self.file_loading_state = FileLoadingState::LoadingMetadata { 
                    path: path.clone() 
                };
                self.status_message = format!("Checking file size...");
                
                Command::perform(
                    async move {
                        // Get file metadata in a background task
                        let result = tokio::task::spawn_blocking(move || {
                            match std::fs::metadata(&path) {
                                Ok(metadata) => {
                                    // Simple binary detection: check first few bytes
                                    let is_binary = match std::fs::read(&path) {
                                        Ok(bytes) => bytes.iter().take(1024).any(|&b| b == 0),
                                        Err(_) => false,
                                    };
                                    Ok(FileMetadata {
                                        path: path.clone(),
                                        size: metadata.len(),
                                        is_binary,
                                    })
                                }
                                Err(e) => Err(format!("Failed to read file metadata: {}", e)),
                            }
                        }).await;
                        
                        match result {
                            Ok(Ok(metadata)) => Message::FileMetadataLoaded(Ok(metadata)),
                            Ok(Err(e)) => Message::FileMetadataLoaded(Err(e)),
                            Err(join_err) => Message::FileMetadataLoaded(Err(format!("Failed to join task: {}", join_err))),
                        }
                    },
                    |result| result,
                )
            }
            Message::FileMetadataLoaded(result) => {
                match result {
                    Ok(metadata) => {
                        // Check file size thresholds
                        if metadata.size > VERY_LARGE_FILE_THRESHOLD {
                            self.file_loading_state = FileLoadingState::VeryLargeFileWarning {
                                path: metadata.path.clone(),
                                size: metadata.size,
                            };
                            self.status_message = format!("Very large file detected ({} MB)", metadata.size / (1024 * 1024));
                            // For now, automatically open in read-only mode
                            // In the future, we could ask for confirmation
                            return Command::perform(
                                async move {
                                    // Small delay to show the warning
                                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                                    Message::OpenLargeFileReadOnly(metadata.path)
                                },
                                |msg| msg,
                            );
                        } else if metadata.size > LARGE_FILE_THRESHOLD {
                            self.file_loading_state = FileLoadingState::LargeFileWarning {
                                path: metadata.path.clone(),
                                size: metadata.size,
                            };
                            self.status_message = format!("Large file detected ({} KB)", metadata.size / 1024);
                            // Automatically proceed with loading, but show warning
                            return Command::perform(
                                async move {
                                    Message::ConfirmOpenLargeFile(metadata.path, metadata.size)
                                },
                                |msg| msg,
                            );
                        } else {
                            // Small file, proceed with normal loading
                            self.file_loading_state = FileLoadingState::LoadingContent {
                                path: metadata.path.clone(),
                                size: metadata.size,
                            };
                            self.status_message = format!("Loading file...");
                            
                            return Command::perform(
                                async move {
                                    let path = metadata.path;
                                    let result = tokio::task::spawn_blocking(move || {
                                        match files::read_file(&path) {
                                            Ok(content) => {
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
                            );
                        }
                    }
                    Err(e) => {
                        self.file_loading_state = FileLoadingState::Idle;
                        self.error_message = Some(e);
                        self.status_message = "Failed to load file metadata".to_string();
                    }
                }
                Command::none()
            }
            Message::ConfirmOpenLargeFile(path, size) => {
                self.file_loading_state = FileLoadingState::LoadingContent {
                    path: path.clone(),
                    size,
                };
                self.status_message = format!("Loading large file...");
                
                Command::perform(
                    async move {
                        let result = tokio::task::spawn_blocking(move || {
                            match files::read_file(&path) {
                                Ok(content) => {
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
            }
            Message::OpenLargeFileReadOnly(path) => {
                self.file_loading_state = FileLoadingState::ReadOnlyPreview {
                    path: path.clone(),
                    size: 0, // We'll get this from metadata
                };
                self.status_message = format!("Opening in read-only mode...");
                self.active_file_path = Some(path.clone());
                self.is_file_too_large_for_editor = true;
                
                // For very large files, only load a preview
                Command::perform(
                    async move {
                        let result = tokio::task::spawn_blocking(move || {
                            // Only read first 100KB for preview
                            use std::fs::File;
                            use std::io::Read;
                            
                            let mut file = match File::open(&path) {
                                Ok(f) => f,
                                Err(e) => return Err(format!("Failed to open file: {}", e)),
                            };
                            
                            let mut buffer = vec![0; 100 * 1024]; // 100KB
                            match file.read(&mut buffer) {
                                Ok(bytes_read) => {
                                    // Convert to string, but be careful about binary files
                                    let content = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
                                    let text_buffer = TextBuffer::new(content.clone());
                                    Ok((path, content, text_buffer))
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
            }
            Message::CancelOpenFile => {
                self.file_loading_state = FileLoadingState::Idle;
                self.status_message = "File open cancelled".to_string();
                Command::none()
            }
            Message::FileLoaded(result) => {
                match result {
                    Ok((path, content, buffer)) => {
                        self.active_file_path = Some(path.clone());
                        self.editor_buffer = Some(buffer);
                        self.file_loading_state = FileLoadingState::Idle;
                        
                        let file_size = content.len();
                        
                        // Determine if we're in read-only mode based on the loading state
                        match &self.file_loading_state {
                            FileLoadingState::ReadOnlyPreview { .. } => {
                                self.is_file_too_large_for_editor = true;
                                self.status_message = format!(
                                    "Very large file opened in read-only preview ({} bytes shown)",
                                    file_size
                                );
                                // For read-only preview, show the content directly
                                self.text_editor = text_editor::Content::with_text(&format!(
                                    "// Read-only preview (first 100KB)\n// File is very large\n\n{}",
                                    content
                                ));
                            }
                            _ => {
                                // Check thresholds for normal files
                                if file_size > LARGE_FILE_THRESHOLD as usize {
                                    self.status_message = format!(
                                        "Large file opened ({} MB) - editing enabled",
                                        file_size / (1024 * 1024)
                                    );
                                    self.is_file_too_large_for_editor = false;
                                } else {
                                    self.status_message = format!("File loaded: {} ({} bytes)", path, file_size);
                                    self.is_file_too_large_for_editor = false;
                                }
                                
                                // Initialize editor content, but do it in a way that doesn't block
                                // This is still synchronous, but for files under the threshold it should be fine
                                if let Some(ref buffer) = self.editor_buffer {
                                    let text = buffer.text();
                                    self.text_editor = text_editor::Content::with_text(&text);
                                }
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
                        self.file_loading_state = FileLoadingState::Idle;
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
            &self.file_loading_state,
        )
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        iced::keyboard::on_key_press(|key, modifiers| {
            Some(Message::KeyPressed(key, modifiers))
        })
    }
}
