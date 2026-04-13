use iced::{Element, Length, Color, widget::{column, container, row, text}};
use syntax_core::Highlight;
use crate::message::Message;
use crate::state::{App, FileLoadingState};
use super::style::StyleHelpers;
use super::editor;
use crate::ui::icons::Icon;
use crate::theme::SemanticColors;


struct SyntaxIndicatorStyle {
    colors: SemanticColors,
    is_active: bool,
}

impl iced::widget::container::StyleSheet for SyntaxIndicatorStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(self.colors.elevated_panel_background.into()),
            border: iced::Border {
                color: if self.is_active { self.colors.accent } else { self.colors.border },
                width: 1.0,
                radius: 6.0.into(),
            },
            ..Default::default()
        }
    }
}

pub fn editor_panel(app: &App) -> Element<'_, Message> {
    let style = StyleHelpers::new(app.theme);
    
    let header = if let Some(active_path) = &app.active_file_path {
        let file_name = active_path.split('/').last().unwrap_or(active_path);
        container(
            row![
                Icon::File.render(&app.editor_typography, &style, Some(12)),
                text(file_name)
                    .size(11)
                    .style(iced::theme::Text::Color(style.colors.text_secondary)),
                iced::widget::horizontal_space(),
                // Show syntax highlight indicator - more visible
                if app.syntax_highlight_span_count > 0 {
                    container(
                        row![
                            text("●")
                                .size(10)
                                .style(iced::theme::Text::Color(style.colors.accent)),
                            text(format!("{} syntax spans", app.syntax_highlight_span_count))
                                .size(9)
                                .style(iced::theme::Text::Color(style.colors.accent)),
                        ]
                        .spacing(4)
                        .align_items(iced::Alignment::Center)
                    )
                    .padding([3, 8])
                    .style(iced::theme::Container::Custom(Box::new(SyntaxIndicatorStyle {
                        colors: style.colors,
                        is_active: true,
                    })))
                } else {
                    container(
                        row![
                            text("○")
                                .size(10)
                                .style(iced::theme::Text::Color(style.colors.text_muted)),
                            text("No syntax")
                                .size(9)
                                .style(iced::theme::Text::Color(style.colors.text_muted)),
                        ]
                        .spacing(4)
                        .align_items(iced::Alignment::Center)
                    )
                    .padding([3, 8])
                    .style(iced::theme::Container::Custom(Box::new(SyntaxIndicatorStyle {
                        colors: style.colors,
                        is_active: false,
                    })))
                },
                if app.is_dirty {
                    Icon::Warning.render_with_color(
                        &app.editor_typography,
                        style.colors.warning,
                        Some(10),
                    )
                } else {
                    Icon::Success.render_with_color(
                        &app.editor_typography,
                        style.colors.success,
                        Some(10),
                    )
                }
            ]
            .spacing(4)  // Less spacing
            .align_items(iced::Alignment::Center)
        )
        .padding([3, 8])  // Reasonable padding
        .width(Length::Fill)
        .style(iced::theme::Container::Custom(Box::new(move |_theme: &iced::Theme| {
            container::Appearance {
                background: Some(style.colors.editor_background.into()),  // Match editor background
                border: iced::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            }
        })))
    } else {
        container(
            row![
                Icon::File.render(&app.editor_typography, &style, Some(12)),
                text("No file selected")
                    .size(11)
                    .style(iced::theme::Text::Color(style.colors.text_muted)),
            ]
            .spacing(4)  // Less spacing
            .align_items(iced::Alignment::Center)
        )
        .padding([3, 8])  // Reasonable padding
        .width(Length::Fill)
        .style(iced::theme::Container::Custom(Box::new(move |_theme: &iced::Theme| {
            container::Appearance {
                background: Some(style.colors.editor_background.into()),  // Match editor background
                border: iced::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            }
        })))
    };
    
    let editor_content: Element<'_, Message> = if let Some(active_path) = &app.active_file_path {
        eprintln!("DEBUG: editor_panel: Rendering for active file: {}, is_file_too_large_for_editor={}, file_loading_state={:?}, text_editor.len={}", 
                 active_path, app.is_file_too_large_for_editor, app.file_loading_state, app.text_editor.text().len());
        
        // Check if we're in a loading state
        match &app.file_loading_state {
            FileLoadingState::LoadingMetadata { .. } |
            FileLoadingState::LoadingContent { .. } |
            FileLoadingState::LargeFileWarning { .. } |
            FileLoadingState::VeryLargeFileWarning { .. } |
            FileLoadingState::ReadOnlyPreview { .. } => {
                // Show loading indicator
                eprintln!("DEBUG: editor_panel: Showing loading state");
                container(
                    column![
                        text("Loading file...")
                            .size(16)
                            .style(iced::theme::Text::Color(style.colors.text_primary)),
                        text("Please wait")
                            .size(12)
                            .style(iced::theme::Text::Color(style.colors.text_secondary)),
                    ]
                    .spacing(12)
                    .align_items(iced::Alignment::Center)
                )
                .center_y()
                .center_x()
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
            }
            FileLoadingState::Idle => {
                // File is loaded, check if it's too large
                if app.is_file_too_large_for_editor {
                    eprintln!("DEBUG: editor_panel: File is marked as too large for editor - showing read-only message");
                    // Very large files (> 100 MB): show read-only preview with message
                    container(
                        column![
                            Icon::Warning.render_with_color(
                                &app.editor_typography,
                                style.colors.warning,
                                Some(24),
                            ),
                            text("File opened in read-only mode")
                                .size(16)
                                .style(iced::theme::Text::Color(style.colors.text_primary)),
                            text("This file is very large (> 100 MB). Editing is disabled for performance.")
                                .size(12)
                                .style(iced::theme::Text::Color(style.colors.text_secondary)),
                            text("You can view the first 100KB of the file.")
                                .size(12)
                                .style(iced::theme::Text::Color(style.colors.text_muted)),
                        ]
                        .spacing(12)
                        .align_items(iced::Alignment::Center)
                    )
                    .center_y()
                    .center_x()
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
                } else {
                    eprintln!("DEBUG: editor_panel: Showing editor for file (is_file_too_large_for_editor=false)");
                    // Use the interactive text editor (editable) with syntax highlighting
                    // Only pass the cache if it's ready (non-empty) and the file is loaded
                    // This prevents rendering with empty cache during file load
                    let version = app.syntax_cache_version;
                    let cache_ready = !app.syntax_highlight_cache.is_empty();
                    
                    eprintln!("DEBUG: editor_panel: syntax_highlight_cache has {} lines with {} total highlights, version {}, cache_ready={}", 
                             app.syntax_highlight_cache.len(),
                             app.syntax_highlight_cache.iter().map(|line| line.len()).sum::<usize>(),
                             version,
                             cache_ready);
                    eprintln!("DEBUG: editor_panel: text_editor text length: {}", app.text_editor.text().len());
                    
                    // Only pass the cache if it's ready
                    let line_cache = if cache_ready {
                        Some(app.syntax_highlight_cache.clone())
                    } else {
                        None
                    };
                    
                    editor::editor(
                        &app.text_editor,
                        &app.editor_typography,
                        style.colors.editor_background,
                        style.colors.text_primary,
                        line_cache,
                    )
                }
            }
        }
    } else {
        // Welcome screen
        container(
            column![
                text("Neote").size(32)
                    .style(iced::theme::Text::Color(style.colors.text_primary)),
                text("AI‑first IDE").size(16)
                    .style(iced::theme::Text::Color(style.colors.text_secondary)),
                container(iced::widget::horizontal_rule(1)).width(150),
                column![
                    text("Open a file from the explorer")
                        .style(iced::theme::Text::Color(style.colors.text_muted)),
                    text("Ask AI about the workspace")
                        .style(iced::theme::Text::Color(style.colors.text_muted)),
                ]
                .spacing(4)
                .padding(8),
            ]
            .spacing(8)
            .align_items(iced::Alignment::Center)
        )
        .center_y()
        .center_x()
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    };
    
    // Syntax highlight legend
    let legend = if !app.syntax_highlight_spans.is_empty() {
        use std::collections::HashSet;
        let mut unique_highlights = HashSet::new();
        for span in &app.syntax_highlight_spans {
            unique_highlights.insert(span.highlight);
        }
        let mut items = Vec::new();
        for &hl in unique_highlights.iter() {
            let color = match hl {
                Highlight::Comment => Color::from_rgb(0.5, 0.5, 0.5),
                Highlight::String => Color::from_rgb(0.0, 0.6, 0.0),
                Highlight::Keyword => Color::from_rgb(0.9, 0.2, 0.2),
                Highlight::Function => Color::from_rgb(0.0, 0.4, 0.8),
                Highlight::Variable => Color::from_rgb(0.8, 0.5, 0.0),
                Highlight::Type => Color::from_rgb(0.4, 0.2, 0.8),
                Highlight::Constant => Color::from_rgb(0.8, 0.0, 0.8),
                Highlight::Attribute => Color::from_rgb(0.2, 0.7, 0.7),
                Highlight::Operator => Color::from_rgb(0.7, 0.7, 0.2),
                Highlight::Number => Color::from_rgb(0.9, 0.6, 0.0),
                Highlight::Property => Color::from_rgb(0.2, 0.8, 0.5),
                Highlight::Namespace => Color::from_rgb(0.5, 0.5, 0.8),
                Highlight::Plain => Color::TRANSPARENT,
            };
            let item = container(iced::widget::Space::with_width(Length::Fixed(8.0)))
                .height(Length::Fixed(8.0))
                .style(iced::theme::Container::Custom(Box::new(move |_theme: &iced::Theme| {
                    container::Appearance {
                        background: Some(color.into()),
                        border: iced::Border {
                            color: style.colors.border,
                            width: 1.0,
                            radius: 1.0.into(),
                        },
                        ..Default::default()
                    }
                })));
            items.push(item.into());
        }
        container(
            row![
                text(format!("{} spans", app.syntax_highlight_span_count))
                    .size(9)
                    .style(iced::theme::Text::Color(style.colors.text_muted)),
                iced::widget::Space::with_width(Length::Fixed(4.0)),
                row(items).spacing(2)
            ]
            .align_items(iced::Alignment::Center)
        )
        .padding([2, 8])
        .width(Length::Fill)
        .style(iced::theme::Container::Custom(Box::new(move |_theme: &iced::Theme| {
            container::Appearance {
                background: Some(style.colors.elevated_panel_background.into()),
                border: iced::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            }
        })))
        .into()
    } else {
        container(iced::widget::Space::with_height(Length::Fixed(0.0)))
            .width(Length::Fill)
            .into()
    };

    // Create an editor panel with a border to match other panels
    // The panel should have a visible border like the explorer and assistant panels
    let mut column_children = vec![header.into()];
    
    // Add separator only when a file is opened
    if app.active_file_path.is_some() {
        // Add a very subtle separator line between header and editor content
        // Use a custom container for precise control
        let separator = container(iced::widget::Space::with_height(Length::Fixed(0.5)))
            .width(Length::Fill)
            .height(Length::Fixed(0.5))
            .style(iced::theme::Container::Custom(Box::new(move |_theme: &iced::Theme| {
                container::Appearance {
                    background: Some(style.colors.border.into()),  // Use divider color
                    border: iced::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: 0.0.into(),
                    },
                    ..Default::default()
                }
            })));
        column_children.push(separator.into());
    }
    
    // Add editor content - the editor widget has its own scrolling
    column_children.push(
        container(editor_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .clip(true) // Ensure content doesn't overflow
            .style(iced::theme::Container::Custom(Box::new(move |_theme: &iced::Theme| {
                container::Appearance {
                    background: Some(style.colors.editor_background.into()),
                    border: iced::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: 0.0.into(),
                    },
                    ..Default::default()
                }
            })))
            .into()
    );

    // Add syntax highlight legend
    column_children.push(legend);
    
    container(
        iced::widget::Column::with_children(column_children)
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(0)  // No spacing between header, separator, and editor
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .clip(true) // Ensure panel content doesn't overflow
    .style(iced::theme::Container::Custom(Box::new(move |_theme: &iced::Theme| {
        container::Appearance {
            background: Some(style.colors.editor_background.into()),
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        }
    })))
    .into()
}

