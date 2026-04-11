use iced::{
    widget::{
        button, column, container, horizontal_space, row, scrollable, text,
        text_input, vertical_rule, Space,
    },
    Alignment, Element, Length,
};

use crate::state::{Activity, FileLoadingState};
use crate::message::Message;
use crate::theme::NeoteTheme;
use crate::ui::style::StyleHelpers;
use crate::settings::editor::EditorTypographySettings;
use crate::ui::icons::Icon;

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
    editor_document: Option<&'a editor_core::Document>,
    is_file_too_large_for_editor: bool,
    file_loading_state: &'a FileLoadingState,
    theme: NeoteTheme,
    editor_typography: &'a EditorTypographySettings,
) -> Element<'a, Message> {
    let style = StyleHelpers::new(theme);
    
    // Top bar
    let top_bar = top_bar(workspace_path, is_dirty);

    // Activity rail
    let activity_rail = activity_rail(active_activity);

    // Main content area
    let ai_panel_widget: Element<_> = if ai_panel_visible {
        container(ai_panel(prompt_input))
            .width(Length::FillPortion(3))
            .height(Length::Fill)
            .into()
    } else {
        container(Space::new(Length::Fixed(0.0), Length::Fixed(0.0)))
            .width(Length::Fixed(0.0))
            .into()
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
        container(editor_panel(active_file_path, text_editor, is_dirty, editor_document, is_file_too_large_for_editor, file_loading_state, editor_typography))
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
        .style(iced::theme::Container::Custom(Box::new(move |_theme: &iced::Theme| {
            container::Appearance {
                background: Some(style.colors.app_background.into()),
                border: iced::Border::default(),
                ..Default::default()
            }
        })))
        .into()
}

fn top_bar<'a>(workspace_path: &'a str, is_dirty: bool) -> Element<'a, Message> {
    // Use theme colors from the outer scope
    // Since we can't access them directly, we'll use a more IDE-like approach
    // For now, use colors that match IDE conventions
    let status_indicator: Element<_> = if is_dirty {
        row![
            text("●").size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(255, 180, 0))),
            text("Unsaved").size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(220, 220, 220)))
        ]
        .spacing(4)
        .align_items(Alignment::Center)
        .into()
    } else {
        row![
            text("✓").size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(0, 200, 100))),
            text("Saved").size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(180, 180, 180)))
        ]
        .spacing(4)
        .align_items(Alignment::Center)
        .into()
    };

    row![
        // Logo/brand
        row![
            text("N").size(20).style(iced::theme::Text::Color(iced::Color::from_rgb8(100, 160, 255))),
            text("eote").size(20).style(iced::theme::Text::Color(iced::Color::from_rgb8(220, 220, 230))),
        ]
        .spacing(0)
        .align_items(Alignment::Center),
        horizontal_space(),
        // Workspace path display with manual entry option
        if workspace_path.is_empty() {
            // When no workspace is open, show an input field for manual entry
            container(
                row![
                    text_input("Enter workspace path manually...", workspace_path)
                        .on_input(Message::WorkspacePathChanged)
                        .on_submit(Message::SubmitManualWorkspacePath(workspace_path.to_string()))
                        .padding([10, 12])
                        .width(Length::Fill)
                        .style(iced::theme::TextInput::Default),
                    button("Open")
                        .on_press(Message::SubmitManualWorkspacePath(workspace_path.to_string()))
                        .padding([10, 14])
                        .style(iced::theme::Button::Secondary),
                ]
                .spacing(8)
                .align_items(Alignment::Center)
            )
            .width(Length::FillPortion(3))
            .into()
        } else {
            // When workspace is open, show it as read-only
            container(
                container(
                    text(workspace_path)
                        .size(14)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(220, 220, 220)))
                )
                .padding([10, 12])
                .width(Length::Fill)
                .style(iced::theme::Container::Box)
            )
            .width(Length::FillPortion(3))
            .style(iced::theme::Container::Box)
        },
        // Buttons
        row![
            button("Open Workspace...")
                .on_press(Message::OpenWorkspace)
                .padding([8, 14])
                .style(iced::theme::Button::Secondary),
            button("Refresh")
                .on_press(Message::RefreshWorkspace)
                .padding([8, 14])
                .style(iced::theme::Button::Secondary),
        ]
        .spacing(8),
        horizontal_space(),
        // Status indicator
        container(status_indicator)
            .padding([6, 12])
            .style(iced::theme::Container::Box),
        // Save button
        button("Save")
            .on_press(Message::SaveFile)
            .padding([10, 18])
            .style(iced::theme::Button::Primary),
    ]
    .padding([12, 20])
    .align_items(Alignment::Center)
    .into()
}

fn activity_rail<'a>(active_activity: Activity) -> Element<'a, Message> {
    // Define activities with their corresponding Activity enum values and icons
    let activities = [
        (Activity::explorer(), Icon::Folder, "Explorer"),
        (Activity::search(), Icon::Search, "Search"),
        (Activity::ai_assistant(), Icon::Robot, "AI"),
        (Activity::source_control(), Icon::Git, "Git"),
        (Activity::settings(), Icon::Settings, "Settings"),
    ];

    let children: Vec<Element<_>> = activities
        .iter()
        .map(|&(activity, icon, label)| {
            let is_active = activity == active_activity;
            let button_style = if is_active {
                iced::theme::Button::Primary
            } else {
                iced::theme::Button::Secondary
            };
            // For AI activity, we want to toggle the panel visibility
            let message = if activity == Activity::ai_assistant() {
                Message::ToggleAiPanel
            } else {
                Message::ActivitySelected(activity)
            };
            
            // Active indicator - More visible for IDE
            let active_indicator: Element<_> = if is_active {
                struct ActiveIndicatorStyle;
                impl iced::widget::container::StyleSheet for ActiveIndicatorStyle {
                    type Style = iced::Theme;
                            
                    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
                        iced::widget::container::Appearance {
                            background: Some(iced::Color::from_rgb(0.35, 0.65, 1.0).into()),
                            border: iced::Border::default(),
                            shadow: Default::default(),
                            text_color: None,
                        }
                    }
                }
                let container = container(
                    iced::widget::Space::new(Length::Fixed(3.0), Length::Fixed(32.0))
                )
                .style(iced::theme::Container::Custom(Box::new(ActiveIndicatorStyle)))
                .width(Length::Fixed(3.0))
                .height(Length::Fixed(32.0));
                container.into()
            } else {
                iced::widget::Space::new(Length::Fixed(0.0), Length::Fixed(0.0)).into()
            };
            
            // Create icon element
            // We need to create a temporary typography settings for the icon
            // Since we don't have access to app here, we'll use defaults
            let typography = EditorTypographySettings::default();
            let style = StyleHelpers::new(NeoteTheme::Dark);
            let icon_color = if is_active {
                style.colors.accent
            } else {
                style.colors.text_muted
            };
            let icon_element = icon.render_with_color(&typography, icon_color, Some(18));
            
            container(
                row![
                    active_indicator,
                    container(
                        button(
                            column![
                                icon_element,
                                text(label).size(11),
                            ]
                            .align_items(Alignment::Center)
                            .spacing(2),
                        )
                        .on_press(message)
                        .padding([12, 8])
                        .style(button_style)
                    )
                    .width(Length::Fixed(64.0))
                    .center_x()
                ]
                .align_items(Alignment::Center)
            )
            .width(Length::Fixed(70.0))
            .into()
        })
        .collect();

    column(children)
        .spacing(4)
        .padding([16, 4])
        .into()
}

fn left_panel_with_expanded<'a>(
    file_entries: &'a [core_types::workspace::DirectoryEntry],
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


fn explorer_panel<'a>(file_entries: &'a [core_types::workspace::DirectoryEntry]) -> Element<'a, Message> {
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
                                    .style(iced::theme::Button::Text)
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
                    .style(iced::theme::Button::Secondary),
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
        container(
            row![
                text("EXPLORER").size(11)
                    .style(iced::theme::Text::Color(iced::Color::from_rgb8(160, 160, 170))),
                horizontal_space(),
                button("⟳")
                    .on_press(Message::RefreshWorkspace)
                    .padding([6, 8])
                    .style(iced::theme::Button::Secondary),
            ]
            .align_items(Alignment::Center)
        )
        .padding([14, 16])
        .width(Length::Fill),
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
    editor_document: Option<&'a editor_core::Document>,
    is_file_too_large_for_editor: bool,
    file_loading_state: &'a FileLoadingState,
    editor_typography: &'a EditorTypographySettings,
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
        if let Some(document) = editor_document {
            if document.is_very_large() {
                status_elements.push(
                    text("⚠ Very Large (Limited Editing)")
                        .size(12)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(255, 100, 100)))
                        .into()
                );
                status_elements.push(horizontal_space().width(Length::Fixed(10.0)).into());
            } else if document.is_large() {
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
        FileLoadingState::LoadingMetadata { path } => {
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
        FileLoadingState::LoadingContent { path, size } => {
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
        FileLoadingState::LargeFileWarning { path, size } => {
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
        FileLoadingState::VeryLargeFileWarning { path, size } => {
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
        FileLoadingState::ReadOnlyPreview { path, size } => {
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
        FileLoadingState::Idle => {
            // Original logic for when not loading
            if active_file_path.is_some() {
                if is_file_too_large_for_editor {
                    // Show read-only text view for very large files
                    let content = if let Some(document) = editor_document {
                        if document.is_very_large() {
                            // For very large files, show only first 100KB
                            // Use safe slicing with bounds checking
                            let len = document.len_chars();
                            let end = 100_000.min(len);
                            document.slice(0, end).unwrap_or_else(|_| String::new())
                        } else {
                            // For large but not very large files, limit to 500KB
                            let text = document.text();
                            if text.len() > 500_000 {
                                text[..500_000].to_string()
                            } else {
                                text
                            }
                        }
                    } else {
                        String::new()
                    };
                    let warning = if let Some(document) = editor_document {
                        if document.is_very_large() {
                            format!("\n\n--- File truncated ({} MB total, showing first 100KB) ---", 
                                   document.len_chars() / 1_000_000)
                        } else if document.is_large() {
                            format!("\n\n--- File truncated ({} MB total, showing first 500KB) ---",
                                   document.len_chars() / 1_000_000)
                        } else {
                            String::new()
                        }
                    } else {
                        String::new()
                    };
                    // Use a scrollable with explicit height
                    scrollable(
                        container(
                            text(content + &warning)
                                .font(iced::Font::MONOSPACE)
                                .size(14)
                        )
                        .padding(16)
                        .width(Length::Fill)
                    )
                    .height(Length::Fill)
                    .into()
                } else {
                    super::editor::editor(text_editor, editor_typography)
                }
            } else {
                container(
                    column![
                        text("Neote").size(32).style(iced::theme::Text::Color(iced::Color::from_rgb8(100, 150, 255))),
                        text("AI‑first IDE").size(16).style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 200))),
                        container(iced::widget::horizontal_rule(1)).width(150),
                        column![
                            button("Open a file from the explorer").style(iced::theme::Button::Secondary),
                            button("Ask AI about the workspace").style(iced::theme::Button::Secondary),
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
        container(
            row![
                text("AI ASSISTANT").size(11)
                    .style(iced::theme::Text::Color(iced::Color::from_rgb8(160, 160, 170))),
                horizontal_space(),
                button("⋯")
                    .on_press(Message::PromptInputChanged("AI options".to_string()))
                    .padding([6, 8])
                    .style(iced::theme::Button::Secondary),
            ]
            .align_items(Alignment::Center)
        )
        .padding([14, 16])
        .width(Length::Fill),
        iced::widget::horizontal_rule(1),
        scrollable(
            column![
                // Welcome card - More solid for IDE
                {
                    struct WelcomeCardStyle;
                    impl iced::widget::container::StyleSheet for WelcomeCardStyle {
                        type Style = iced::Theme;
                
                        fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
                            iced::widget::container::Appearance {
                                background: Some(iced::Color::from_rgb(0.12, 0.12, 0.16).into()),
                                border: iced::Border {
                                    color: iced::Color::from_rgb(0.25, 0.45, 0.85),
                                    width: 1.0,
                                    radius: 8.0.into(),
                                },
                                shadow: Default::default(),
                                text_color: None,
                            }
                        }
                    }
                    container(
                        column![
                            row![
                                Icon::Robot.render_with_color(
                                    &EditorTypographySettings::default(),
                                    iced::Color::from_rgb8(220, 220, 255),
                                    Some(20)
                                ),
                                text("Neote AI").size(16).style(iced::theme::Text::Color(iced::Color::from_rgb8(220, 220, 255))),
                            ]
                            .spacing(8)
                            .align_items(Alignment::Center),
                            text("Ask questions about your code, get explanations, refactor suggestions, and more.")
                                .size(13)
                                .style(iced::theme::Text::Color(iced::Color::from_rgb8(180, 190, 210))),
                        ]
                        .spacing(10)
                        .padding(20)
                    )
                    .style(iced::theme::Container::Custom(Box::new(WelcomeCardStyle)))
                },
                // Quick actions
                container(
                    column![
                        text("Quick Actions").size(13)
                            .style(iced::theme::Text::Color(iced::Color::from_rgb8(160, 160, 170))),
                        column![
                            button("Explain this file")
                                .on_press(Message::PromptInputChanged("Explain the current file".to_string()))
                                .padding([10, 12])
                                .width(Length::Fill)
                                .style(iced::theme::Button::Secondary),
                            button("Refactor selection")
                                .on_press(Message::PromptInputChanged("Refactor the selected code".to_string()))
                                .padding([10, 12])
                                .width(Length::Fill)
                                .style(iced::theme::Button::Secondary),
                            button("Find bugs")
                                .on_press(Message::PromptInputChanged("Find potential bugs in this code".to_string()))
                                .padding([10, 12])
                                .width(Length::Fill)
                                .style(iced::theme::Button::Secondary),
                            button("Write tests")
                                .on_press(Message::PromptInputChanged("Write unit tests for this code".to_string()))
                                .padding([10, 12])
                                .width(Length::Fill)
                                .style(iced::theme::Button::Secondary),
                        ]
                        .spacing(6),
                    ]
                    .spacing(12)
                    .padding(16)
                )
                .style(iced::theme::Container::Box),
                // Info note
                container(
                    text("AI features are coming soon. This is a placeholder for the AI assistant interface.")
                        .size(11)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 160)))
                )
                .padding(16),
            ]
            .spacing(16)
            .padding([0, 16])
        )
        .height(Length::Fill),
        iced::widget::horizontal_rule(1),
        container(
            row![
                text_input("Ask Neote AI...", prompt_input)
                    .on_input(Message::PromptInputChanged)
                    .padding([12, 14])
                    .width(Length::Fill),
                button("Send")
                    .on_press(Message::SendPrompt)
                    .padding([12, 18])
                    .style(iced::theme::Button::Primary),
            ]
            .align_items(Alignment::Center)
        )
        .padding([12, 16])
        .width(Length::Fill),
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
                .style(iced::theme::Button::Secondary),
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
                    .style(iced::theme::Button::Primary)
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
    // For now, return a placeholder since terminal module might not be updated
    container(
        text("Terminal placeholder")
            .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150)))
    )
    .center_y()
    .center_x()
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn settings_panel<'a>() -> Element<'a, Message> {
    column![
        row![
            text("SETTINGS").size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150))),
            horizontal_space(),
            button("Save")
                .on_press(Message::PromptInputChanged("Settings saved".to_string()))
                .style(iced::theme::Button::Primary),
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
                            .style(iced::theme::Button::Secondary),
                        button("Light")
                            .on_press(Message::PromptInputChanged("Theme set to Light".to_string()))
                            .style(iced::theme::Button::Secondary),
                    ]
                    .spacing(8)
                    .padding(16)
                )
                .style(iced::theme::Container::Box),
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
                .style(iced::theme::Container::Box),
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

pub fn explorer_panel_with_expanded<'a>(
    file_entries: &'a [core_types::workspace::DirectoryEntry],
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
        struct ErrorWidgetStyle;
        impl iced::widget::container::StyleSheet for ErrorWidgetStyle {
            type Style = iced::Theme;
            
            fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
                iced::widget::container::Appearance {
                    background: Some(iced::Color::from_rgb(0.25, 0.08, 0.08).into()),
                    border: iced::Border {
                        color: iced::Color::from_rgb(0.8, 0.2, 0.2),
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    shadow: Default::default(),
                    text_color: None,
                }
            }
        }
        container(
            row![
                text("⚠").size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(255, 100, 100))),
                text(err).size(11).style(iced::theme::Text::Color(iced::Color::from_rgb8(255, 180, 180))),
            ]
            .spacing(6)
            .align_items(Alignment::Center)
        )
        .padding([4, 8])
        .style(iced::theme::Container::Custom(Box::new(ErrorWidgetStyle)))
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
            text(status_message).size(11),
        ]
        .spacing(6)
        .align_items(Alignment::Center)
        .into()
    } else {
        container(
            text(status_message).size(11)
        )
        .padding([4, 8])
        .style(iced::theme::Container::Box)
        .into()
    };

    row![
        // File count
        container(
            row![
                text("📁").size(12),
                text(format!("{}", file_count)).size(11)
                    .style(iced::theme::Text::Color(iced::Color::from_rgb8(180, 180, 200))),
            ]
            .spacing(6)
            .align_items(Alignment::Center)
        )
        .padding([4, 8])
        .style(iced::theme::Container::Box),
        horizontal_space(),
        // Active file
        {
            let container_widget: Element<_> = if let Some(path) = active_file_path {
                let file_name = path.split('/').last().unwrap_or(path);
                container(
                    text(file_name).size(11)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(180, 180, 255)))
                )
                .padding([4, 8])
                .style(iced::theme::Container::Box)
                .into()
            } else {
                container(
                    text("No active file").size(11)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 170)))
                )
                .padding([4, 8])
                .style(iced::theme::Container::Box)
                .into()
            };
            container_widget
        },
        horizontal_space(),
        // Status indicator
        status_indicator,
        // Error widget
        error_widget,
        horizontal_space(),
        // Line and column
        container(
            text("Ln 1, Col 1").size(11)
                .style(iced::theme::Text::Color(iced::Color::from_rgb8(160, 160, 180)))
        )
        .padding([4, 8])
        .style(iced::theme::Container::Box),
        // File type
        container(
            text(file_type).size(11)
                .style(iced::theme::Text::Color(iced::Color::from_rgb8(200, 200, 120)))
        )
        .padding([4, 8])
        .style(iced::theme::Container::Box),
    ]
    .padding([8, 12])
    .align_items(Alignment::Center)
    .into()
}
