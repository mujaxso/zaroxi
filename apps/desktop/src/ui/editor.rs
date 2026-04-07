use iced::{
    widget::{scrollable, text_input},
    Element, Length, Font,
};

use crate::app::Message;

pub fn editor<'a>(editor_content: &'a str) -> Element<'a, Message> {
    scrollable(
        text_input("", editor_content)
            .on_input(Message::EditorContentChanged)
            .padding(16)
            .width(Length::Fill)
            .font(Font::MONOSPACE)
            .size(14)
            .style(|theme| text_input::Style {
                background: iced::Background::Color(
                    theme.extended_palette().background.base.color
                ),
                border: iced::Border {
                    width: 0.0,
                    ..Default::default()
                },
                ..Default::default()
            })
    )
    .height(Length::Fill)
    .into()
}
