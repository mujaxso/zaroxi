use iced::{
    widget::{button, column, container, horizontal_space, row, text, text_input},
    Alignment, Element, Length,
};

use crate::message::Message;

pub fn top_bar<'a>(workspace_path: &'a str, is_dirty: bool) -> Element<'a, Message> {
    let status_indicator: Element<_> = if is_dirty {
        row![
            text("●").size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(255, 180, 0))),
            text("Unsaved").size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(220, 220, 220)))
        ]
        .spacing(4)
        .align_items(Alignment::Center)
        .into()
    } else {
        row![
            text("✓").size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(0, 200, 100))),
            text("Saved").size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(180, 180, 180)))
        ]
        .spacing(4)
        .align_items(Alignment::Center)
        .into()
    };

    container(
        row![
            // Logo/brand - refined
            container(
                row![
                    text("Q").size(18).style(iced::theme::Text::Color(iced::Color::from_rgb8(100, 160, 255))),
                    text("yzer Studio").size(14).style(iced::theme::Text::Color(iced::Color::from_rgb8(200, 210, 230))),
                ]
                .spacing(2)
                .align_items(Alignment::Center)
            )
            .padding([0, 16]),
            
            // Divider
            container(iced::widget::Space::with_width(1.0))
                .style(iced::theme::Container::Custom(Box::new(|_theme: &iced::Theme| {
                    container::Appearance {
                        background: Some(iced::Color::from_rgb8(60, 65, 85).into()),
                        ..Default::default()
                    }
                })))
                .height(Length::Fixed(20.0))
                .width(Length::Fixed(1.0)),
            
            // Workspace path area
            if workspace_path.is_empty() {
                container(
                    row![
                        {
                            let mut input = iced::widget::text_input("Enter workspace path...", workspace_path);
                            input = input.on_input(Message::WorkspacePathChanged);
                            input = input.on_submit(Message::SubmitManualWorkspacePath(workspace_path.to_string()));
                            input = input.padding([8, 12]);
                            input = input.width(Length::Fixed(300.0));
                            input = input.style(iced::theme::TextInput::Default);
                            input
                        },
                        button("Open")
                            .on_press(Message::SubmitManualWorkspacePath(workspace_path.to_string()))
                            .padding([8, 12])
                            .style(iced::theme::Button::Secondary),
                    ]
                    .spacing(8)
                    .align_items(Alignment::Center)
                )
                .into()
            } else {
                container(
                    text(workspace_path)
                        .size(13)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(180, 190, 210)))
                )
                .padding([8, 12])
                .into()
            },
            
            horizontal_space(),
            
            // Action buttons - refined
            row![
                button(
                    container(
                        row![
                            text("📂").size(12),
                            text("Open...").size(13),
                        ]
                        .spacing(6)
                        .align_items(Alignment::Center)
                    )
                    .padding([6, 10])
                )
                .on_press(Message::OpenWorkspace)
                .style(iced::theme::Button::Secondary),
                button(
                    container(
                        text("⟳").size(13)
                    )
                    .padding([6, 8])
                )
                .on_press(Message::RefreshWorkspace)
                .style(iced::theme::Button::Text),
            ]
            .spacing(4),
            
            // Status indicator
            container(status_indicator)
                .padding([6, 12]),
            
            // Save button - refined
            button(
                container(
                    text("Save")
                        .size(13)
                )
                .padding([8, 16])
            )
            .on_press(Message::SaveFile)
            .style(iced::theme::Button::Primary),
        ]
        .align_items(Alignment::Center)
    )
    .padding([8, 16])
    .width(Length::Fill)
    .height(Length::Fixed(48.0))
    .style(iced::theme::Container::Custom(Box::new(|_theme: &iced::Theme| {
        container::Appearance {
            background: Some(iced::Color::from_rgb8(30, 33, 45).into()),
            border: iced::Border {
                color: iced::Color::from_rgb8(50, 55, 70),
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        }
    })))
    .into()
}
