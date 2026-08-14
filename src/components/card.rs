use crate::app::Message;
use iced::{
    Length,
    widget::{button, column, container, text},
};

pub fn view<'a>(title: &'a str, year: &'a str) -> iced::Element<'a, Message> {
    button(
        container(column![
            text(title)
                .width(Length::Fill)
                .wrapping(text::Wrapping::WordOrGlyph),
            text(year)
                .width(Length::Fill)
                .wrapping(text::Wrapping::WordOrGlyph)
        ])
        .padding(12)
        .width(200)
        .height(250),
    )
    .width(200)
    .into()
}
