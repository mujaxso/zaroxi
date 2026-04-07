use iced::{
    widget::{scrollable, text_input, column, container, row, text},
    Element, Length, Font,
};

use crate::app::Message;

pub fn editor<'a>(editor_content: &'a str) -> Element<'a, Message> {
    // Count lines to show line numbers
    let line_count = editor_content.lines().count().max(1);
    let visible_lines = line_count.max(20); // Show at least 20 lines
    
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
    
    // Create a multi-line text input
    // Use .lines() to specify the number of visible lines
    let editor_input = text_input("", editor_content)
        .on_input(Message::EditorContentChanged)
        .padding(16)
        .width(Length::Fill)
        .font(Font::MONOSPACE)
        .size(14)
        .lines(visible_lines as u16);
    
    // Wrap in a scrollable to handle when content exceeds visible area
    let scrollable_editor = scrollable(
        container(editor_input)
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
