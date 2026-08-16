use iced::widget::{container, text};
use iced::{Alignment, Length, font};

use crate::app::{AppState, Message};

pub fn view(app_state: &AppState) -> iced::Element<'_, Message> {
    container(
        text("Mona")
            .font(font::Font {
                weight: font::Weight::Bold,
                ..Default::default()
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .align_y(Alignment::Center)
            .align_x(Alignment::Center),
    )
    .height(60)
    .into()
}
