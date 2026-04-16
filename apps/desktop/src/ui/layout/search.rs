use iced::{
    widget::{button, column, container, horizontal_space, row, text, text_input},
    Alignment, Element, Length,
};
use crate::message::Message;

pub fn search_panel<'a>() -> Element<'a, Message> {
    column![
        row![
            text("SEARCH").size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150))),
            horizontal_space(),
            button("⋯")
                .on_press(Message::PromptInputChanged("Search options".to_string()))
                .style(iced::theme::Button::Secondary),
        ]
        .padding([12, 16])
        .align_items(Alignment::Center),
        iced::widget::horizontal_rule(1),
        container(
            column![
                {
                    // Create text input with explicit type annotation
                    let mut input: iced::widget::TextInput<'_, Message, iced::Theme, iced::Renderer> = 
                        iced::widget::text_input("Search in workspace...", "");
                    input = input.on_input(|query| Message::PromptInputChanged(format!("search: {}", query)));
                    input = input.padding(12);
                    input = input.width(Length::Fill);
                    input
                },
                button("Find All")
                    .on_press(Message::PromptInputChanged("Find all in workspace".to_string()))
                    .style(iced::theme::Button::Primary)
                    .width(Length::Fill)
                    .padding(8),
                container(
                    text("No search results")
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150)))
                )
                .center_y()
                .center_x()
                .height(Length::Fill)
            ]
            .spacing(16)
            .padding(16)
        )
        .height(Length::Fill)
    ]
    .width(Length::Fixed(250.0))
    .height(Length::Fill)
    .into()
}
