use std::sync::{Arc, Mutex};
use workspace_daemon::files;
use workspace_model::state::WorkspaceState;
use core_types::workspace::DirectoryEntry;
use editor_buffer::buffer::TextBuffer;
use iced::{
    widget::{button, column, container, row, scrollable, text, text_input},
    Alignment, Element, Length, Task,
};

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
}

impl iced::Application for App {
    type Message = Message;
    type Theme = iced::Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Task<Message>) {
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
            },
            Task::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Neote")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::WorkspacePathChanged(path) => {
                self.workspace_path = path;
                Task::none()
            }
            Message::OpenWorkspace => {
                if self.workspace_path.is_empty() {
                    self.status_message = "Please enter a workspace path".to_string();
                    return Task::none();
                }
                
                let path = self.workspace_path.clone();
                Task::perform(
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
                Task::none()
            }
            Message::FileSelected(index) => {
                if index < self.file_entries.len() {
                    let entry = &self.file_entries[index];
                    if !entry.is_dir {
                        let path = entry.path.clone();
                        self.active_file_path = Some(path.clone());
                        
                        Task::perform(
                            async move {
                                match files::read_file(&path) {
                                    Ok(content) => Message::FileLoaded(Ok((path, content))),
                                    Err(e) => Message::FileLoaded(Err(format!("Failed to read file: {}", e))),
                                }
                            },
                            |result| result,
                        )
                    } else {
                        Task::none()
                    }
                } else {
                    Task::none()
                }
            }
            Message::FileLoaded(result) => {
                match result {
                    Ok((path, content)) => {
                        let mut buffer = TextBuffer::new(content.clone());
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
                Task::none()
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
                Task::none()
            }
            Message::SaveFile => {
                if let Some(path) = &self.active_file_path {
                    let content = self.editor_content.clone();
                    let path_clone = path.clone();
                    
                    Task::perform(
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
                    Task::none()
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
                Task::none()
            }
            Message::RefreshWorkspace => {
                if !self.workspace_path.is_empty() {
                    let path = self.workspace_path.clone();
                    Task::perform(
                        async move {
                            match files::list_directory(&path) {
                                Ok(entries) => Message::WorkspaceLoaded(Ok(entries)),
                                Err(e) => Message::WorkspaceLoaded(Err(format!("Failed to refresh workspace: {}", e))),
                            }
                        },
                        |result| result,
                    )
                } else {
                    Task::none()
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let workspace_controls = row![
            text_input("Workspace path", &self.workspace_path)
                .on_input(Message::WorkspacePathChanged)
                .padding(10)
                .width(Length::FillPortion(3)),
            button("Open Workspace")
                .on_press(Message::OpenWorkspace)
                .padding(10),
            button("Refresh")
                .on_press(Message::RefreshWorkspace)
                .padding(10),
        ]
        .spacing(10)
        .align_items(Alignment::Center);

        let file_list = if self.file_entries.is_empty() {
            container(text("No files found").size(16))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
        } else {
            let items: Element<_> = scrollable(
                column(
                    self.file_entries
                        .iter()
                        .enumerate()
                        .map(|(i, entry)| {
                            let label = if entry.is_dir {
                                format!("📁 {}", entry.name)
                            } else {
                                format!("📄 {}", entry.name)
                            };
                            button(text(label).size(14))
                                .on_press(Message::FileSelected(i))
                                .width(Length::Fill)
                                .padding(8)
                                .into()
                        })
                        .collect(),
                )
                .spacing(5),
            )
            .height(Length::Fill)
            .into();

            container(items).width(Length::Fill).height(Length::Fill)
        };

        let editor_header = row![
            text(
                self.active_file_path
                    .as_ref()
                    .map(|p| format!("Editing: {}", p))
                    .unwrap_or_else(|| "No file selected".to_string())
            )
            .size(16),
            if self.is_dirty {
                text(" (modified)").size(16).style(iced::Color::from_rgb8(255, 165, 0))
            } else {
                text(" (saved)").size(16).style(iced::Color::from_rgb8(0, 128, 0))
            },
            button("Save").on_press(Message::SaveFile).padding(8),
        ]
        .spacing(10)
        .align_items(Alignment::Center);

        let editor = scrollable(
            text_input("", &self.editor_content)
                .on_input(Message::EditorContentChanged)
                .padding(10)
        )
        .height(Length::Fill);

        let status_bar = row![
            text(&self.status_message).size(14),
            if let Some(err) = &self.error_message {
                text(format!(" | Error: {}", err)).size(14).style(iced::Color::from_rgb8(255, 0, 0))
            } else {
                text("").size(14)
            }
        ]
        .spacing(10);

        let content = column![
            workspace_controls,
            iced::widget::horizontal_rule(1),
            row![
                container(file_list).width(Length::FillPortion(2)),
                container(
                    column![
                        editor_header,
                        iced::widget::horizontal_rule(1),
                        editor,
                    ]
                    .spacing(10)
                )
                .width(Length::FillPortion(5))
                .padding(10),
            ]
            .height(Length::Fill),
            iced::widget::horizontal_rule(1),
            status_bar,
        ]
        .spacing(10)
        .padding(10)
        .height(Length::Fill);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
