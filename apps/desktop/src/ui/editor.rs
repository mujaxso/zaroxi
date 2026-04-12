use iced::{
    widget::{container, text_editor},
    Element, Length, Font, Color,
};
use std::ops::Range;

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
            // Convert character ranges to byte ranges
            let line_chars: Vec<char> = line.chars().collect();
            for (char_range, color) in line_highlights {
                // Convert character positions to byte positions
                let char_start = char_range.start;
                let char_end = char_range.end.min(line_chars.len());
                
                if char_start < char_end {
                    // Calculate byte positions
                    let byte_start = line_chars[..char_start].iter().map(|c| c.len_utf8()).sum::<usize>();
                    let byte_end = byte_start + line_chars[char_start..char_end].iter().map(|c| c.len_utf8()).sum::<usize>();
                    
                    if byte_start < byte_end && byte_end <= line.len() {
                        ranges.push((byte_start..byte_end, *color));
                    }
                }
            }
        }
        // The iterator must be sorted by position ascending.
        ranges.sort_by_key(|(range, _)| range.start);
        ranges.into_iter()
    }
}

impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self {
            line_cache: Vec::new(),
            current_line: 0,
        }
    }
}

// Transparent style sheet for text editor (background comes from container)
struct TransparentStyle;

impl iced::widget::text_editor::StyleSheet for TransparentStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> iced::widget::text_editor::Appearance {
        iced::widget::text_editor::Appearance {
            background: Color::TRANSPARENT.into(),
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 0.0.into(),
            },
        }
    }

    fn hovered(&self, _style: &Self::Style) -> iced::widget::text_editor::Appearance {
        iced::widget::text_editor::Appearance {
            background: Color::TRANSPARENT.into(),
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 0.0.into(),
            },
        }
    }

    fn focused(&self, _style: &Self::Style) -> iced::widget::text_editor::Appearance {
        iced::widget::text_editor::Appearance {
            background: Color::TRANSPARENT.into(),
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
    let font_family = typography.font_family.to_family_string();
    let font = Font::with_name(font_family);
    
    // Use a transparent style for the editor; background is provided by the container
    let custom_style = iced::theme::TextEditor::Custom(Box::new(TransparentStyle));
    
    // Check if we should use syntax highlighting
    let use_syntax_highlighting = line_cache.as_ref().map_or(false, |cache| !cache.is_empty());
    
    if use_syntax_highlighting {
        let cache = line_cache.unwrap();
        eprintln!("DEBUG: Using syntax highlighting with {} lines of cache", cache.len());
        // Create editor with syntax highlighting
        let editor = text_editor::TextEditor::new(text_editor_content)
            .on_action(Message::EditorContentChanged)
            .font(font)
            .style(custom_style)
            .highlight::<SyntaxHighlighter>(
                cache,
                |color: &Color, _font: &Font| {
                    iced_core::text::highlighter::Format {
                        color: Some(*color),
                        font: None,
                    }
                },
            );
        
        container(editor)
            .padding(0)
            .width(Length::Fill)
            .height(Length::Fill)
            .clip(true)
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
    } else {
        eprintln!("DEBUG: No syntax highlighting");
        // Create editor without syntax highlighting
        let editor = text_editor::TextEditor::new(text_editor_content)
            .on_action(Message::EditorContentChanged)
            .font(font)
            .style(custom_style);
        
        container(editor)
            .padding(0)
            .width(Length::Fill)
            .height(Length::Fill)
            .clip(true)
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
}
