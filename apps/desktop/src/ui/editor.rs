use iced::{
    widget::{container, text_editor},
    Element, Length, Font,
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
    
    // Place the editor in a container with padding
    // The text_editor widget will handle its own scrolling
    container(editor)
        .padding([12, 20, 20, 20]) // Comfortable padding for coding
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
