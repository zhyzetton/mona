use crate::app::{AppState, Message};
use iced::Length;
use iced::widget::container;

pub fn view(state: &AppState) -> iced::Element<'_, Message> {
    container("Settings").padding(8).width(Length::Fill).into()
}
