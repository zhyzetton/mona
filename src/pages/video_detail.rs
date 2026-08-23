use crate::app::{AppState, Message};
use crate::components::poster::poster_widget;
use iced::widget::{button, column, container, row, text};

pub fn view(state: &AppState) -> iced::Element<'_, Message> {
    let Some(id) = state.selected_id else {
        return text!("未选择媒体").into();
    };
    let Some(media) = state.medias.iter().find(|m| m.id == Some(id)) else {
        return text!("媒体不存在").into();
    };
    let poster = poster_widget(media.poster_path.as_deref());
    let header = row![button("返回").on_press(Message::BackToLibrary)];
    column![
        header,
        row![
            poster,
            column![
                text(&media.title),
                text(media.year.as_deref().unwrap_or(""))
            ]
            .spacing(10)
        ].spacing(10),
        button("▶ 播放").on_press(Message::PlayerVideo)
    ]
    .spacing(10)
    .into()
}
