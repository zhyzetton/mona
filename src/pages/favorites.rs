//! 收藏页(占位)。

use iced::Element;

use crate::app::Message;
use crate::pages::empty_state;

/// 收藏页。
pub fn view() -> Element<'static, Message> {
    empty_state(
        "收藏为空".into(),
        "标记收藏后,条目会出现在这里(数据层待接入)。",
    )
}
