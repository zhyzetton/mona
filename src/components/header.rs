use crate::app::{AppState, Message};
use iced::widget::{button, container, row, space, text, text_input};
use iced::{Alignment, Background, Color, Length};

pub fn view(app_state: &AppState) -> iced::Element<'_, Message> {
    container(
        row![
            text("Header"),
            space().width(Length::Fill),
            row![
                text_input("输入关键词", app_state.search_query.as_str())
                    .on_input(Message::SearchChanged)
                    .width(150),
                button(text(&app_state.search_query))
            ]
            .spacing(8)
        ]
        .height(Length::Fill)
        .padding(8)
        .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .height(60)
    .style(|_theme| container::Style {
        background: Some(Background::Color(Color::from_rgb(0.3, 0.3, 0.3))),
        ..Default::default()
    })
    .into()
}
