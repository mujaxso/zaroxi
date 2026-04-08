use iced::{
    widget::{
        button, column, container, horizontal_space, row, scrollable, text,
        text_input, vertical_rule,
    },
    Alignment, Element, Length,
    theme,
};

use crate::app::{Activity, Message};

pub fn ide_layout<'a>(
    workspace_path: &'a str,
    file_entries: &'a [core_types::workspace::DirectoryEntry],
    active_file_path: Option<&'a String>,
    is_dirty: bool,
    status_message: &'a str,
    error_message: Option<&'a String>,
    active_activity: Activity,
    ai_panel_visible: bool,
    prompt_input: &'a str,
    _expanded_directories: &'a std::collections::HashSet<String>,
    text_editor: &'a iced::widget::text_editor::Content,
    editor_buffer: Option<&'a editor_buffer::buffer::TextBuffer>,
    is_file_too_large_for_editor: bool,
    file_loading_state: &'a crate::app::FileLoadingState,
) -> Element<'a, Message> {
    // Top bar
    let top_bar = top_bar(workspace_path, is_dirty);

    // Activity rail
    let activity_rail = activity_rail(active_activity);

    // Main content area
    let ai_panel_widget: Element<_> = if ai_panel_visible {
        container(ai_panel(prompt_input))
            .width(Length::FillPortion(3))
            .height(Length::Fill)
            .style(theme::Container::Box)
            .into()
    } else {
        container(horizontal_space()).width(Length::Fixed(0.0)).into()
    };

    let main_content = row![
        // Activity rail
        activity_rail,
        vertical_rule(1),
        // Left panel (explorer) - flexible width
        container(left_panel_with_expanded(file_entries, active_activity, _expanded_directories, workspace_path))
            .width(Length::FillPortion(2))
            .height(Length::Fill),
        vertical_rule(1),
        // Editor area - takes most space
        container(editor_panel(active_file_path, text_editor, is_dirty, editor_buffer, is_file_too_large_for_editor, file_loading_state))
            .width(Length::FillPortion(5))
            .height(Length::Fill),
        // AI panel (conditionally visible) - flexible width
        ai_panel_widget,
    ]
    .height(Length::Fill);

    // Status bar
    let status_bar = status_bar(
        status_message,
        error_message,
        active_file_path,
        file_entries.len(),
    );

    // Combine everything
    let content = column![
        top_bar,
        iced::widget::horizontal_rule(1),
        main_content,
        iced::widget::horizontal_rule(1),
        status_bar,
    ]
    .height(Length::Fill);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::Container::Box)
        .into()
}

fn top_bar<'a>(workspace_path: &'a str, is_dirty: bool) -> Element<'a, Message> {
    let status_indicator: Element<_> = if is_dirty {
        row![
            text("●").size(14).style(iced::theme::Text::Color(iced::Color::from_rgb8(255, 165, 0))),
            text("Unsaved").size(14)
        ]
        .spacing(4)
        .align_items(Alignment::Center)
        .into()
    } else {
        row![
            text("✓").size(14).style(iced::theme::Text::Color(iced::Color::from_rgb8(0, 200, 0))),
            text("Saved").size(14)
        ]
        .spacing(4)
        .align_items(Alignment::Center)
        .into()
    };

    row![
        text("Neote").size(20).style(iced::theme::Text::Color(iced::Color::from_rgb8(100, 150, 255))),
        horizontal_space(),
        text_input("Workspace path", workspace_path)
            .on_input(Message::WorkspacePathChanged)
            .padding(8)
            .width(Length::FillPortion(3)),
        button("Open")
            .on_press(Message::OpenWorkspace)
            .padding([8, 12])
            .style(theme::Button::Secondary),
        button("Refresh")
            .on_press(Message::RefreshWorkspace)
            .padding([8, 12])
            .style(theme::Button::Secondary),
        horizontal_space(),
        status_indicator,
        button("Save")
            .on_press(Message::SaveFile)
            .padding([8, 16])
            .style(theme::Button::Primary),
    ]
    .padding([8, 16])
    .align_items(Alignment::Center)
    .into()
}

fn activity_rail<'a>(active_activity: Activity) -> Element<'a, Message> {
    let activities = [
        (Activity::Explorer, "📁", "Explorer"),
        (Activity::Search, "🔍", "Search"),
        (Activity::Ai, "🤖", "AI"),
        (Activity::SourceControl, "🔄", "Git"),
        (Activity::Settings, "⚙️", "Settings"),
    ];

    let children: Vec<Element<_>> = activities
        .iter()
        .map(|&(activity, icon, label)| {
            let is_active = activity == active_activity;
            let button_style = if is_active {
                theme::Button::Primary
            } else {
                theme::Button::Secondary
            };
            // For AI activity, we want to toggle the panel visibility
            let message = if activity == Activity::Ai {
                Message::ToggleAiPanel
            } else {
                Message::ActivitySelected(activity)
            };
            container(
                button(
                    column![
                        text(icon).size(20),
                        text(label).size(12),
                    ]
                    .align_items(Alignment::Center)
                    .spacing(4),
                )
                .on_press(message)
                .padding(12)
                .style(button_style),
            )
            .width(Length::Fixed(70.0))
            .center_x()
            .into()
        })
        .collect();

    column(children)
        .spacing(8)
        .padding([16, 8])
        .into()
}

fn left_panel_with_expanded<'a>(
    file_entries: &'a [core_types::workspace::DirectoryEntry],
    active_activity: Activity,
    _expanded_directories: &'a std::collections::HashSet<String>,
    workspace_path: &'a str,
) -> Element<'a, Message> {
    match active_activity {
        Activity::Explorer => explorer_panel_with_expanded(file_entries, _expanded_directories, workspace_path),
        Activity::Search => search_panel(),
        Activity::SourceControl => terminal_panel(),
        Activity::Settings => settings_panel(),
        _ => placeholder_panel(&format!("{} panel", format!("{:?}", active_activity))),
    }
}


fn explorer_panel<'a>(file_entries: &'a [core_types::workspace::DirectoryEntry]) -> Element<'a, Message> {
    let content: Element<_> = if file_entries.is_empty() {
        container(
            column![
                text("No files in workspace")
                    .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150))),
                button("Open Workspace")
                    .on_press(Message::OpenWorkspace)
                    .padding(8)
                    .style(theme::Button::Secondary),
            ]
            .spacing(10)
            .align_items(Alignment::Center)
        )
        .center_y()
        .center_x()
        .height(Length::Fill)
        .into()
    } else {
        let children: Vec<Element<_>> = file_entries
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                let is_file = !entry.is_dir;
                let icon = if is_file { "📄" } else { "📁" };
                let text_color = if is_file {
                    iced::Color::from_rgb8(220, 220, 220)
                } else {
                    iced::Color::from_rgb8(180, 180, 255)
                };
                let padding_left = if entry.is_dir { 0 } else { 20 };
                container(
                    button(
                        row![
                            if entry.is_dir {
                                // Add a toggle button for directories
                                let btn: Element<_> = button("▶")
                                    .on_press(Message::ToggleDirectory(entry.path.clone()))
                                    .style(theme::Button::Text)
                                    .padding(0)
                                    .into();
                                btn
                            } else {
                                let space: Element<_> = horizontal_space().width(20).into();
                                space
                            },
                            text(icon).size(14),
                            text(&entry.name).size(14)
                                .style(iced::theme::Text::Color(text_color)),
                        ]
                        .spacing(8)
                        .align_items(Alignment::Center),
                    )
                    .on_press(if entry.is_dir {
                        Message::ToggleDirectory(entry.path.clone())
                    } else {
                        Message::FileSelected(i)
                    })
                    .padding([6, 12])
                    .width(Length::Fill)
                    .style(theme::Button::Secondary),
                )
                .padding(iced::Padding::new(padding_left as f32))
                .into()
            })
            .collect();
        
        scrollable(
            column(children)
                .spacing(2),
        )
        .height(Length::Fill)
        .into()
    };

    column![
        row![
            text("EXPLORER").size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150))),
            horizontal_space(),
            button("Refresh")
                .on_press(Message::RefreshWorkspace)
                .padding([4, 8])
                .style(theme::Button::Secondary),
        ]
        .padding([12, 16])
        .align_items(Alignment::Center),
        iced::widget::horizontal_rule(1),
        content,
    ]
    .height(Length::Fill)
    .into()
}

fn editor_panel<'a>(
    active_file_path: Option<&'a String>,
    text_editor: &'a iced::widget::text_editor::Content,
    is_dirty: bool,
    editor_buffer: Option<&'a editor_buffer::buffer::TextBuffer>,
    is_file_too_large_for_editor: bool,
    file_loading_state: &'a crate::app::FileLoadingState,
) -> Element<'a, Message> {
    let header = if let Some(path) = active_file_path {
        let mut status_elements = Vec::new();
        
        // File path
        status_elements.push(
            text(path)
                .size(14)
                .style(iced::theme::Text::Color(iced::Color::from_rgb8(200, 200, 200)))
                .into()
        );
        
        status_elements.push(horizontal_space().into());
        
        // Large file warning
        if let Some(buffer) = editor_buffer {
            if buffer.is_very_large() {
                status_elements.push(
                    text("⚠ Very Large (Limited Editing)")
                        .size(12)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(255, 100, 100)))
                        .into()
                );
                status_elements.push(horizontal_space().width(Length::Fixed(10.0)).into());
            } else if buffer.is_large() {
                status_elements.push(
                    text("⚠ Large (Performance may be affected)")
                        .size(12)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(255, 200, 0)))
                        .into()
                );
                status_elements.push(horizontal_space().width(Length::Fixed(10.0)).into());
            }
        }
        
        // Show if file is too large for editor
        if is_file_too_large_for_editor {
            status_elements.push(
                text("⚠ Too Large for Editor (Read-Only)")
                    .size(12)
                    .style(iced::theme::Text::Color(iced::Color::from_rgb8(255, 50, 50)))
                    .into()
            );
            status_elements.push(horizontal_space().width(Length::Fixed(10.0)).into());
        }
        
        // Dirty status (only show if not read-only)
        if !is_file_too_large_for_editor {
            let status_text = if is_dirty {
                text("● Unsaved").size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(255, 165, 0)))
            } else {
                text("✓ Saved").size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(0, 200, 0)))
            };
            status_elements.push(status_text.into());
        }
        
        row(status_elements)
            .padding([12, 16])
            .align_items(Alignment::Center)
    } else {
        row![
            text("No file selected").size(14).style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150))),
            horizontal_space(),
        ]
        .padding([12, 16])
        .align_items(Alignment::Center)
    };

    // Check loading state
    let editor_content = match file_loading_state {
        crate::app::FileLoadingState::LoadingMetadata { path } => {
            container(
                column![
                    text("Checking file size...")
                        .size(16)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 255))),
                    text(format!("Path: {}", path))
                        .size(12)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150))),
                ]
                .align_items(Alignment::Center)
                .spacing(16),
            )
            .center_y()
            .center_x()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        }
        crate::app::FileLoadingState::LoadingContent { path, size } => {
            let size_mb = size / (1024 * 1024);
            let size_kb = size / 1024;
            let size_str = if size_mb > 0 {
                format!("{} MB", size_mb)
            } else {
                format!("{} KB", size_kb)
            };
            container(
                column![
                    text("Loading file...")
                        .size(16)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 255))),
                    text(format!("{} ({})", path, size_str))
                        .size(12)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150))),
                    text("This may take a moment for large files")
                        .size(10)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150))),
                ]
                .align_items(Alignment::Center)
                .spacing(16),
            )
            .center_y()
            .center_x()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        }
        crate::app::FileLoadingState::LargeFileWarning { path, size } => {
            let size_mb = size / (1024 * 1024);
            container(
                column![
                    text("⚠ Large File Detected")
                        .size(16)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(255, 200, 0))),
                    text(format!("{} ({} MB)", path, size_mb))
                        .size(12)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(200, 200, 200))),
                    text("Opening in editable mode...")
                        .size(10)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150))),
                ]
                .align_items(Alignment::Center)
                .spacing(16),
            )
            .center_y()
            .center_x()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        }
        crate::app::FileLoadingState::VeryLargeFileWarning { path, size } => {
            let size_mb = size / (1024 * 1024);
            container(
                column![
                    text("⚠ Very Large File Detected")
                        .size(16)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(255, 100, 100))),
                    text(format!("{} ({} MB)", path, size_mb))
                        .size(12)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(200, 200, 200))),
                    text("Opening in read-only preview mode")
                        .size(10)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150))),
                ]
                .align_items(Alignment::Center)
                .spacing(16),
            )
            .center_y()
            .center_x()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        }
        crate::app::FileLoadingState::ReadOnlyPreview { path, size } => {
            let size_mb = size / (1024 * 1024);
            container(
                column![
                    text("📖 Read-Only Preview")
                        .size(16)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 200, 255))),
                    text(format!("{} ({} MB total)", path, size_mb))
                        .size(12)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(200, 200, 200))),
                    text("Only first 100KB shown. Editing disabled.")
                        .size(10)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150))),
                ]
                .align_items(Alignment::Center)
                .spacing(16),
            )
            .center_y()
            .center_x()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        }
        crate::app::FileLoadingState::Idle => {
            // Original logic for when not loading
            if active_file_path.is_some() {
                if is_file_too_large_for_editor {
                    // Show read-only text view for very large files
                    let content = if let Some(buffer) = editor_buffer {
                        if buffer.is_very_large() {
                            // For very large files, show only first 100KB
                            buffer.slice_char_range(0, 100_000.min(buffer.len_chars())).unwrap_or_else(|_| String::new())
                        } else {
                            buffer.text()
                        }
                    } else {
                        String::new()
                    };
                    let warning = if let Some(buffer) = editor_buffer {
                        if buffer.is_very_large() {
                            format!("\n\n--- File truncated ({} MB total, showing first 100KB) ---", 
                                   buffer.len_chars() / 1_000_000)
                        } else {
                            String::new()
                        }
                    } else {
                        String::new()
                    };
                    scrollable(
                        column![
                            text(content + &warning)
                                .font(iced::Font::MONOSPACE)
                                .size(14),
                        ]
                    )
                    .height(Length::Fill)
                    .into()
                } else {
                    super::editor::editor(text_editor)
                }
            } else {
                container(
                    column![
                        text("Neote").size(32).style(iced::theme::Text::Color(iced::Color::from_rgb8(100, 150, 255))),
                        text("AI‑first IDE").size(16).style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 200))),
                        container(iced::widget::horizontal_rule(1)).width(150),
                        column![
                            button("Open a file from the explorer").style(theme::Button::Secondary),
                            button("Ask AI about the workspace").style(theme::Button::Secondary),
                        ]
                        .spacing(8)
                        .padding(16),
                    ]
                    .align_items(Alignment::Center)
                    .spacing(16),
                )
                .center_y()
                .center_x()
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
            }
        }
    };

    column![
        header,
        iced::widget::horizontal_rule(1),
        editor_content,
    ]
    .height(Length::Fill)
    .into()
}

fn ai_panel<'a>(prompt_input: &'a str) -> Element<'a, Message> {
    column![
        row![
            text("AI ASSISTANT").size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150))),
            horizontal_space(),
            button("⋯").style(theme::Button::Secondary),
        ]
        .padding([12, 16])
        .align_items(Alignment::Center),
        iced::widget::horizontal_rule(1),
        scrollable(
            column![
                container(
                    column![
                        text("Welcome to Neote AI").size(16),
                        text("Ask questions about your code, get explanations, refactor suggestions, and more.")
                            .size(14)
                            .style(iced::theme::Text::Color(iced::Color::from_rgb8(180, 180, 180))),
                    ]
                    .padding(16)
                    .spacing(8)
                )
                .style(theme::Container::Box),
                column![
                    button("Explain this file")
                        .on_press(Message::PromptInputChanged("Explain the current file".to_string()))
                        .padding(12)
                        .style(theme::Button::Secondary),
                    button("Refactor selection")
                        .on_press(Message::PromptInputChanged("Refactor the selected code".to_string()))
                        .padding(12)
                        .style(theme::Button::Secondary),
                    button("Find bugs")
                        .on_press(Message::PromptInputChanged("Find potential bugs in this code".to_string()))
                        .padding(12)
                        .style(theme::Button::Secondary),
                    button("Write tests")
                        .on_press(Message::PromptInputChanged("Write unit tests for this code".to_string()))
                        .padding(12)
                        .style(theme::Button::Secondary),
                ]
                .spacing(8)
                .padding(16),
                container(
                    text("AI features are coming soon. This is a placeholder for the AI assistant interface.")
                        .size(12)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150)))
                )
                .padding(16),
            ]
            .spacing(16)
        )
        .height(Length::Fill),
        iced::widget::horizontal_rule(1),
        row![
            text_input("Ask Neote AI...", prompt_input)
                .on_input(Message::PromptInputChanged)
                .padding(12)
                .width(Length::Fill),
            button("Send")
                .on_press(Message::SendPrompt)
                .padding([12, 16])
                .style(theme::Button::Primary),
        ]
        .padding([8, 16])
        .align_items(Alignment::Center),
    ]
    .height(Length::Fill)
    .into()
}

fn search_panel<'a>() -> Element<'a, Message> {
    column![
        row![
            text("SEARCH").size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150))),
            horizontal_space(),
            button("⋯")
                .on_press(Message::PromptInputChanged("Search options".to_string()))
                .style(theme::Button::Secondary),
        ]
        .padding([12, 16])
        .align_items(Alignment::Center),
        iced::widget::horizontal_rule(1),
        container(
            column![
                text_input("Search in workspace...", "")
                    .on_input(|query| Message::PromptInputChanged(format!("search: {}", query)))
                    .padding(12)
                    .width(Length::Fill),
                button("Find All")
                    .on_press(Message::PromptInputChanged("Find all in workspace".to_string()))
                    .style(theme::Button::Primary)
                    .width(Length::Fill)
                    .padding(8),
                container(
                    text("No search results")
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150)))
                )
                .center_y()
                .center_x()
                .height(Length::Fill)
            ]
            .spacing(16)
            .padding(16)
        )
        .height(Length::Fill)
    ]
    .width(Length::Fixed(250.0))
    .height(Length::Fill)
    .into()
}

fn terminal_panel<'a>() -> Element<'a, Message> {
    super::terminal::terminal("")
}

fn settings_panel<'a>() -> Element<'a, Message> {
    column![
        row![
            text("SETTINGS").size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150))),
            horizontal_space(),
            button("Save")
                .on_press(Message::PromptInputChanged("Settings saved".to_string()))
                .style(theme::Button::Primary),
        ]
        .padding([12, 16])
        .align_items(Alignment::Center),
        iced::widget::horizontal_rule(1),
        scrollable(
            column![
                container(
                    column![
                        text("Editor").size(16),
                        text("Font size:").size(14),
                        text_input("14", "14")
                            .on_input(|size| Message::PromptInputChanged(format!("Font size: {}", size)))
                            .padding(8),
                        text("Theme:").size(14),
                        button("Dark")
                            .on_press(Message::PromptInputChanged("Theme set to Dark".to_string()))
                            .style(theme::Button::Secondary),
                        button("Light")
                            .on_press(Message::PromptInputChanged("Theme set to Light".to_string()))
                            .style(theme::Button::Secondary),
                    ]
                    .spacing(8)
                    .padding(16)
                )
                .style(theme::Container::Box),
                container(
                    column![
                        text("AI Settings").size(16),
                        text("Model:").size(14),
                        text_input("gpt-4", "gpt-4")
                            .on_input(|model| Message::PromptInputChanged(format!("AI model: {}", model)))
                            .padding(8),
                        text("API Key:").size(14),
                        text_input("••••••••", "")
                            .on_input(|_| Message::PromptInputChanged("API key updated".to_string()))
                            .padding(8),
                    ]
                    .spacing(8)
                    .padding(16)
                )
                .style(theme::Container::Box),
            ]
            .spacing(16)
            .padding(16)
        )
        .height(Length::Fill)
    ]
    .width(Length::Fixed(250.0))
    .height(Length::Fill)
    .into()
}

fn explorer_panel_with_expanded<'a>(
    file_entries: &'a [core_types::workspace::DirectoryEntry],
    expanded_directories: &'a std::collections::HashSet<String>,
    workspace_path: &'a str,
) -> Element<'a, Message> {
    // Build a tree structure from the flat list of entries
    // We'll use indices to avoid lifetime issues
    let mut root_indices = Vec::new();
    let mut children_map: std::collections::HashMap<String, Vec<usize>> = 
        std::collections::HashMap::new();
    
    // Use the provided workspace path
    let workspace_root = if workspace_path.is_empty() {
        String::new()
    } else {
        normalize_path(workspace_path)
    };
    
    // Build children map: parent path -> indices of children
    for (i, entry) in file_entries.iter().enumerate() {
        let path = std::path::Path::new(&entry.path);
        if let Some(parent) = path.parent() {
            let parent_str = normalize_path(&parent.to_string_lossy());
            children_map.entry(parent_str).or_insert_with(Vec::new).push(i);
        } else {
            // Entry has no parent (root of filesystem)
            root_indices.push(i);
        }
    }
    
    // Identify root entries: those whose parent is the workspace root
    // Also include entries with no parent (already added above)
    for (i, entry) in file_entries.iter().enumerate() {
        let path = std::path::Path::new(&entry.path);
        if let Some(parent) = path.parent() {
            let parent_str = normalize_path(&parent.to_string_lossy());
            if parent_str == workspace_root {
                if !root_indices.contains(&i) {
                    root_indices.push(i);
                }
            }
        }
    }
    
    // If we still have no root entries, maybe all files are at workspace root
    // Add entries whose parent is empty or "."
    if root_indices.is_empty() {
        for (i, entry) in file_entries.iter().enumerate() {
            let path = std::path::Path::new(&entry.path);
            if let Some(parent) = path.parent() {
                let parent_str = parent.to_string_lossy();
                if parent_str == "." || parent_str == "" {
                    if !root_indices.contains(&i) {
                        root_indices.push(i);
                    }
                }
            }
        }
    }
    
    // Final fallback: if still empty, add all entries
    if root_indices.is_empty() {
        root_indices = (0..file_entries.len()).collect();
    }
    
    // Sort indices: directories first, then files, both alphabetically
    root_indices.sort_by(|&a_idx, &b_idx| {
        let a = &file_entries[a_idx];
        let b = &file_entries[b_idx];
        if a.is_dir != b.is_dir {
            b.is_dir.cmp(&a.is_dir) // Directories first (true > false)
        } else {
            a.name.cmp(&b.name)
        }
    });
    
    // Render the tree
    let content: Element<_> = if file_entries.is_empty() {
        container(
            column![
                text("No files in workspace")
                    .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150))),
                button("Open Workspace")
                    .on_press(Message::OpenWorkspace)
                    .padding(8)
                    .style(theme::Button::Secondary),
            ]
            .spacing(10)
            .align_items(Alignment::Center)
        )
        .center_y()
        .center_x()
        .height(Length::Fill)
        .into()
    } else {
        // Collect all elements first to avoid lifetime issues
        let mut all_elements = Vec::new();
        for &idx in &root_indices {
            let entry = &file_entries[idx];
            let mut elements = render_directory_entry_with_indices(
                entry,
                idx,
                file_entries,
                &children_map,
                expanded_directories,
                0,
            );
            all_elements.append(&mut elements);
        }
        
        // If no elements were generated (tree building failed), fall back to flat list
        if all_elements.is_empty() {
            return explorer_panel(file_entries);
        }
        
        scrollable(
            column(all_elements)
                .spacing(2),
        )
        .height(Length::Fill)
        .into()
    };

    column![
        row![
            text("EXPLORER").size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150))),
            horizontal_space(),
            button("Refresh")
                .on_press(Message::RefreshWorkspace)
                .padding([4, 8])
                .style(theme::Button::Secondary),
        ]
        .padding([12, 16])
        .align_items(Alignment::Center),
        iced::widget::horizontal_rule(1),
        content,
    ]
    .height(Length::Fill)
    .into()
}


// Helper function to normalize paths for consistent comparison
fn normalize_path(path: &str) -> String {
    let path = std::path::Path::new(path);
    // Convert to absolute path if possible
    let path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        // Try to canonicalize to get absolute path
        match std::fs::canonicalize(path) {
            Ok(p) => p,
            Err(_) => path.to_path_buf(),
        }
    };
    // Convert to string and remove trailing separator if present
    let s = path.to_string_lossy().to_string();
    // Remove any trailing separator
    s.trim_end_matches(std::path::MAIN_SEPARATOR).to_string()
}

fn render_directory_entry_with_indices<'a, 'b>(
    entry: &'a core_types::workspace::DirectoryEntry,
    _idx: usize,
    file_entries: &'a [core_types::workspace::DirectoryEntry],
    children_map: &'b std::collections::HashMap<String, Vec<usize>>,
    expanded_directories: &'a std::collections::HashSet<String>,
    depth: usize,
) -> Vec<Element<'a, Message>> {
    let mut elements = Vec::new();
    
    let is_expanded = expanded_directories.contains(&entry.path);
    let icon = if entry.is_dir {
        if is_expanded { "📂" } else { "📁" }
    } else {
        "📄"
    };
    
    let text_color = if entry.is_dir {
        iced::Color::from_rgb8(180, 180, 255)
    } else {
        iced::Color::from_rgb8(220, 220, 220)
    };
    
    let padding_left = depth * 20;
    
    // Create the entry element
    let entry_element = container(
        button(
            row![
                if entry.is_dir {
                    // Add a toggle button for directories
                    let btn_icon = if is_expanded { "▼" } else { "▶" };
                    let btn: Element<_> = button(btn_icon)
                        .on_press(Message::ToggleDirectory(entry.path.clone()))
                        .style(theme::Button::Text)
                        .padding(0)
                        .into();
                    btn
                } else {
                    let space: Element<_> = horizontal_space().width(20).into();
                    space
                },
                text(icon).size(14),
                text(&entry.name).size(14)
                    .style(iced::theme::Text::Color(text_color)),
            ]
            .spacing(8)
            .align_items(Alignment::Center),
        )
        .on_press(if entry.is_dir {
            Message::ToggleDirectory(entry.path.clone())
        } else {
            Message::FileSelectedByPath(entry.path.clone())
        })
        .padding([6, 12])
        .width(Length::Fill)
        .style(theme::Button::Secondary),
    )
    .padding(iced::Padding::new(padding_left as f32))
    .into();
    
    elements.push(entry_element);
    
    // If this is a directory and it's expanded, render its children
    if entry.is_dir && is_expanded {
        // Normalize the path for lookup
        let normalized_path = normalize_path(&entry.path);
        // Try to find children in the map
        if let Some(child_indices) = children_map.get(&normalized_path) {
            // Sort child indices: directories first, then files
            let mut sorted_child_indices: Vec<usize> = child_indices.clone();
            sorted_child_indices.sort_by(|&a_idx, &b_idx| {
                let a = &file_entries[a_idx];
                let b = &file_entries[b_idx];
                if a.is_dir != b.is_dir {
                    b.is_dir.cmp(&a.is_dir) // Directories first
                } else {
                    a.name.cmp(&b.name)
                }
            });
            
            for &child_idx in &sorted_child_indices {
                let child_entry = &file_entries[child_idx];
                elements.extend(render_directory_entry_with_indices(
                    child_entry,
                    child_idx,
                    file_entries,
                    children_map,
                    expanded_directories,
                    depth + 1,
                ));
            }
        } else {
            // Directory is empty
            let placeholder = container(
                text("(empty)")
                    .size(12)
                    .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150)))
            )
            .padding(iced::Padding::new((depth + 1) as f32 * 20.0 + 20.0))
            .into();
            elements.push(placeholder);
        }
    }
    
    elements
}

fn placeholder_panel<'a>(label: &str) -> Element<'a, Message> {
    container(
        text(label)
            .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150)))
    )
    .center_y()
    .center_x()
    .width(Length::Fixed(250.0))
    .height(Length::Fill)
    .into()
}

fn status_bar<'a>(
    status_message: &'a str,
    error_message: Option<&'a String>,
    active_file_path: Option<&'a String>,
    file_count: usize,
) -> Element<'a, Message> {
    let error_widget: Element<_> = if let Some(err) = error_message {
        row![
            text(" | Error:").size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(255, 100, 100))),
            text(err).size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(255, 150, 150))),
        ]
        .spacing(4)
        .into()
    } else {
        horizontal_space().into()
    };

    // Determine file type for display
    let file_type = if let Some(path) = active_file_path {
        if path.ends_with(".rs") {
            "Rust"
        } else if path.ends_with(".toml") {
            "TOML"
        } else if path.ends_with(".md") {
            "Markdown"
        } else if path.ends_with(".json") {
            "JSON"
        } else if path.ends_with(".txt") {
            "Plain Text"
        } else {
            "Unknown"
        }
    } else {
        "Plain Text"
    };

    // Show a loading indicator if status contains "Loading"
    let status_indicator: Element<_> = if status_message.contains("Loading") || status_message.contains("loading") {
        row![
            text("⏳").size(12),
            text(status_message).size(12),
        ]
        .spacing(4)
        .into()
    } else {
        text(status_message).size(12).into()
    };

    row![
        text(format!("📁 {} files", file_count)).size(12),
        horizontal_space(),
        if let Some(path) = active_file_path {
            // Show only the file name, not the full path
            let file_name = path.split('/').last().unwrap_or(path);
            text(file_name).size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(180, 180, 255)))
        } else {
            text("No active file").size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150)))
        },
        horizontal_space(),
        status_indicator,
        error_widget,
        horizontal_space(),
        text("Ln 1, Col 1").size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150))),
        horizontal_space(),
        text(file_type).size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(200, 200, 100))),
    ]
    .padding([8, 16])
    .align_items(Alignment::Center)
    .into()
}
