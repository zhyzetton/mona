use crate::app::{AppState, Message};
use iced::widget::{container, row, space, text, text_input};
use iced::{Alignment, Length};

pub fn view(app_state: &AppState) -> iced::Element<'_, Message> {
    container(
        row![
            text_input("输入关键词", app_state.search_query.as_str())
                .on_input(Message::SearchChanged)
                .width(250)
                .padding(8)
                .style(|_theme, status| text_input::Style {
                    background: iced::Background::Color(iced::Color::from_rgb8(240, 240, 240)),
                    border: iced::Border {
                        radius: 8.0.into(),
                        ..Default::default()
                    },
                    icon: iced::Color::from_rgb8(120, 120, 120),
                    placeholder: iced::Color::from_rgb8(150, 150, 150),
                    value: iced::Color::from_rgb8(50, 50, 50),
                    selection: iced::Color::from_rgb8(200, 200, 200)
                }),
            space().width(Length::Fill),
            text("个人信息")
        ]
        .height(Length::Fill)
        .padding(8)
        .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .height(60)
    .into()
}
