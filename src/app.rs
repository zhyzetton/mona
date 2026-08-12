use crate::components::sider;
use crate::pages::{home, library, settings};
use iced::widget::{container, row};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Page {
    Home,
    Library,
    Settings,
}

#[derive(Clone, Debug)]
pub enum Message {
    NavigateTo(Page),
}

pub struct AppState {
    pub(crate) current_page: Page,
}

impl AppState {
    fn new() -> Self {
        Self {
            current_page: Page::Home,
        }
    }
}

fn update(app_state: &mut AppState, message: Message) {
    match message {
        Message::NavigateTo(page) => {
            app_state.current_page = page;
        }
    }
}

fn view(app_state: &AppState) -> iced::Element<'_, Message> {
    container(
        row![
            sider::view(&app_state).width(200),
            match app_state.current_page {
                Page::Home => home::view(app_state),
                Page::Library => library::view(app_state),
                Page::Settings => settings::view(app_state),
            }
        ]
        .spacing(20),
    )
    .into()
}

pub fn run() -> iced::Result {
    iced::application(AppState::new, update, view).run()
}
