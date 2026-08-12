use iced::{Length, Padding};
use iced::widget::{container, button, column};
use crate::app::{AppState, Message, Page};
use crate::components::menu;

pub fn view(app_state: &AppState) -> iced::widget::Container<'_, Message> {
    container(column![
        menu::view(Page::Home, app_state).width(Length::Fill).on_press(Message::NavigateTo(Page::Home)),
        menu::view(Page::Library, app_state).width(Length::Fill).on_press(Message::NavigateTo(Page::Library)),
        menu::view(Page::Settings, app_state).width(Length::Fill).on_press(Message::NavigateTo(Page::Settings)),
    ])
}