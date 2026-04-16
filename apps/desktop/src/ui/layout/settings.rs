use iced::{
    widget::{button, column, container, horizontal_space, row, scrollable, text, text_input},
    Alignment, Element, Length,
};
use crate::message::Message;

pub fn settings_panel<'a>() -> Element<'a, Message> {
    column![
        row![
            text("SETTINGS").size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150))),
            horizontal_space(),
            button("Save")
                .on_press(Message::PromptInputChanged("Settings saved".to_string()))
                .style(iced::theme::Button::Primary),
        ]
        .padding([12, 16])
        .align_items(Alignment::Center),
        iced::widget::horizontal_rule(1),
        scrollable(
            column![
                container(
                    column![
                        text("Editor").size(16),
                        text("Font size:").size(14),
                        {
                            let mut input = iced::widget::text_input("14", "14");
                            input = input.on_input(|size| Message::PromptInputChanged(format!("Font size: {}", size)));
                            input = input.padding(8);
                            input
                        },
                        text("Theme:").size(14),
                        button("Dark")
                            .on_press(Message::PromptInputChanged("Theme set to Dark".to_string()))
                            .style(iced::theme::Button::Secondary),
                        button("Light")
                            .on_press(Message::PromptInputChanged("Theme set to Light".to_string()))
                            .style(iced::theme::Button::Secondary),
                    ]
                    .spacing(8)
                    .padding(16)
                )
                .style(iced::theme::Container::Box),
                container(
                    column![
                        text("AI Settings").size(16),
                        text("Model:").size(14),
                        {
                            let mut input = iced::widget::text_input("gpt-4", "gpt-4");
                            input = input.on_input(|model| Message::PromptInputChanged(format!("AI model: {}", model)));
                            input = input.padding(8);
                            input
                        },
                        text("API Key:").size(14),
                        {
                            let mut input = iced::widget::text_input("••••••••", "");
                            input = input.on_input(|_| Message::PromptInputChanged("API key updated".to_string()));
                            input = input.padding(8);
                            input
                        },
                    ]
                    .spacing(8)
                    .padding(16)
                )
                .style(iced::theme::Container::Box),
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
