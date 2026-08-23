use crate::app::{AppState, Message};
use iced::widget::{button, column, row, text, text_input};

pub fn view(state: &AppState) -> iced::Element<'_, Message> {
    let dirs = state
        .config
        .local_dirs
        .iter()
        .enumerate()
        .map(|(i, dir)| {
            row![
                text(dir.to_string_lossy()),
                button("删除").on_press(Message::RemoveLocalDir(i)),
            ]
            .spacing(8).into()
        })
        .collect::<Vec<_>>();

    column![
        row![text!("本地媒体库目录"), button("扫面本地").on_press(Message::ScanLocalVideos)],
        column(dirs).spacing(4),
        row![
            text_input("输入目录路径", &state.settings_path_input)
                .on_input(Message::SettingsPathInputChanged),
            button("添加").on_press(Message::AddLocalDir)
        ]
    ]
    .into()
}
