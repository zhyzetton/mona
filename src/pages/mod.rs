//! 内容区各页面。

mod browse;
mod detail;
mod favorites;
mod home;
mod library;
mod search;
mod settings;

use iced::widget::{column, container, text};
use iced::{Element, Fill, Length, Padding};

use crate::app::{App, Message};
use crate::nav::Route;
use crate::theme;

/// 按当前路由渲染内容区。
pub fn view(app: &App) -> Element<'_, Message> {
    match app.route() {
        Route::Home => home::view(),
        Route::Library(kind) => library::view(app, *kind),
        Route::Browse(id) => browse::view(app, *id),
        Route::Detail(id) => detail::view(*id),
        Route::Favorites => favorites::view(),
        Route::Search => search::view(app),
        Route::Settings => settings::view(app),
    }
}

/// 页面小节标题。
pub fn section_header(title: &'static str) -> Element<'static, Message> {
    text(title).size(11).color(theme::TEXT_MUTED).into()
}

/// 占位海报卡:海报区 + 标题栏(数据层接入前的布局替身)。
pub fn placeholder_card(
    title: &'static str,
    kind: &'static str,
) -> Element<'static, Message> {
    column![
        container(text(kind).size(12).color(theme::TEXT_MUTED).center())
            .width(Fill)
            .height(Fill)
            .style(theme::surface),
        container(text(title).size(13).color(theme::TEXT))
            .padding(Padding { top: 6.0, ..Padding::ZERO }),
    ]
    .into()
}

/// 空状态占位(居中的标题 + 提示)。
pub fn empty_state(title: String, hint: &'static str) -> Element<'static, Message> {
    container(
        column![
            text(title).size(20).color(theme::TEXT),
            text(hint).size(13).color(theme::TEXT_MUTED),
        ]
        .spacing(8)
        .align_x(iced::Center)
        .width(Length::Fill),
    )
    .width(Fill)
    .height(Fill)
    .center_y(Length::Fill)
    .into()
}
