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
    // Track if we've started highlighting a new document
    is_new_document: bool,
}

impl iced_core::text::highlighter::Highlighter for SyntaxHighlighter {
    type Settings = Vec<Vec<(Range<usize>, Color)>>;
    type Highlight = Color;
    type Iterator<'a> = std::vec::IntoIter<(Range<usize>, Color)>;

    fn new(settings: &Self::Settings) -> Self {
        Self {
            line_cache: settings.clone(),
            current_line: 0,
            is_new_document: true,
        }
    }

    fn update(&mut self, settings: &Self::Settings) {
        self.line_cache = settings.clone();
        // Reset when we get new settings (new document)
        self.current_line = 0;
        self.is_new_document = true;
        eprintln!("DEBUG: SyntaxHighlighter::update called with {} lines", self.line_cache.len());
    }

    fn change_line(&mut self, line: usize) {
        eprintln!("DEBUG: SyntaxHighlighter::change_line: {}", line);
        // Update current_line to the given line, but don't set is_new_document = false
        // because we want to continue using sequential line numbers
        self.current_line = line;
        // Note: We're not setting is_new_document = false to avoid breaking sequential processing
    }

    fn current_line(&self) -> usize {
        self.current_line
    }

    fn highlight_line(&mut self, line: &str) -> Self::Iterator<'_> {
        let mut ranges = Vec::new();
        
        // If the cache is empty, return empty ranges immediately
        if self.line_cache.is_empty() {
            eprintln!("DEBUG: highlight_line called with empty cache, returning 0 ranges");
            return ranges.into_iter();
        }
        
        // Use current_line as the line index, assuming lines are processed in order
        let line_index = self.current_line;
        
        // Increment current_line for the next call
        // This ensures sequential processing even if change_line is not called for each line
        if line_index < self.line_cache.len() {
            self.current_line += 1;
        }
        
        eprintln!("DEBUG: highlight_line called with text length {}, line_index = {}, current_line = {}, is_new_document = {}, cache_size = {}", 
                 line.len(), line_index, self.current_line, self.is_new_document, self.line_cache.len());
        
        // Check bounds
        if line_index < self.line_cache.len() {
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
            eprintln!("DEBUG: line_index {} out of bounds (cache size: {})", line_index, self.line_cache.len());
        }
        
        eprintln!("DEBUG: returning {} ranges", ranges.len());
        // The iterator must be sorted by position ascending.
        ranges.sort_by_key(|(range, _)| range.start);
        ranges.into_iter()
    }
}

impl SyntaxHighlighter {
    // Try to find the best line match for the given text
    fn find_best_line_match(&self, line_text: &str) -> Option<usize> {
        let line_text_len = line_text.chars().count();
        
        // First, try to find a line where all highlights fit within the text
        for (i, highlights) in self.line_cache.iter().enumerate() {
            if !highlights.is_empty() {
                let all_fit = highlights.iter().all(|(range, _)| range.end <= line_text_len);
                if all_fit {
                    // Check that at least one highlight is not empty
                    let has_valid = highlights.iter().any(|(range, _)| range.start < range.end);
                    if has_valid {
                        return Some(i);
                    }
                }
            }
        }
        
        // If no perfect match, try to find a line where at least some highlights fit
        for (i, highlights) in self.line_cache.iter().enumerate() {
            if !highlights.is_empty() {
                let some_fit = highlights.iter().any(|(range, _)| range.end <= line_text_len);
                if some_fit {
                    return Some(i);
                }
            }
        }
        
        None
    }
}

impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self {
            line_cache: Vec::new(),
            current_line: 0,
            is_new_document: true,
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
    
    // Check if we have a valid cache
    let cache = line_cache.unwrap_or_else(|| Vec::new());
    let has_cache = !cache.is_empty();
    
    eprintln!("DEBUG: Using editor with cache of {} lines, total highlights: {}, has_cache={}", 
              cache.len(),
              cache.iter().map(|line| line.len()).sum::<usize>(),
              has_cache);
    
    // Create editor with syntax highlighting only if we have a cache
    let base_editor = text_editor::TextEditor::new(text_editor_content)
        .on_action(Message::EditorContentChanged)
        .font(font)
        .style(custom_style);
    
    let editor: Element<'_, Message> = if has_cache {
        base_editor.highlight::<SyntaxHighlighter>(
            cache,
            |color: &Color, _theme: &Theme| -> Format<Font> {
                Format {
                    color: Some(*color),
                    font: None,
                }
            },
        )
        .into()
    } else {
        // Use plain text editor without highlighting when cache is empty
        base_editor.into()
    };
    
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
