mod app;
mod pages;
mod components;
mod database;

mod media;
mod config;
mod scan;

fn main() -> iced::Result {
    app::run()
}
