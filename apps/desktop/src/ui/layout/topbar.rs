use iced::{
    widget::{container, horizontal_space, row, text},
    Alignment, Element, Length,
};

use crate::message::Message;

pub fn top_bar<'a>(workspace_path: &'a str) -> Element<'a, Message> {
    container(
        row![
            // Branding - minimal and elegant
            container(
                row![
                    text("Z").size(18).style(iced::theme::Text::Color(iced::Color::from_rgb8(100, 160, 255))),
                    text("aroxi Studio").size(14).style(iced::theme::Text::Color(iced::Color::from_rgb8(200, 210, 230))),
                ]
                .spacing(2)
                .align_items(Alignment::Center)
            )
            .padding([0, 16]),
            
            // Subtle divider
            container(iced::widget::Space::with_width(1.0))
                .style(iced::theme::Container::Custom(Box::new(|_theme: &iced::Theme| {
                    container::Appearance {
                        background: Some(iced::Color::from_rgb8(60, 65, 85).into()),
                        ..Default::default()
                    }
                })))
                .height(Length::Fixed(20.0))
                .width(Length::Fixed(1.0)),
            
            // Workspace path display - clean and non-interactive
            if !workspace_path.is_empty() {
                Element::from(container(
                    text(workspace_path)
                        .size(13)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(180, 190, 210)))
                )
                .padding([8, 12]))
            } else {
                Element::from(container(
                    text("No workspace open")
                        .size(13)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(120, 130, 150)))
                )
                .padding([8, 12]))
            },
            
            horizontal_space(),
            
            // Right side reserved for potential window controls or other premium IDE elements
            // Currently kept empty for a clean, professional look
        ]
        .align_items(Alignment::Center)
    )
    .padding([8, 16])
    .width(Length::Fill)
    .height(Length::Fixed(40.0)) // More compact height
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
