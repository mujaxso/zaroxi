use iced::{
    widget::{scrollable, column, container, row, text, text_editor},
    Element, Length, Font,
};

use crate::app::Message;

pub fn editor<'a>(text_editor: &'a iced::widget::text_editor::Content) -> Element<'a, Message> {
    // Count lines to show line numbers
    let text = text_editor.text();
    let line_count = text.lines().count().max(1);
    
    let line_numbers: Vec<Element<_>> = (1..=line_count)
        .map(|i| {
            container(
                text(format!("{:>4}", i))
                    .size(14)
                    .font(Font::MONOSPACE)
                    .style(iced::theme::Text::Color(iced::Color::from_rgb8(100, 100, 100)))
            )
            .padding([0, 8, 0, 0])
            .width(Length::Fill)
            .align_x(iced::alignment::Horizontal::Right)
            .into()
        })
        .collect();
    
    let line_numbers_column = column(line_numbers)
        .spacing(0)
        .width(Length::Fixed(60.0));
    
    // Create a text editor
    let editor = text_editor::Editor::new(text_editor)
        .on_action(Message::EditorContentChanged)
        .font(Font::MONOSPACE)
        .size(14)
        .height(Length::Fill);
    
    // Wrap in a scrollable
    let scrollable_editor = scrollable(
        container(editor)
            .padding(16)
            .width(Length::Fill)
            .height(Length::Fill)
    )
    .height(Length::Fill);
    
    row![
        container(line_numbers_column)
            .style(iced::theme::Container::Box)
            .height(Length::Fill),
        scrollable_editor,
    ]
    .height(Length::Fill)
    .into()
}
