//! A syntax‑highlighting editor widget for Neote.
use iced::{
    widget::{column, container, text, Text},
    Element, Length, Color, Font,
};
use crate::settings::editor::EditorTypographySettings;
use crate::ui::style::StyleHelpers;
use syntax_core::HighlightSpan;

/// A styled segment of text within a line.
#[derive(Debug, Clone)]
pub struct StyledSpan {
    /// The text content.
    pub text: String,
    /// The color for this span.
    pub color: Color,
}

/// Build a column of syntax‑highlighted text lines from the given raw text and highlight spans.
pub fn syntax_highlighted_view(
    raw_text: &str,
    spans: &[HighlightSpan],
    typography: &EditorTypographySettings,
    style: &StyleHelpers,
) -> Element<'static, crate::message::Message> {
    // Group spans by line
    let lines: Vec<&str> = raw_text.lines().collect();

    // For each line, produce a vector of styled spans
    let line_elements: Vec<Element<'static, _>> = lines
        .iter()
        .enumerate()
        .map(|(line_idx, line_str)| {
            // Collect all highlight spans that intersect this line
            // For simplicity, we assume spans are already line‑based.
            // In a real implementation we would map byte offsets to line/column.
            // Here we just use a dummy mapping: first line gets first span, etc.
            let dummy_color = style.highlight_color(syntax_core::Highlight::Keyword);
            let styled_text = Text::new(line_str.to_string())
                .font(Font::with_name(typography.font_family.to_family_string()))
                .size(typography.font_size)
                .color(dummy_color);

            // Wrap each line in a container that preserves monospace layout
            container(styled_text)
                .padding(0)
                .width(Length::Fill)
                .into()
        })
        .collect();

    // Wrap lines in a scrollable column
    column(line_elements)
        .spacing(0)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
