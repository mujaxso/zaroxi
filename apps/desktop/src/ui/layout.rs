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
    editor_content: &'a str,
    is_dirty: bool,
    status_message: &'a str,
    error_message: Option<&'a String>,
    active_activity: Activity,
    ai_panel_visible: bool,
    prompt_input: &'a str,
    _expanded_directories: &'a std::collections::HashSet<String>,
    text_editor: &'a iced::widget::text_editor::Content,
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
        container(left_panel_with_expanded(file_entries, active_activity, _expanded_directories))
            .width(Length::FillPortion(2))
            .height(Length::Fill),
        vertical_rule(1),
        // Editor area - takes most space
        container(editor_panel(active_file_path, text_editor, is_dirty))
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
) -> Element<'a, Message> {
    match active_activity {
        Activity::Explorer => explorer_panel_with_expanded(file_entries, _expanded_directories),
        Activity::Search => search_panel(),
        Activity::SourceControl => terminal_panel(),
        Activity::Settings => settings_panel(),
        _ => placeholder_panel(&format!("{} panel", format!("{:?}", active_activity))),
    }
}

fn left_panel<'a>(
    file_entries: &'a [core_types::workspace::DirectoryEntry],
    active_activity: Activity,
) -> Element<'a, Message> {
    match active_activity {
        Activity::Explorer => explorer_panel(file_entries),
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
) -> Element<'a, Message> {
    let header = if let Some(path) = active_file_path {
        let status_text = if is_dirty {
            text("● Unsaved").size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(255, 165, 0)))
        } else {
            text("✓ Saved").size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(0, 200, 0)))
        };
        row![
            text(path)
                .size(14)
                .style(iced::theme::Text::Color(iced::Color::from_rgb8(200, 200, 200))),
            horizontal_space(),
            status_text,
        ]
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

    let editor_content = if active_file_path.is_some() {
        super::editor::editor(text_editor)
    } else {
        container(
            column![
                text("Neote").size(48).style(iced::theme::Text::Color(iced::Color::from_rgb8(100, 150, 255))),
                text("AI‑first IDE").size(20).style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 200))),
                container(iced::widget::horizontal_rule(1)).width(200),
                column![
                    button("Open a file from the explorer").style(theme::Button::Secondary),
                    button("Ask AI about the workspace").style(theme::Button::Secondary),
                    button("Create a new note").style(theme::Button::Secondary),
                    button("Review project structure").style(theme::Button::Secondary),
                ]
                .spacing(12)
                .padding(20),
            ]
            .align_items(Alignment::Center)
            .spacing(20),
        )
        .center_y()
        .center_x()
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
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
    _expanded_directories: &'a std::collections::HashSet<String>,
) -> Element<'a, Message> {
    // Filter files based on expanded directories
    // For simplicity, show all entries for now
    // In a real implementation, you would filter based on directory structure
    explorer_panel(file_entries)
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
        text(status_message).size(12),
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
