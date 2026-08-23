use crate::app::{AppState, Message};
use crate::components::card;
use iced::Length;
use iced::widget::{column, container, grid, scrollable, text};
use crate::pages::video_detail;

pub fn view(state: &AppState) -> iced::Element<'_, Message> {

    let cards = state
        .medias
        .iter()
        .map(|m| {
            let year = m.year.as_deref().unwrap_or("");
            card::view(m.id.unwrap(), &m.title, &year, m.poster_path.as_deref())
        })
        .collect::<Vec<_>>();
    let content = if state.selected_id.is_some() {
        video_detail::view(state)
    } else {
        grid(cards).spacing(16).into()
    };

    container(scrollable(column![content]).height(Length::Fill).width(Length::Fill))
        .padding(8)
        .into()
}
