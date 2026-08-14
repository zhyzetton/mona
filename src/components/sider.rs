use crate::app::{AppState, Message, Page};
use crate::components::menu;
use iced::widget::{column, container};
use iced::{Background, Color, Length};

pub fn view(app_state: &AppState) -> iced::Element<'_, Message> {
    container(column![
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
    .style(|_theme| container::Style {
        background: Some(Background::Color(Color::from_rgb(0.2, 0.2, 0.2))),
        ..Default::default()
    })
    .into()
}
