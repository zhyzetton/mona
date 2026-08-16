use crate::app::{AppState, Message, Page};

use iced::widget::button::Status;
use iced::widget::{button, text};
use iced::{Alignment, Length, font};

fn match_page_title(page: &Page) -> &'static str {
    match page {
        Page::Home => "主页",
        Page::Library => "资源库",
        Page::Settings => "设置",
    }
}

pub fn view(page: Page, app_state: &AppState) -> iced::widget::Button<'_, Message> {
    button(
        text(match_page_title(&page))
            .font(font::Font {
                weight: font::Weight::Bold,
                ..Default::default()
            })
            .height(Length::Fill)
            .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .height(40)
    .style(move |_theme, status| {
        if app_state.current_page == page {
            return button::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb8(
                    252, 237, 231,
                ))),
                text_color: iced::Color::from_rgb8(168, 57, 0),
                border: iced::Border {
                    radius: 6.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            };
        }
        match status {
            Status::Active => button::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb8(
                    251, 251, 251,
                ))),
                text_color: iced::Color::from_rgb8(94, 94, 99),
                ..Default::default()
            },
            _ => button::Style::default(),
        }
    })
}
