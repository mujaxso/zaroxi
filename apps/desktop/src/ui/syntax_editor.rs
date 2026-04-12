//! A syntax‑highlighting editor widget for Neote.
use iced::{
    widget::{column, container, Row, Text},
    Element, Length, Color, Font, Renderer,
};
use crate::settings::editor::EditorTypographySettings;
use crate::ui::style::StyleHelpers;
use syntax_core::HighlightSpan;

/// Build a column of syntax‑highlighted text lines from the given raw text and highlight spans.
pub fn syntax_highlighted_view(
    raw_text: &str,
    spans: &[HighlightSpan],
    typography: &EditorTypographySettings,
    style: &StyleHelpers,
) -> Element<'static, crate::message::Message> {
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
    column(line_elements)
        .spacing(0)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
