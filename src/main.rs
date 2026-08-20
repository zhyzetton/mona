use iced::Error;

mod app;
mod pages;
mod components;
mod database;

mod media;

fn main() -> iced::Result {
    app::run()
}
