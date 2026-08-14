use crate::app::{AppState, Message};
use iced::widget::{column, container, text};

pub fn view(app_state: &AppState) -> iced::Element<'_, Message> {
    container(column![text!("title"), text!("description"),]).into()
}
