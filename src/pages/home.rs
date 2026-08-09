//! 发现主页:竖向分节 + 横向海报行。

use iced::widget::{
    column, container, row, scrollable, text, Scrollable,
};
use iced::{Element, Length};

use crate::app::Message;
use crate::pages::placeholder_card;
use crate::theme;

/// 主页。
pub fn view() -> Element<'static, Message> {
    column![
        section("最近观看", &["占位影片 1", "占位影片 2", "占位影片 3"]),
        section(
            "最近添加",
            &["占位影片 4", "占位影片 5", "占位影片 6", "占位影片 7"]
        ),
        section("我的收藏", &["占位影片 8", "占位影片 9"]),
    ]
    .spacing(28)
    .into()
}

/// 小节:标题 + 横向滚动海报行。
fn section<'a>(title: &'a str, titles: &[&'static str]) -> Element<'a, Message> {
    let mut cards = row![].spacing(12);
    for t in titles {
        cards = cards.push(
            container(placeholder_card(t, "影视"))
                .width(Length::Fixed(140.0)),
        );
    }

    column![
        text(title).size(18).color(theme::TEXT),
        Scrollable::new(cards)
            .direction(scrollable::Direction::Horizontal(
                scrollable::Scrollbar::default()
            )),
    ]
    .spacing(10)
    .into()
}
