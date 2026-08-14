use crate::app::{AppState, Message};
use crate::components::card;
use iced::widget::grid;
use iced::{
    Length,
    widget::{column, container, text},
};

pub fn view(state: &AppState) -> iced::Element<'_, Message> {
    let files_card = state
        .files
        .iter()
        .map(|f| card::view(&f.name, &f.path.to_str().unwrap()))
        .collect::<Vec<_>>();
    container(column![text!("资源库"), grid(files_card).spacing(16)])
        .padding(8)
        .width(Length::Fill)
        .into()
}
