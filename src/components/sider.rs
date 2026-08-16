use crate::app::{AppState, Message, Page};
use crate::components::{icon, menu};
use iced::widget::{column, container};
use iced::{Background, Color, Length, Padding};

pub fn view(app_state: &AppState) -> iced::Element<'_, Message> {
    container(column![
        icon::view(app_state),
        menu::view(Page::Home, app_state)
            .width(Length::Fill)
            .on_press(Message::NavigateTo(Page::Home)),
        menu::view(Page::Library, app_state)
            .width(Length::Fill)
            .on_press(Message::NavigateTo(Page::Library)),
        menu::view(Page::Settings, app_state)
            .width(Length::Fill)
            .on_press(Message::NavigateTo(Page::Settings)),
    ])
    .width(200)
    .height(Length::Fill)
    .padding(Padding {
        top: 0.0,
        right: 16.0,
        bottom: 0.0,
        left: 16.0,
    })
    .style(|_theme| container::Style {
        background: Some(Background::Color(Color::from_rgb8(251, 251, 251))),
        ..Default::default()
    })
    .into()
}
