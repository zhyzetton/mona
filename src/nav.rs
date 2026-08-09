//! 导航模型:路由定义 + 侧边栏。

use iced::widget::{button, column, container, text};
use iced::{Element, Fill, Length, Padding};
use crate::app::{App, Message};
use crate::theme;

/// 媒体源 id(后续由数据库分配)。
pub type SourceId = u32;
/// 媒体条目 id(后续由数据库分配)。
pub type MediaId = u32;

/// 应用路由。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    /// 发现主页。
    Home,
    /// 媒体库分类页。
    Library(LibraryKind),
    /// 媒体源内浏览。
    Browse(SourceId),
    /// 媒体详情。
    // TODO(data-layer): 媒体卡点击后由卡片组件导航到此。
    #[allow(dead_code)]
    Detail(MediaId),
    /// 收藏。
    Favorites,
    /// 搜索。
    Search,
    /// 设置。
    Settings,
}

/// 媒体库分类。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LibraryKind {
    Movies,
    TvShows,
}

impl LibraryKind {
    pub fn title(self) -> &'static str {
        match self {
            Self::Movies => "电影",
            Self::TvShows => "剧集",
        }
    }
}

/// 侧边栏条目(用于高亮当前路由)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Entry {
    Home,
    Movies,
    TvShows,
    Source(SourceId),
    Favorites,
    Settings,
}

/// 当前路由对应的侧边栏高亮条目;详情页 / 搜索页无对应项。
fn active_entry(app: &App) -> Option<Entry> {
    match app.route() {
        Route::Home => Some(Entry::Home),
        Route::Library(crate::nav::LibraryKind::Movies) => Some(Entry::Movies),
        Route::Library(crate::nav::LibraryKind::TvShows) => Some(Entry::TvShows),
        Route::Browse(id) => Some(Entry::Source(*id)),
        Route::Detail(_) | Route::Search => None,
        Route::Favorites => Some(Entry::Favorites),
        Route::Settings => Some(Entry::Settings),
    }
}

/// 侧边栏。
pub fn sidebar(app: &App) -> Element<'_, Message> {
    let active = active_entry(app);

    let mut col = column![
        container(text("Mona").size(20).color(theme::TEXT))
            .padding(Padding {
                top: 4.0,
                right: 10.0,
                bottom: 22.0,
                left: 10.0,
            }),
        group_header("媒体库"),
        item(Entry::Home, "主页", Route::Home, active),
        item(
            Entry::Movies,
            "电影",
            Route::Library(LibraryKind::Movies),
            active
        ),
        item(
            Entry::TvShows,
            "剧集",
            Route::Library(LibraryKind::TvShows),
            active
        ),
        group_header("媒体源"),
    ]
    .spacing(2);

    if app.sources().is_empty() {
        col = col.push(
            container(text("还没有媒体源").size(12).color(theme::TEXT_MUTED))
                .padding([6.0, 10.0]),
        );
    } else {
        for source in app.sources() {
            col = col.push(item(
                Entry::Source(source.id),
                &source.name,
                Route::Browse(source.id),
                active,
            ));
        }
    }

    col = col
        .push(group_header("更多"))
        .push(item(Entry::Favorites, "收藏", Route::Favorites, active))
        .push(item(Entry::Settings, "设置", Route::Settings, active));

    container(col)
        .width(Length::Fixed(260.0))
        .height(Fill)
        .padding(12.0)
        .style(|_| container::Style {
            background: Some(theme::SIDEBAR_BG.into()),
            ..Default::default()
        })
        .into()
}

/// 侧边栏分组标题。
fn group_header(title: &str) -> Element<'_, Message> {
    container(text(title).size(11).color(theme::TEXT_MUTED))
        .padding(Padding {
            top: 14.0,
            right: 10.0,
            bottom: 6.0,
            left: 10.0,
        })
        .into()
}

/// 单个侧边栏条目。
fn item<'a>(
    entry: Entry,
    label: &'a str,
    route: Route,
    active: Option<Entry>,
) -> Element<'a, Message> {
    let is_active = active == Some(entry);
    button(
        container(
            text(label)
                .size(14)
                .color(if is_active { theme::ACCENT } else { theme::TEXT }),
        )
        .width(Fill)
        .padding([8.0, 10.0]),
    )
    .width(Fill)
    .style(theme::nav_item(is_active))
    .on_press(Message::Navigate(route))
    .into()
}
