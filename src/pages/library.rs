//! 媒体库页:海报网格 / 列表视图。

use iced::widget::{column, container, grid, row, text, Grid};
use iced::{Center, Element, Length};

use crate::app::{App, Message, ViewMode};
use crate::nav::LibraryKind;
use crate::pages::placeholder_card;
use crate::theme;

/// 占位标题(数据层接入后由媒体库填充)。
pub const PLACEHOLDER_TITLES: [&str; 8] = [
    "占位影片 1",
    "占位影片 2",
    "占位影片 3",
    "占位影片 4",
    "占位影片 5",
    "占位影片 6",
    "占位影片 7",
    "占位影片 8",
];

/// 媒体库页。
pub fn view(app: &App, kind: LibraryKind) -> Element<'_, Message> {
    match app.view_mode() {
        ViewMode::Grid => grid_view(kind),
        ViewMode::List => list_view(kind),
    }
}

/// 海报墙:fluid 网格,单元格按 2:3 海报比例约束。
fn grid_view(kind: LibraryKind) -> Element<'static, Message> {
    let mut grid = Grid::new()
        .fluid(160.0)
        .spacing(12.0)
        .height(grid::Sizing::AspectRatio(2.0 / 3.0));
    for title in PLACEHOLDER_TITLES {
        grid = grid.push(placeholder_card(title, kind.title()));
    }
    grid.into()
}

/// 列表视图:缩略图 + 标题行。
fn list_view(kind: LibraryKind) -> Element<'static, Message> {
    let rows = PLACEHOLDER_TITLES.iter().map(|t| {
        container(
            row![
                container(text(kind.title()).size(12).color(theme::TEXT_MUTED).center())
                    .width(Length::Fixed(96.0))
                    .height(Length::Fixed(56.0))
                    .style(theme::surface),
                text(*t).size(14).color(theme::TEXT),
            ]
            .spacing(12)
            .align_y(Center),
        )
        .width(Length::Fill)
        .padding(8.0)
        .style(theme::surface)
        .into()
    });

    column(rows).spacing(8).into()
}
