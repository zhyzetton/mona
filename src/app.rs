use crate::components::{header, sider};
use crate::pages::{home, library, settings};
use iced::Length;
use iced::widget::{column, container, row};
use std::path::PathBuf;

pub struct FileItem {
    pub name: String,
    pub path: PathBuf,
}

fn read_files() -> Vec<FileItem> {
    std::fs::read_dir("C:\\Users\\zhyze\\Projects")
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.ok()?;

            if entry.file_type().ok()?.is_file() {
                Some(FileItem {
                    name: entry.file_name().to_string_lossy().to_string(),
                    path: entry.path(),
                })
            } else {
                None
            }
        })
        .collect()
}

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
}

pub struct AppState {
    pub current_page: Page,
    pub files: Vec<FileItem>,
    pub search_query: String,
}

impl AppState {
    fn new() -> Self {
        let files = read_files();

        Self {
            current_page: Page::Home,
            files,
            search_query: String::new(),
        }
    }
}

fn update(app_state: &mut AppState, message: Message) {
    match message {
        Message::NavigateTo(page) => {
            app_state.current_page = page;
        }
        Message::SearchChanged(query) => {
            app_state.search_query = query;
        }
    }
}

fn view(app_state: &AppState) -> iced::Element<'_, Message> {
    container(
        row![
            sider::view(&app_state),
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
