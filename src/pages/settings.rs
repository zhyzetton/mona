//! 设置页:媒体源管理(占位) + 外观。

use iced::widget::{column, container, row, text, Space};
use iced::{Element, Fill};

use crate::app::{App, Message};
use crate::pages::section_header;
use crate::theme;

/// 设置页。
pub fn view(app: &App) -> Element<'_, Message> {
    let mut source_rows = column![].spacing(8);

    if app.sources().is_empty() {
        source_rows = source_rows.push(
            container(text("还没有媒体源").size(13).color(theme::TEXT_MUTED))
                .width(Fill)
                .padding(10.0)
                .style(theme::surface),
        );
    } else {
        for source in app.sources() {
            source_rows = source_rows.push(
                container(
                    row![
                        text(&source.name).size(14).color(theme::TEXT),
                        Space::new().width(Fill),
                        text("未连接(占位)").size(12).color(theme::TEXT_MUTED),
                    ]
                    .align_y(iced::Center),
                )
                .width(Fill)
                .padding(10.0)
                .style(theme::surface),
            );
        }
    }

    column![
        section_header("媒体源"),
        source_rows,
        text("添加媒体源(本地目录 / SMB / WebDAV)将在后端里程碑实现。")
            .size(12)
            .color(theme::TEXT_MUTED),
        section_header("外观"),
        container(text("深色主题 · 强调色 #4C8DFF").size(13).color(theme::TEXT))
            .width(Fill)
            .padding(10.0)
            .style(theme::surface),
    ]
    .spacing(12)
    .into()
}
