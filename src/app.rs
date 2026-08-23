use std::fs;
use crate::components::{header, sider};
use crate::pages::{home, library, settings, video_detail};
use iced::widget::{column, container, row};
use iced::{Color, Length};
use std::path::PathBuf;
use std::process::Command;
use crate::{config, database, scan};
use crate::config::Config;
use crate::database::repository;
use crate::media::model::{Media, MediaType};
use crate::media::player;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Page {
    Home,
    Library,
    Settings,
}

#[derive(Clone, Debug)]
pub enum Message {
    NavigateTo(Page),
    SearchChanged(String),
    SettingsPathInputChanged(String),
    AddLocalDir,
    RemoveLocalDir(usize),   // 点击第几个删除
    ScanLocalVideos,
    ScanFinished(i64),
    OpenDetail(i64),
    BackToLibrary,
    PlayerVideo
}

pub struct AppState {
    pub current_page: Page,
    pub search_query: String,
    pub config: config::Config,
    pub settings_path_input: String,
    pub medias: Vec<Media>,
    pub selected_id: Option<i64>
}

impl AppState {
    fn new() -> Self {
        let medias = load_medias();
        Self {
            current_page: Page::Home,
            search_query: String::new(),
            config: Config::load(),
            settings_path_input: String::default(),
            medias,
            selected_id: None
        }
    }
}

fn update(app_state: &mut AppState, message: Message) -> iced::Task<Message> {
    match message {
        Message::NavigateTo(page) => {
            app_state.current_page = page;
            iced::Task::none()
        }
        Message::SearchChanged(query) => {
            app_state.search_query = query;
            iced::Task::none()
        }
        Message::SettingsPathInputChanged(v) => {
            app_state.settings_path_input = v;
            iced::Task::none()
        }
        Message::AddLocalDir => {
            let p = app_state.settings_path_input.trim().to_string();
            if !p.is_empty() {
                app_state.config.local_dirs.push(PathBuf::from(p));
                app_state.config.save();
            }
            app_state.settings_path_input.clear();
            iced::Task::none()
        }
        Message::RemoveLocalDir(i) => {
            if i < app_state.config.local_dirs.len() {
                app_state.config.local_dirs.remove(i);
                app_state.config.save();
            }
            iced::Task::none()
        }
        Message::ScanLocalVideos => {
            let dirs = app_state.config.local_dirs.clone();
            iced::Task::perform(scan::scan_job(dirs), Message::ScanFinished)
        }
        Message::ScanFinished(count) => {
            println!("扫描完成, {count} 成功");
            app_state.medias = load_medias();
            iced::Task::none()
        }
        Message::OpenDetail(id) => {
            app_state.selected_id = Some(id);
            iced::Task::none()
        }
        Message::BackToLibrary => {
            app_state.current_page = Page::Library;
            app_state.selected_id = None;
            iced::Task::none()
        }
        Message::PlayerVideo => {
            let Some(media) = app_state.medias.iter()
                .find(|m| m.id == app_state.selected_id) else { return iced::Task::none(); };
            let result = match &app_state.config.player_name {
                Some(player_name) => Command::new(player_name).arg(&media.file_path).spawn(),
                None => player::open_with_system_default(&media.file_path)
            };
            iced::Task::none()
        }
    }
}

pub(crate) fn load_medias() -> Vec<Media> {
    database::connection::open()
        .ok()
        .and_then(|conn| repository::find_all(&conn).ok())
        .unwrap_or_default()
}

fn view(app_state: &AppState) -> iced::Element<'_, Message> {
    container(
        row![
            sider::view(&app_state),
            container("")
                .width(1)
                .height(Length::Fill)
                .style(|_theme| container::Style {
                    background: Some(iced::Background::Color(Color::from_rgb8(238, 238, 239))),
                    ..Default::default()
                }),
            column![
                header::view(app_state),
                match app_state.current_page {
                    Page::Home => home::view(app_state),
                    Page::Library => library::view(app_state),
                    Page::Settings => settings::view(app_state),
                }
            ]
        ]
        .width(Length::Fill),
    )
    .into()
}

pub fn run() -> iced::Result {
    iced::application(AppState::new, update, view).run()
}
