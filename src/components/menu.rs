use crate::app::{AppState, Message, Page};
use iced::widget::button;
use iced::widget::button::Status;

fn match_page_title(page: &Page) -> &'static str {
    match page {
        Page::Home => "主页",
        Page::Library => "资源库",
        Page::Settings => "设置",
    }
}

pub fn view(page: Page, app_state: &AppState) -> iced::widget::Button<'_, Message> {
    button(match_page_title(&page)).style(move |_theme, status| {
        if app_state.current_page == page {
            return button::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb(
                    0.1, 0.1, 0.1,
                ))),
                text_color: iced::Color::WHITE,
                ..Default::default()
            };
        }
        match status {
            Status::Active => button::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb(
                    0.2, 0.2, 0.2,
                ))),
                text_color: iced::Color::WHITE,
                ..Default::default()
            },
            Status::Hovered => button::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb(
                    0.3, 0.3, 0.3,
                ))),
                text_color: iced::Color::WHITE,
                ..Default::default()
            },
            _ => button::Style::default(),
        }
    })
}
