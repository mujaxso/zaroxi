use iced::{
    widget::{container, text_editor},
    Element, Length, Font, Color,
};

use crate::app::Message;
use crate::settings::editor::EditorTypographySettings;

pub fn editor<'a>(
    text_editor_content: &'a iced::widget::text_editor::Content,
    typography: &EditorTypographySettings,
) -> Element<'a, Message> {
    // Create font based on selected font family
    // Note: Iced's font support is limited, so we use the first font in the fallback stack
    let font_family = typography.font_family.to_family_string();
    let font = Font::with_name(font_family);
    
    // Create a text editor with its own built-in scrolling
    // The text_editor widget handles scrolling internally, so we should NOT wrap it
    // in an outer scrollable container to avoid conflicts that cause crashes
    let editor = text_editor::TextEditor::new(text_editor_content)
        .on_action(Message::EditorContentChanged)
        .font(font)
        .height(Length::Fill);
    
    // Place the editor in a container with NO padding
    // The text_editor widget will handle its own scrolling
    // Ensure the editor is properly constrained and clipped
    container(editor)
        .padding(0) // No padding
        .width(Length::Fill)
        .height(Length::Fill)
        .clip(true) // Clip text to prevent overflow
        .style(iced::theme::Container::Custom(Box::new(|_theme: &iced::Theme| {
            container::Appearance {
                background: None, // Let parent handle background
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
