use iced::{
    widget::{container, text_editor},
    Element, Length, Font, Color,
};
use std::ops::Range;
use iced_core::text::highlighter::{Highlighter, Format};

use crate::app::Message;
use crate::settings::editor::EditorTypographySettings;

/// Highlighter that uses the per‑line cache built from `syntax‑core`.
struct SyntaxHighlighter {
    line_cache: Vec<Vec<(Range<usize>, Color)>>,
    current_line: usize,
}

impl iced_core::text::highlighter::Highlighter for SyntaxHighlighter {
    type Settings = Vec<Vec<(Range<usize>, Color)>>;
    type Highlight = Color;
    type Iterator<'a> = std::vec::IntoIter<(Range<usize>, Color)>;

    fn new(settings: &Self::Settings) -> Self {
        Self {
            line_cache: settings.clone(),
            current_line: 0,
        }
    }

    fn update(&mut self, settings: &Self::Settings) {
        self.line_cache = settings.clone();
    }

    fn change_line(&mut self, line: usize) {
        self.current_line = line;
    }

    fn current_line(&self) -> usize {
        self.current_line
    }

    fn highlight_line(&mut self, line: &str) -> Self::Iterator<'_> {
        let line_index = self.current_line;
        let mut ranges = Vec::new();
        if let Some(line_highlights) = self.line_cache.get(line_index) {
            for (range, color) in line_highlights {
                // Ensure range is within line bytes length.
                let end = range.end.min(line.len());
                let start = range.start.min(end);
                if start < end {
                    ranges.push((start..end, *color));
                }
            }
        }
        // The iterator must be sorted by position ascending.
        ranges.sort_by_key(|(range, _)| range.start);
        ranges.into_iter()
    }
}

// Custom style sheet for text editor
struct EditorStyleSheet {
    background_color: Color,
}

impl iced::widget::text_editor::StyleSheet for EditorStyleSheet {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> iced::widget::text_editor::Appearance {
        iced::widget::text_editor::Appearance {
            background: self.background_color.into(),
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 0.0.into(),
            },
        }
    }

    fn hovered(&self, _style: &Self::Style) -> iced::widget::text_editor::Appearance {
        iced::widget::text_editor::Appearance {
            background: self.background_color.into(),
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 0.0.into(),
            },
        }
    }

    fn focused(&self, _style: &Self::Style) -> iced::widget::text_editor::Appearance {
        iced::widget::text_editor::Appearance {
            background: self.background_color.into(),
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 0.0.into(),
            },
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgb(0.5, 0.5, 0.5)
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        Color::WHITE
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgb(0.7, 0.7, 0.7)
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgba(0.5, 0.5, 1.0, 0.3)
    }

    fn disabled(&self, _style: &Self::Style) -> iced::widget::text_editor::Appearance {
        iced::widget::text_editor::Appearance {
            background: Color::from_rgb(0.3, 0.3, 0.3).into(),
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 0.0.into(),
            },
        }
    }
}

pub fn editor<'a>(
    text_editor_content: &'a iced::widget::text_editor::Content,
    typography: &EditorTypographySettings,
    background_color: Color,
    line_cache: Option<Vec<Vec<(Range<usize>, Color)>>>,
) -> Element<'a, Message> {
    // Create font based on selected font family
    // Note: Iced's font support is limited, so we use the first font in the fallback stack
    let font_family = typography.font_family.to_family_string();
    let font = Font::with_name(font_family);
    
    // Create a custom style sheet
    let style_sheet = EditorStyleSheet {
        background_color,
    };
    let custom_style: iced::theme::TextEditor = 
        iced::theme::TextEditor::Custom(Box::new(style_sheet) as Box<dyn iced::widget::text_editor::StyleSheet<Style = iced::Theme>>);
    
    let mut editor = text_editor::TextEditor::new(text_editor_content)
        .on_action(Message::EditorContentChanged)
        .font(font)
        .style(custom_style);

    // Apply syntax highlighting if a line cache is provided
    if let Some(cache) = line_cache {
        let highlighter = SyntaxHighlighter::new(&cache);
        editor = editor.highlight(highlighter, |_settings: &Vec<Vec<(Range<usize>, Color)>>, color| {
            Format {
                color: Some(*color),
                font: None,
            }
        });
    }
    
    // The text editor widget has built-in scrolling capabilities
    // It handles both vertical and horizontal scrolling automatically
    // We don't need to wrap it in additional scrollable containers
    // Ensure the editor is properly clipped to prevent overflow
    container(editor)
        .padding(0) // No padding
        .width(Length::Fill)
        .height(Length::Fill)
        .clip(true) // Ensure content doesn't overflow
        .style(iced::theme::Container::Custom(Box::new(move |_theme: &iced::Theme| {
            container::Appearance {
                background: Some(background_color.into()),
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
