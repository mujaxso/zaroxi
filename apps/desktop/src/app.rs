use std::sync::{Arc, Mutex};
use workspace_daemon::files;
use workspace_model::state::WorkspaceState;
use core_types::workspace::DirectoryEntry;
use editor_buffer::buffer::TextBuffer;
use iced::{Element, Command};

#[derive(Debug, Clone)]
pub enum Message {
    WorkspacePathChanged(String),
    OpenWorkspace,
    WorkspaceLoaded(Result<Vec<DirectoryEntry>, String>),
    FileSelected(usize),
    FileLoaded(Result<(String, String), String>),
    EditorContentChanged(String),
    SaveFile,
    FileSaved(Result<(), String>),
    RefreshWorkspace,
    ToggleAiPanel,
    ActivitySelected(Activity),
    PromptInputChanged(String),
    SendPrompt,
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
                        
                        Command::perform(
                            async move {
                                match files::read_file(&path) {
                                    Ok(content) => Message::FileLoaded(Ok((path, content))),
                                    Err(e) => Message::FileLoaded(Err(format!("Failed to read file: {}", e))),
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
                        let buffer = TextBuffer::new(content.clone());
                        self.editor_content = content;
                        self.editor_buffer = Some(buffer);
                        self.is_dirty = false;
                        
                        let mut state = self.workspace_state.lock().unwrap();
                        state.open_buffer(&path, self.editor_content.clone());
                        
                        self.status_message = format!("Loaded: {}", path);
                        self.error_message = None;
                    }
                    Err(e) => {
                        self.error_message = Some(e);
                        self.status_message = "Failed to load file".to_string();
                    }
                }
                Command::none()
            }
            Message::EditorContentChanged(new_content) => {
                self.editor_content = new_content.clone();
                if let Some(buffer) = &mut self.editor_buffer {
                    buffer.replace_all(new_content);
                    self.is_dirty = buffer.is_dirty();
                }
                self.status_message = if self.is_dirty {
                    "File has unsaved changes".to_string()
                } else {
                    "All changes saved".to_string()
                };
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
                Command::none()
            }
            Message::ActivitySelected(activity) => {
                self.active_activity = activity;
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
        )
    }
}
