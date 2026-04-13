use iced::{
    widget::{container, text_editor},
    Element, Length, Font, Color, Theme,
};
use iced_core::text::highlighter::Format;
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
        eprintln!("DEBUG: SyntaxHighlighter::change_line: {}", line);
        self.current_line = line;
    }

    fn current_line(&self) -> usize {
        self.current_line
    }

    fn highlight_line(&mut self, line: &str) -> Self::Iterator<'_> {
        // Try to find which line this is by matching with the cache
        // This is inefficient but works around Iced not calling change_line properly
        let mut ranges = Vec::new();
        
        // Find the line index by searching through the cache
        // We'll use a simple approach: find the first line where the text could match
        // Since we don't have the full line text, we'll use a heuristic
        let line_index = self.find_line_index(line);
        
        eprintln!("DEBUG: highlight_line called with text length {}, found line index {:?}", line.len(), line_index);
        
        if let Some(line_index) = line_index {
            if let Some(line_highlights) = self.line_cache.get(line_index) {
                eprintln!("DEBUG: line {} has {} highlights", line_index, line_highlights.len());
                // Convert character ranges to byte ranges
                let line_chars: Vec<char> = line.chars().collect();
                for (i, (char_range, color)) in line_highlights.iter().enumerate() {
                    // Convert character positions to byte positions
                    let char_start = char_range.start;
                    let char_end = char_range.end.min(line_chars.len());
                    
                    eprintln!("DEBUG: highlight {}: char_range {:?}, color {:?}", i, char_range, color);
                    
                    if char_start < char_end {
                        // Calculate byte positions
                        let byte_start = line_chars[..char_start].iter().map(|c| c.len_utf8()).sum::<usize>();
                        let byte_end = byte_start + line_chars[char_start..char_end].iter().map(|c| c.len_utf8()).sum::<usize>();
                        
                        eprintln!("DEBUG:   byte_range {}..{}", byte_start, byte_end);
                        
                        if byte_start < byte_end && byte_end <= line.len() {
                            ranges.push((byte_start..byte_end, *color));
                            eprintln!("DEBUG:   added range");
                        } else {
                            eprintln!("DEBUG:   range invalid (byte_end {} > line.len() {})", byte_end, line.len());
                        }
                    } else {
                        eprintln!("DEBUG:   char_start >= char_end");
                    }
                }
            } else {
                eprintln!("DEBUG: line {} has no highlights in cache", line_index);
            }
        } else {
            eprintln!("DEBUG: could not find line index for text");
        }
        
        eprintln!("DEBUG: returning {} ranges", ranges.len());
        // The iterator must be sorted by position ascending.
        ranges.sort_by_key(|(range, _)| range.start);
        ranges.into_iter()
    }
}

impl SyntaxHighlighter {
    // Try to find which line index corresponds to the given text
    // This is a heuristic approach since we don't have the full context
    fn find_line_index(&self, line_text: &str) -> Option<usize> {
        // Simple approach: find the first line in the cache where the highlights
        // could match the text length
        // This assumes that lines with highlights are unique enough
        for (i, highlights) in self.line_cache.iter().enumerate() {
            if !highlights.is_empty() {
                // Check if the last highlight end is within the text length
                if let Some(last_highlight) = highlights.last() {
                    if last_highlight.0.end <= line_text.chars().count() {
                        return Some(i);
                    }
                }
            }
        }
        // If no line with highlights matches, return None
        None
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

// Custom style sheet for text editor (background comes from container)
struct EditorStyle {
    text_color: Color,
}

impl iced::widget::text_editor::StyleSheet for EditorStyle {
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
        self.text_color
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
    text_color: Color,
    line_cache: Option<Vec<Vec<(Range<usize>, Color)>>>,
) -> Element<'a, Message> {
    // Create font based on selected font family
    let font_family = typography.font_family.to_family_string();
    let font = Font::with_name(font_family);
    
    // Use a custom style for the editor with proper text color
    let custom_style = iced::theme::TextEditor::Custom(Box::new(EditorStyle {
        text_color,
    }));
    
    // Check if we should use syntax highlighting
    // We have syntax highlighting if any line in the cache has highlights
    let use_syntax_highlighting = line_cache.as_ref().map_or(false, |cache| {
        cache.iter().any(|line| !line.is_empty())
    });
    eprintln!("DEBUG: use_syntax_highlighting = {}, line_cache.is_some() = {}, line_cache.map_or(0, |c| c.len()) = {}, total_highlights = {}", 
              use_syntax_highlighting, 
              line_cache.is_some(),
              line_cache.as_ref().map_or(0, |c| c.len()),
              line_cache.as_ref().map_or(0, |c| c.iter().map(|line| line.len()).sum::<usize>()));
    
    // Create a base editor with explicit theme type
    fn create_base_editor<'b>(
        content: &'b iced::widget::text_editor::Content,
        font: Font,
        custom_style: iced::theme::TextEditor,
    ) -> iced::widget::TextEditor<'b, iced_core::text::highlighter::PlainText, Message, iced::Theme> {
        text_editor::TextEditor::new(content)
            .on_action(Message::EditorContentChanged)
            .font(font)
            .style(custom_style)
    }
    
    if use_syntax_highlighting {
        let cache = line_cache.unwrap();
        eprintln!("DEBUG: Using syntax highlighting with {} lines of cache", cache.len());
        // Create editor with syntax highlighting
        let base_editor = create_base_editor(text_editor_content, font, custom_style);
        let editor = base_editor.highlight::<SyntaxHighlighter>(
            cache,
            |color: &Color, _theme: &Theme| -> Format<Font> {
                Format {
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
        let editor = create_base_editor(text_editor_content, font, custom_style);
        
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
