use crate::app::Message;
use iced::widget::image::Handle;
use iced::widget::{container, image, text};
use iced::{Color, ContentFit};
use std::path::Path;

pub fn poster_widget(poster_path: Option<&str>) -> iced::Element<'_, Message> {
    match poster_path {
        Some(p) if Path::new(p).exists() => image(Handle::from_path(p))
            .width(150)
            .height(200)
            .content_fit(ContentFit::Cover)
            .into(),
        _ => container(text!("无封面"))
            .center_x(150)
            .center_y(200)
            .style(|_| container::Style {
                background: Some(iced::Background::Color(Color::from_rgb8(60, 60, 60))),
                ..Default::default()
            })
            .into(),
    }
}
