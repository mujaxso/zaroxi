use iced::{
    widget::{
        button, column, container, horizontal_space, row, scrollable, text,
        text_input, vertical_rule,
    },
    Alignment, Element, Length, Theme,
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
) -> Element<'a, Message> {
    // Top bar
    let top_bar = top_bar(workspace_path, is_dirty);

    // Activity rail
    let activity_rail = activity_rail(active_activity);

    // Main content area
    let main_content = row![
        // Left panel (explorer)
        left_panel(file_entries, active_activity),
        vertical_rule(1),
        // Editor area
        editor_panel(active_file_path, editor_content, is_dirty),
        // AI panel (conditionally visible)
        if ai_panel_visible {
            container(ai_panel(prompt_input))
                .width(Length::Fixed(320.0))
                .height(Length::Fill)
                .style(|theme| container::Style {
                    background: Some(
                        iced::Background::Color(
                            theme.extended_palette().background.weak.color
                        )
                    ),
                    border: iced::Border {
                        width: 0.0,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .into()
        } else {
            container(horizontal_space()).width(Length::Fixed(0.0)).into()
        }
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
        .style(|theme| container::Style {
            background: Some(iced::Background::Color(
                theme.extended_palette().background.base.color,
            )),
            ..Default::default()
        })
        .into()
}

fn top_bar<'a>(workspace_path: &'a str, is_dirty: bool) -> Element<'a, Message> {
    row![
        text("Neote").size(20).style(iced::Color::from_rgb8(100, 150, 255)),
        horizontal_space(),
        text_input("Workspace path", workspace_path)
            .on_input(Message::WorkspacePathChanged)
            .padding(8)
            .width(Length::Fixed(400.0))
            .style(|theme| text_input::Style {
                background: iced::Background::Color(
                    theme.extended_palette().background.weak.color
                ),
                border: iced::Border {
                    width: 1.0,
                    radius: 4.0.into(),
                    color: theme.extended_palette().background.strong.color,
                },
                ..Default::default()
            }),
        button("Open")
            .on_press(Message::OpenWorkspace)
            .padding([8, 12])
            .style(|theme| button::secondary(theme)),
        button("Refresh")
            .on_press(Message::RefreshWorkspace)
            .padding([8, 12])
            .style(|theme| button::secondary(theme)),
        horizontal_space(),
        if is_dirty {
            row![
                text("●").size(14).style(iced::Color::from_rgb8(255, 165, 0)),
                text("Unsaved").size(14)
            ]
            .spacing(4)
            .align_items(Alignment::Center)
            .into()
        } else {
            row![
                text("✓").size(14).style(iced::Color::from_rgb8(0, 200, 0)),
                text("Saved").size(14)
            ]
            .spacing(4)
            .align_items(Alignment::Center)
            .into()
        },
        button("Save")
            .on_press(Message::SaveFile)
            .padding([8, 16])
            .style(|theme| button::primary(theme)),
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

    column(
        activities
            .iter()
            .map(|&(activity, icon, label)| {
                let is_active = activity == active_activity;
                let button_style = if is_active {
                    button::primary
                } else {
                    button::secondary
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
                    .on_press(Message::ActivitySelected(activity))
                    .padding(12)
                    .style(button_style),
                )
                .width(Length::Fixed(70.0))
                .center_x()
                .into()
            })
            .collect(),
    )
    .spacing(8)
    .padding([16, 8])
    .into()
}

fn left_panel<'a>(
    file_entries: &'a [core_types::workspace::DirectoryEntry],
    active_activity: Activity,
) -> Element<'a, Message> {
    match active_activity {
        Activity::Explorer => explorer_panel(file_entries),
        _ => placeholder_panel(&format!("{} panel", format!("{:?}", active_activity))),
    }
}

fn explorer_panel<'a>(file_entries: &'a [core_types::workspace::DirectoryEntry]) -> Element<'a, Message> {
    column![
        row![
            text("EXPLORER").size(12).style(iced::Color::from_rgb8(150, 150, 150)),
            horizontal_space(),
            button("⋯").style(|theme| button::secondary(theme)),
        ]
        .padding([12, 16])
        .align_items(Alignment::Center),
        iced::widget::horizontal_rule(1),
        if file_entries.is_empty() {
            container(
                text("No files in workspace")
                    .style(iced::Color::from_rgb8(150, 150, 150))
            )
            .center_y()
            .center_x()
            .height(Length::Fill)
            .into()
        } else {
            scrollable(
                column(
                    file_entries
                        .iter()
                        .enumerate()
                        .map(|(i, entry)| {
                            let is_file = !entry.is_dir;
                            let icon = if is_file { "📄" } else { "📁" };
                            container(
                                button(
                                    row![
                                        text(icon).size(14),
                                        text(&entry.name).size(14),
                                    ]
                                    .spacing(8)
                                    .align_items(Alignment::Center),
                                )
                                .on_press(Message::FileSelected(i))
                                .padding([6, 12])
                                .width(Length::Fill)
                                .style(|theme| button::secondary(theme)),
                            )
                            .into()
                        })
                        .collect(),
                )
                .spacing(2),
            )
            .height(Length::Fill)
            .into()
        }
    ]
    .width(Length::Fixed(250.0))
    .height(Length::Fill)
    .into()
}

fn editor_panel<'a>(
    active_file_path: Option<&'a String>,
    editor_content: &'a str,
    is_dirty: bool,
) -> Element<'a, Message> {
    let header = if let Some(path) = active_file_path {
        row![
            text(path)
                .size(14)
                .style(iced::Color::from_rgb8(200, 200, 200)),
            horizontal_space(),
            if is_dirty {
                text("● Unsaved").size(12).style(iced::Color::from_rgb8(255, 165, 0))
            } else {
                text("✓ Saved").size(12).style(iced::Color::from_rgb8(0, 200, 0))
            },
        ]
        .padding([12, 16])
        .align_items(Alignment::Center)
    } else {
        row![
            text("No file selected").size(14).style(iced::Color::from_rgb8(150, 150, 150)),
            horizontal_space(),
        ]
        .padding([12, 16])
        .align_items(Alignment::Center)
    };

    let editor_content = if active_file_path.is_some() {
        super::editor::editor(editor_content)
    } else {
        container(
            column![
                text("Neote").size(48).style(iced::Color::from_rgb8(100, 150, 255)),
                text("AI‑first IDE").size(20).style(iced::Color::from_rgb8(150, 150, 200)),
                iced::widget::horizontal_rule(1).width(200),
                column![
                    button("Open a file from the explorer").style(|theme| button::secondary(theme)),
                    button("Ask AI about the workspace").style(|theme| button::secondary(theme)),
                    button("Create a new note").style(|theme| button::secondary(theme)),
                    button("Review project structure").style(|theme| button::secondary(theme)),
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
            text("AI ASSISTANT").size(12).style(iced::Color::from_rgb8(150, 150, 150)),
            horizontal_space(),
            button("⋯").style(|theme| button::secondary(theme)),
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
                            .style(iced::Color::from_rgb8(180, 180, 180)),
                    ]
                    .padding(16)
                    .spacing(8)
                )
                .style(|theme| container::Style {
                    background: Some(iced::Background::Color(
                        theme.extended_palette().background.weak.color
                    )),
                    border: iced::Border {
                        width: 1.0,
                        radius: 8.0.into(),
                        color: theme.extended_palette().background.strong.color,
                    },
                    ..Default::default()
                }),
                column![
                    button("Explain this file").on_press(Message::SendPrompt).padding(12),
                    button("Refactor selection").on_press(Message::SendPrompt).padding(12),
                    button("Find bugs").on_press(Message::SendPrompt).padding(12),
                    button("Write tests").on_press(Message::SendPrompt).padding(12),
                ]
                .spacing(8)
                .padding(16),
                container(
                    text("AI features are coming soon. This is a placeholder for the AI assistant interface.")
                        .size(12)
                        .style(iced::Color::from_rgb8(150, 150, 150))
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
                .style(|theme| button::primary(theme)),
        ]
        .padding([8, 16])
        .align_items(Alignment::Center),
    ]
    .height(Length::Fill)
    .into()
}

fn placeholder_panel<'a>(label: &str) -> Element<'a, Message> {
    container(
        text(label)
            .style(iced::Color::from_rgb8(150, 150, 150))
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
    row![
        text(format!("{} files", file_count)).size(12),
        horizontal_space(),
        if let Some(path) = active_file_path {
            text(path).size(12).style(iced::Color::from_rgb8(180, 180, 255))
        } else {
            text("No active file").size(12).style(iced::Color::from_rgb8(150, 150, 150))
        },
        horizontal_space(),
        text(status_message).size(12),
        if let Some(err) = error_message {
            row![
                text(" | Error:").size(12).style(iced::Color::from_rgb8(255, 100, 100)),
                text(err).size(12).style(iced::Color::from_rgb8(255, 150, 150)),
            ]
            .spacing(4)
            .into()
        } else {
            horizontal_space().into()
        },
        horizontal_space(),
        text("Ln 1, Col 1").size(12).style(iced::Color::from_rgb8(150, 150, 150)),
        horizontal_space(),
        text("Plain Text").size(12).style(iced::Color::from_rgb8(150, 150, 150)),
    ]
    .padding([8, 16])
    .align_items(Alignment::Center)
    .into()
}
