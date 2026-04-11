use iced::{
    widget::{container, text_editor},
    Element, Length, Font, Color,
};

use crate::app::Message;
use crate::settings::editor::EditorTypographySettings;
use crate::ui::style::StyleHelpers;

pub fn editor<'a>(
    text_editor_content: &'a iced::widget::text_editor::Content,
    typography: &EditorTypographySettings,
    background_color: Color,
) -> Element<'a, Message> {
    // Create font based on selected font family
    // Note: Iced's font support is limited, so we use the first font in the fallback stack
    let font_family = typography.font_family.to_family_string();
    let font = Font::with_name(font_family);
    
    // Create a text editor with its own built-in scrolling
    // The text_editor widget handles scrolling internally, so we should NOT wrap it
    // in an outer scrollable container to avoid conflicts that cause crashes
    // We need to create a temporary StyleHelpers to get the text editor style
    // For now, we'll create a custom style that matches the background color
    let editor_style = iced::widget::text_editor::Appearance {
        background: background_color.into(),
        border: iced::Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 0.0.into(),
        },
        cursor: iced::widget::text_editor::Cursor {
            color: Color::WHITE,
        },
        selection: iced::widget::text_editor::Selection {
            background: Color::from_rgba(0.5, 0.5, 1.0, 0.3),
            color: Color::WHITE,
        },
        placeholder: iced::widget::text_editor::Placeholder {
            color: Color::from_rgb(0.5, 0.5, 0.5),
        },
        line_numbers: iced::widget::text_editor::LineNumbers {
            color: Color::from_rgb(0.5, 0.5, 0.5),
            background: background_color.into(),
        },
        scrollbar: iced::widget::text_editor::Scrollbar {
            background: Color::TRANSPARENT,
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 0.0.into(),
            },
            scroller: iced::widget::text_editor::Scroller {
                color: Color::from_rgb(0.3, 0.3, 0.3),
                border: iced::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 0.0.into(),
                },
            },
        },
        handle: iced::widget::text_editor::Handle {
            color: Color::from_rgb(0.3, 0.3, 0.3),
        },
    };
    
    let editor = text_editor::TextEditor::new(text_editor_content)
        .on_action(Message::EditorContentChanged)
        .font(font)
        .height(Length::Fill)
        .style(iced::theme::TextEditor::Custom(Box::new(move || editor_style)));
    
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
