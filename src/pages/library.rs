use crate::app::{AppState, Message};
use iced::widget::container;

pub fn view(state: &AppState) -> iced::Element<'_, Message> {
    container("Library").into()
}
