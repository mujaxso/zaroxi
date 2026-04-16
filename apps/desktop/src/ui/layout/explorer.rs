use iced::{
    widget::{button, column, container, horizontal_space, row, scrollable, text},
    Alignment, Element, Length,
};

use crate::message::Message;
use crate::state::Activity;
use core_types::workspace::DirectoryEntry;

// Helper function to normalize paths for consistent comparison
fn normalize_path(path: &str) -> String {
    use std::path::Path;
    let path = Path::new(path);
    // Remove trailing separators and normalize
    let mut normalized = path.to_string_lossy().to_string();
    // Remove trailing separator if present
    while normalized.ends_with(std::path::MAIN_SEPARATOR) {
        normalized.pop();
    }
    normalized
}

pub fn left_panel_with_expanded<'a>(
    file_entries: &'a [DirectoryEntry],
    active_activity: Activity,
    _expanded_directories: &'a std::collections::HashSet<String>,
    workspace_path: &'a str,
) -> Element<'a, Message> {
    match active_activity {
        Activity::Primary(crate::state::PrimarySidebarView::Explorer) => 
            explorer_panel_with_expanded(file_entries, _expanded_directories, workspace_path),
        Activity::Primary(crate::state::PrimarySidebarView::Search) => 
            search_panel(),
        Activity::Primary(crate::state::PrimarySidebarView::SourceControl) => 
            terminal_panel(),
        Activity::Primary(crate::state::PrimarySidebarView::Settings) => 
            settings_panel(),
        _ => placeholder_panel(&format!("{} panel", format!("{:?}", active_activity))),
    }
}

pub fn explorer_panel_with_expanded<'a>(
    file_entries: &'a [DirectoryEntry],
    expanded_directories: &'a std::collections::HashSet<String>,
    workspace_path: &'a str,
) -> Element<'a, Message> {
    // Always show at least a flat list
    let content: Element<_> = if file_entries.is_empty() {
        container(
            column![
                text("No files in workspace")
                    .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150))),
                button("Open Workspace")
                    .on_press(Message::OpenWorkspace)
                    .padding(8)
                    .style(iced::theme::Button::Secondary),
            ]
            .spacing(10)
            .align_items(Alignment::Center)
        )
        .center_y()
        .center_x()
        .height(Length::Fill)
        .into()
    } else {
        // Always use simple approach for now to ensure something is shown
        let mut elements = Vec::new();
        
        // Sort entries by name for better display
        let mut sorted_entries = file_entries.to_vec();
        sorted_entries.sort_by(|a, b| {
            // Directories first, then by name
            if a.is_dir != b.is_dir {
                b.is_dir.cmp(&a.is_dir) // Directories first
            } else {
                a.name.cmp(&b.name)
            }
        });
        
        for entry in &sorted_entries {
            let normalized_path = normalize_path(&entry.path);
            let is_expanded = expanded_directories.contains(&normalized_path);
            
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
            
            // Simple depth calculation: count path separators
            // Remove workspace path from the entry path to get relative depth
            let depth = if workspace_path.is_empty() {
                entry.path.matches(std::path::MAIN_SEPARATOR).count()
            } else {
                let workspace_normalized = normalize_path(workspace_path);
                let entry_normalized = normalize_path(&entry.path);
                if entry_normalized.starts_with(&workspace_normalized) {
                    let relative = &entry_normalized[workspace_normalized.len()..];
                    relative.matches(std::path::MAIN_SEPARATOR).count()
                } else {
                    entry.path.matches(std::path::MAIN_SEPARATOR).count()
                }
            };
            
            let padding_left = depth * 20;
            
            let entry_element = container(
                button(
                    row![
                        if entry.is_dir {
                            let chevron = if is_expanded { "▼" } else { "▶" };
                            text(chevron).size(12)
                                .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150)))
                        } else {
                            text("  ").size(12)
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
                .style(iced::theme::Button::Secondary),
            )
            .padding(iced::Padding::new(padding_left as f32))
            .into();
            
            elements.push(entry_element);
        }
        
        if elements.is_empty() {
            container(
                text("No files to display")
                    .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150)))
            )
            .center_y()
            .center_x()
            .height(Length::Fill)
            .into()
        } else {
            scrollable(
                column(elements)
                    .spacing(2),
            )
            .height(Length::Fill)
            .into()
        }
    };

    column![
        row![
            text("EXPLORER").size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150))),
            horizontal_space(),
            button("Refresh")
                .on_press(Message::RefreshWorkspace)
                .padding([4, 8])
                .style(iced::theme::Button::Secondary),
        ]
        .padding([12, 16])
        .align_items(Alignment::Center),
        iced::widget::horizontal_rule(1),
        content,
    ]
    .height(Length::Fill)
    .into()
}

fn search_panel<'a>() -> Element<'a, Message> {
    super::search::search_panel()
}

fn terminal_panel<'a>() -> Element<'a, Message> {
    super::terminal::terminal_panel()
}

fn settings_panel<'a>() -> Element<'a, Message> {
    super::settings::settings_panel()
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
