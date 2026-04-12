use iced::{Element, Length, Color, widget::{column, container, row, text, horizontal_rule, Space, text::Span}};
use syntax_core::Highlight;
use crate::message::Message;
use crate::state::App;
use super::style::StyleHelpers;
use super::editor;
use crate::ui::icons::Icon;

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
                // Show syntax highlight indicator
                container(
                    text(format!("{} spans", app.syntax_highlight_span_count))
                        .size(9)
                        .style(iced::theme::Text::Color(style.colors.accent)),
                )
                .padding([2, 4])
                .style(iced::theme::Container::Custom(Box::new(move |_theme: &iced::Theme| {
                    container::Appearance {
                        background: Some(style.colors.elevated_panel_background.into()),
                        border: iced::Border {
                            color: style.colors.border,
                            width: 1.0,
                            radius: 2.0.into(),
                        },
                        ..Default::default()
                    }
                }))),
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
    
    let editor_content = if let Some(_) = &app.active_file_path {
        if app.is_file_too_large_for_editor {
            // Show a message for large files instead of the editor
            container(
                column![
                    Icon::Warning.render_with_color(
                        &app.editor_typography,
                        style.colors.warning,
                        Some(24),
                    ),
                    text("File is too large for editing")
                        .size(16)
                        .style(iced::theme::Text::Color(style.colors.text_primary)),
                    text("The file is too large to open in the editor for performance reasons.")
                        .size(12)
                        .style(iced::theme::Text::Color(style.colors.text_secondary)),
                    text("Consider using a different tool for very large files.")
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
            // Use the existing editor component with typography settings
            editor::editor(&app.text_editor, &app.editor_typography, style.colors.editor_background)
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
                    background: Some(style.colors.divider.into()),  // Use divider color
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
    
    // Add syntax preview if we have highlight spans
    if !app.syntax_highlight_spans.is_empty() {
        // Show a small colored preview (first 500 chars)
        let preview = syntax_preview(&app.syntax_highlight_spans, &app.text_editor.text(), &style);
        column_children.push(
            container(preview)
                .padding(8)
                .width(Length::Fill)
                .style(iced::theme::Container::Custom(Box::new(move |_theme: &iced::Theme| {
                    container::Appearance {
                        background: Some(style.colors.elevated_panel_background.into()),
                        border: iced::Border {
                            color: style.colors.border,
                            width: 1.0,
                            radius: 4.0.into(),
                        },
                        ..Default::default()
                    }
                })))
                .into()
        );
    }
    
    // Add editor content
    column_children.push(
        container(editor_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .clip(true) // Ensure content doesn't overflow
            .into()
    );
    
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
                color: style.colors.border,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        }
    })))
    .into()
}

/// Create a colored preview of the first part of the document using highlight spans.
fn syntax_preview<'a>(
    spans: &'a [syntax_core::HighlightSpan],
    source: &'a str,
    style: &'a StyleHelpers,
) -> Element<'a, Message> {
    // Take the first 500 characters (or less)
    const PREVIEW_LEN: usize = 500;
    let preview_source = if source.len() > PREVIEW_LEN {
        &source[..PREVIEW_LEN]
    } else {
        source
    };
    
    // Map Highlight to color using theme
    let highlight_to_color = |hl: Highlight| -> Color {
        match hl {
            Highlight::Comment => style.colors.text_muted,
            Highlight::String => Color::from_rgb(0.8, 0.6, 0.2), // orange-ish
            Highlight::Keyword => style.colors.accent,
            Highlight::Function => Color::from_rgb(0.2, 0.8, 0.8), // cyan
            Highlight::Variable => style.colors.text_primary,
            Highlight::Type => Color::from_rgb(0.4, 0.8, 0.4), // green
            Highlight::Constant => Color::from_rgb(0.8, 0.4, 0.8), // magenta
            Highlight::Attribute => Color::from_rgb(0.8, 0.8, 0.2), // yellow
            Highlight::Operator => style.colors.text_secondary,
            Highlight::Number => Color::from_rgb(0.8, 0.5, 0.2), // orange
            Highlight::Property => Color::from_rgb(0.6, 0.6, 1.0), // light blue
            Highlight::Namespace => Color::from_rgb(0.6, 0.4, 0.8), // purple
            Highlight::Plain => style.colors.text_primary,
        }
    };
    
    // We'll create a simple line of colored spans.
    // For simplicity, just show the first few spans.
    let mut spans_iter = spans.iter().filter(|s| s.start < PREVIEW_LEN).peekable();
    let mut position = 0;
    let mut text_spans: Vec<Span<'a>> = Vec::new();
    
    while let Some(span) = spans_iter.next() {
        // Add plain text before this span
        if span.start > position {
            text_spans.push(Span::new(&preview_source[position..span.start]));
        }
        // Add highlighted span
        let end = span.end.min(PREVIEW_LEN);
        if end > span.start {
            let colored_text = &preview_source[span.start..end];
            text_spans.push(
                Span::new(colored_text)
                    .color(highlight_to_color(span.highlight))
            );
        }
        position = end;
        if position >= PREVIEW_LEN {
            break;
        }
    }
    // Add remaining plain text
    if position < preview_source.len() {
        text_spans.push(Span::new(&preview_source[position..]));
    }
    
    // Build the text widget
    let mut final_text = text::Text::new("");
    for sp in text_spans {
        final_text = final_text.push(sp);
    }
    
    container(
        column![
            text("Syntax Preview").size(10).color(style.colors.text_muted),
            text(final_text).size(12).font(iced::Font::MONOSPACE),
        ]
        .spacing(4)
    )
    .into()
}
