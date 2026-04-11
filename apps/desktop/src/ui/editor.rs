use iced::{
    widget::{container, text_editor},
    Element, Length, Font, Color,
};

use crate::app::Message;
use crate::settings::editor::EditorTypographySettings;

// Custom style sheet for text editor
struct EditorStyleSheet {
    background_color: Color,
}

impl iced::widget::text_editor::StyleSheet for EditorStyleSheet {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::text_editor::Appearance {
        iced::widget::text_editor::Appearance {
            background: self.background_color.into(),
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
) -> Element<'a, Message> {
    // Create font based on selected font family
    // Note: Iced's font support is limited, so we use the first font in the fallback stack
    let font_family = typography.font_family.to_family_string();
    let font = Font::with_name(font_family);
    
    // Create a custom style sheet
    let style_sheet = EditorStyleSheet {
        background_color,
    };
    
    let editor = text_editor::TextEditor::new(text_editor_content)
        .on_action(Message::EditorContentChanged)
        .font(font)
        .height(Length::Fill)
        .style(iced::theme::TextEditor::Custom(Box::new(style_sheet)));
    
    // Place the editor in a container with NO padding
    // The text_editor widget will handle its own scrolling
    // Ensure the editor is properly constrained and clipped
    // The container sets the width to Fill, which constrains the text editor
    container(editor)
        .padding(0) // No padding
        .width(Length::Fill)
        .height(Length::Fill)
        .clip(true) // Clip text to prevent overflow
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
