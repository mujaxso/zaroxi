use iced::{
    widget::{scrollable, text_input, column, container, row, text},
    Element, Length, Font,
};

use crate::app::Message;

pub fn editor<'a>(editor_content: &'a str) -> Element<'a, Message> {
    // Count lines to show line numbers
    let line_count = editor_content.lines().count().max(1);
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
    
    let editor_input = text_input("", editor_content)
        .on_input(Message::EditorContentChanged)
        .padding([0, 16])
        .width(Length::Fill)
        .font(Font::MONOSPACE)
        .size(14);
    
    row![
        container(line_numbers_column)
            .style(iced::theme::Container::Box)
            .height(Length::Fill),
        scrollable(
            editor_input
        )
        .height(Length::Fill)
    ]
    .height(Length::Fill)
    .into()
}
