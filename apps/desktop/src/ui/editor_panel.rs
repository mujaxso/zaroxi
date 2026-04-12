use iced::{Element, Length, Color, Font, Renderer, widget::{column, container, row, Row, text, Text, scrollable}};
use syntax_core::{Highlight, HighlightSpan};
use crate::message::Message;
use crate::state::App;
use super::style::StyleHelpers;
use super::editor;
use crate::ui::icons::Icon;

/// Build a column of syntax‑highlighted text lines from the given raw text and highlight spans.
pub fn syntax_highlighted_view(
    raw_text: &str,
    spans: &[syntax_core::HighlightSpan],
    typography: &crate::settings::editor::EditorTypographySettings,
    style: &super::style::StyleHelpers,
) -> Element<'static, Message> {
    // Pre‑compute line start bytes and the full text as bytes.
    let text_bytes = raw_text.as_bytes();
    let mut line_starts = vec![0];
    let mut byte_pos = 0;
    for ch in raw_text.chars() {
        if ch == '\n' {
            byte_pos += ch.len_utf8();
            line_starts.push(byte_pos);
        } else {
            byte_pos += ch.len_utf8();
        }
    }
    // Add an extra sentinel for the line after the last newline.
    line_starts.push(byte_pos); // ensures a line after last char

    // For each line, build a Row of colored Text segments.
    let line_count = line_starts.len() - 1;
    let font = Font::with_name(typography.font_family.to_family_string());
    let default_color = style.colors.text_primary;

    let line_elements: Vec<Element<'static, _>> = (0..line_count)
        .map(|line_idx| {
            let line_start = line_starts[line_idx];
            let line_end = line_starts[line_idx + 1];
            // Clamp line_end to not exceed text length (the sentinel may overshoot).
            let line_end = line_end.min(text_bytes.len());

            let mut segments: Vec<Text<'_, iced::Theme, Renderer>> = Vec::new();
            let mut current_pos = line_start;

            // Collect spans that overlap this line.
            for span in spans.iter() {
                let span_start = span.start;
                let span_end = span.end;

                // Skip spans that end before this line or start after this line.
                if span_end <= line_start || span_start >= line_end {
                    continue;
                }

                // Add plain text before this span.
                if current_pos < span_start {
                    let plain = &raw_text[current_pos..span_start];
                    if !plain.is_empty() {
                        segments.push(Text::new(plain.to_string())
                            .font(font)
                            .size(typography.font_size)
                            .style(iced::theme::Text::Color(default_color)));
                    }
                }

                // Add the highlighted span.
                let seg_start = span_start.max(line_start);
                let seg_end = span_end.min(line_end);
                if seg_start < seg_end {
                    let seg_text = &raw_text[seg_start..seg_end];
                    let color = style.highlight_color(span.highlight);
                    segments.push(Text::new(seg_text.to_string())
                        .font(font)
                        .size(typography.font_size)
                        .style(iced::theme::Text::Color(color)));
                }
                current_pos = seg_end;
            }

            // Add remaining plain text after the last span.
            if current_pos < line_end {
                let plain = &raw_text[current_pos..line_end];
                if !plain.is_empty() {
                    segments.push(Text::new(plain.to_string())
                        .font(font)
                        .size(typography.font_size)
                        .style(iced::theme::Text::Color(default_color)));
                }
            }

            // If the line had no spans at all, we still need to show its content.
            if segments.is_empty() && line_start < line_end {
                let whole_line = &raw_text[line_start..line_end];
                segments.push(Text::new(whole_line.to_string())
                    .font(font)
                    .size(typography.font_size)
                    .style(iced::theme::Text::Color(default_color)));
            }

            // Join all segments into a Row.
            let row = Row::with_children(
                segments.into_iter().map(Element::from)
            )
            .spacing(0);

            container(row)
                .padding(0)
                .width(Length::Fill)
                .into()
        })
        .collect();

    // Wrap lines in a scrollable column.
    let col = column(line_elements)
        .spacing(0)
        .width(Length::Fill)
        .height(Length::Fill);
    scrollable(col)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
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
    
    let editor_content: Element<'_, Message> = if let Some(_) = &app.active_file_path {
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
            // Use syntax‑highlighted view (read‑only for now)
            let raw_text = app.text_editor.text();
            let spans = &app.syntax_highlight_spans;
            let view = syntax_highlighted_view(
                &raw_text,
                spans,
                &app.editor_typography,
                &style,
            );
            container(view)
                .width(Length::Fill)
                .height(Length::Fill)
                .clip(true)
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
            
        // Add a note about editing being disabled for syntax highlighting
        let note = container(
            text("Syntax highlighting enabled — editing temporarily disabled")
                .size(10)
                .style(iced::theme::Text::Color(style.colors.text_muted))
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
        })));
        column_children.push(note.into());
    }
    
    // Add editor content
    column_children.push(
        container(editor_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .clip(true) // Ensure content doesn't overflow
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
                color: style.colors.border,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        }
    })))
    .into()
}

