//! 媒体详情页:hero + 元数据 + 操作按钮 + 剧集列表。

use iced::widget::{button, column, container, row, text};
use iced::{Center, Color, Element, Length};

use crate::app::Message;
use crate::nav::MediaId;
use crate::theme;

/// 详情页(占位:元数据层接入后由媒体条目填充)。
pub fn view(_id: MediaId) -> Element<'static, Message> {
    column![
        // Hero 区
        container(text("海报占位").size(14).color(theme::TEXT_MUTED).center())
            .width(Length::Fill)
            .height(Length::Fixed(240.0))
            .style(theme::surface),
        // 标题 + 元数据
        text("占位影片标题").size(24).color(theme::TEXT),
        text("2024 · 2 小时 15 分 · ★ 7.8")
            .size(13)
            .color(theme::TEXT_MUTED),
        // 操作按钮
        row![
            button(text("播放").size(14).color(Color::WHITE))
                .padding([9.0, 22.0])
                .style(theme::primary),
            button(container(text("收藏").size(14).color(theme::TEXT)).padding([9.0, 22.0]))
                .style(theme::nav_item(false)),
            text("播放器未接入(下一阶段)")
                .size(12)
                .color(theme::TEXT_MUTED),
        ]
        .spacing(10)
        .align_y(Center),
        // 简介
        text("简介占位:数据层接入后展示剧情摘要与演职员信息。")
            .size(14)
            .color(theme::TEXT_MUTED),
        // 剧集
        text("剧集").size(18).color(theme::TEXT),
        episode_row("占位剧集 1", "S01E01 · 45 分钟"),
        episode_row("占位剧集 2", "S01E02 · 45 分钟"),
        episode_row("占位剧集 3", "S01E03 · 45 分钟"),
    ]
    .spacing(12)
    .into()
}

/// 剧集列表行。
fn episode_row(title: &'static str, meta: &'static str) -> Element<'static, Message> {
    container(
        row![
            container(text("集").size(12).color(theme::TEXT_MUTED).center())
                .width(Length::Fixed(96.0))
                .height(Length::Fixed(54.0))
                .style(theme::surface),
            column![
                text(title).size(14).color(theme::TEXT),
                text(meta).size(12).color(theme::TEXT_MUTED),
            ]
            .spacing(4),
        ]
        .spacing(12)
        .align_y(Center),
    )
    .width(Length::Fill)
    .padding(8.0)
    .style(theme::surface)
    .into()
}
