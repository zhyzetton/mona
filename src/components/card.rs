use crate::app::Message;
use iced::{
    Alignment, Length,
    widget::{button, column, container, text},
};

use crate::components::poster::poster_widget;

pub fn view<'a>(
    id: i64,
    title: &'a str,
    year: &'a str,
    poster: Option<&'a str>,
) -> iced::Element<'a, Message> {
    let poster_widget: iced::Element<'a, Message> = poster_widget(poster);

    button(
        container(column![
            container(poster_widget)
                .width(Length::Fill)
                .align_x(Alignment::Center),
            text(title)
                .width(Length::Fill)
                .wrapping(text::Wrapping::WordOrGlyph),
            text(year)
                .width(Length::Fill)
                .wrapping(text::Wrapping::WordOrGlyph)
        ])
        .padding(12)
        .width(200)
        .height(300),
    )
    .on_press(Message::OpenDetail(id))
    .width(200)
    .into()
}
