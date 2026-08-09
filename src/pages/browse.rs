//! 媒体源浏览页:面包屑 + 文件夹 / 影片网格。

use iced::widget::{column, grid, text, Grid};
use iced::Element;

use crate::app::{App, Message};
use crate::nav::SourceId;
use crate::pages::placeholder_card;
use crate::theme;

/// 占位文件夹。
const FOLDER_TITLES: [&str; 4] = ["文件夹 1", "文件夹 2", "文件夹 3", "文件夹 4"];

/// 媒体源浏览页。
pub fn view(app: &App, id: SourceId) -> Element<'_, Message> {
    let source_name = app
        .sources()
        .iter()
        .find(|s| s.id == id)
        .map(|s| s.name.as_str())
        .unwrap_or("未知媒体源");

    let mut folder_grid = Grid::new()
        .fluid(160.0)
        .spacing(12.0)
        .height(grid::Sizing::AspectRatio(1.0));
    for title in FOLDER_TITLES {
        folder_grid = folder_grid.push(placeholder_card(title, "文件夹"));
    }

    let mut movie_grid = Grid::new()
        .fluid(160.0)
        .spacing(12.0)
        .height(grid::Sizing::AspectRatio(2.0 / 3.0));
    for title in crate::pages::library::PLACEHOLDER_TITLES {
        movie_grid = movie_grid.push(placeholder_card(title, "影片"));
    }

    column![
        text(format!("{source_name} / 根目录"))
            .size(13)
            .color(theme::TEXT_MUTED),
        text("文件夹").size(18).color(theme::TEXT),
        folder_grid,
        text("影片").size(18).color(theme::TEXT),
        movie_grid,
    ]
    .spacing(12)
    .into()
}
