//! Mona 深色主题:调色板 + 通用样式闭包。

use iced::theme::Palette;
use iced::widget::{button, container};
use iced::{color, Border, Color, Theme};

/// 强调色(类 Infuse 蓝)。
pub const ACCENT: Color = color!(0x4C8DFF);
/// 应用背景。
pub const BACKGROUND: Color = color!(0x0B0E14);
/// 侧边栏背景(比主背景略浅一档)。
pub const SIDEBAR_BG: Color = color!(0x0E1220);
/// 卡片 / 面板表面。
pub const SURFACE: Color = color!(0x141A28);
/// hover 表面。
pub const SURFACE_HOVER: Color = color!(0x1C2436);
/// 选中表面。
pub const SURFACE_ACTIVE: Color = color!(0x233253);
/// 主文字。
pub const TEXT: Color = color!(0xE8EAED);
/// 次级文字。
pub const TEXT_MUTED: Color = color!(0x8B94A7);

/// 应用主题。
pub fn mona() -> Theme {
    Theme::custom(
        "Mona Dark",
        Palette {
            background: BACKGROUND,
            text: TEXT,
            primary: ACCENT,
            success: color!(0x3ECF8E),
            warning: color!(0xFFC53D),
            danger: color!(0xFF5C5C),
        },
    )
}

/// 通用卡片表面样式。
pub fn surface(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(SURFACE.into()),
        border: Border::default().rounded(8.0),
        ..Default::default()
    }
}

/// 导航 / 切换类条目按钮样式(侧边栏、视图切换)。
pub fn nav_item(active: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let background = match (active, status) {
            (true, _) => Some(SURFACE_ACTIVE.into()),
            (false, button::Status::Hovered) => Some(SURFACE_HOVER.into()),
            (false, button::Status::Pressed) => Some(SURFACE.into()),
            _ => None,
        };
        button::Style {
            background,
            text_color: if active { ACCENT } else { TEXT },
            border: Border::default().rounded(6.0),
            ..Default::default()
        }
    }
}

/// 图标 / 次要按钮样式(返回键等)。
pub fn icon_button(_theme: &Theme, status: button::Status) -> button::Style {
    let background = match status {
        button::Status::Hovered | button::Status::Pressed => Some(SURFACE_HOVER.into()),
        _ => None,
    };
    button::Style {
        background,
        text_color: if matches!(status, button::Status::Disabled) {
            TEXT_MUTED
        } else {
            TEXT
        },
        border: Border::default().rounded(6.0),
        ..Default::default()
    }
}

/// 主操作按钮样式(播放等)。
pub fn primary(_theme: &Theme, status: button::Status) -> button::Style {
    let (background, text_color) = match status {
        button::Status::Disabled => (SURFACE, TEXT_MUTED),
        button::Status::Hovered => (color!(0x3E7DF0), Color::WHITE),
        _ => (ACCENT, Color::WHITE),
    };
    button::Style {
        background: Some(background.into()),
        text_color,
        border: Border::default().rounded(6.0),
        ..Default::default()
    }
}
