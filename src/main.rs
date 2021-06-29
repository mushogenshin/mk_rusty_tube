mod simple_youtube_dl;
use iced::{settings, window, Sandbox};
use simple_youtube_dl::SimpleYouTubeDL;

fn main() -> iced::Result {
    let window_settings = window::Settings {
        max_size: Some((600, 130)),
        ..window::Settings::default()
    };
    let app_settings = settings::Settings {
        window: window_settings,
        ..settings::Settings::default()
    };
    SimpleYouTubeDL::run(app_settings)
}
