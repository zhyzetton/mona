use crate::pages::{home, library, settings};

pub enum Page {
    Home,
    Library,
    Settings,
}

pub enum Message {
    NavigateTo(Page),
}

pub struct AppState {
    current_page: Page,
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
    match app_state.current_page {
        Page::Home => home::view(app_state),
        Page::Library => library::view(app_state),
        Page::Settings => settings::view(app_state),
    }
}

pub fn run() -> iced::Result {
    iced::application(AppState::new, update, view).run()
}
