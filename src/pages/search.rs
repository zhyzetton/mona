//! 搜索页(占位:检索待数据层接入)。

use iced::Element;

use crate::app::{App, Message};
use crate::pages::empty_state;

/// 搜索页。
pub fn view(app: &App) -> Element<'_, Message> {
    if app.search().is_empty() {
        empty_state(
            "搜索媒体".into(),
            "输入关键词搜索影片与剧集(检索待数据层接入)。",
        )
    } else {
        empty_state(
            format!("搜索 “{}”", app.search()),
            "检索尚未接入,数据层完成后生效。",
        )
    }
}
