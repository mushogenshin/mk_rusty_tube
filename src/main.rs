mod rusty_tube;
use iced::{settings, window, Sandbox};
use rusty_tube::RustyTubeDL;

fn main() -> iced::Result {
    let window_settings = window::Settings {
        size: (600, 170),
        min_size: Some((570, 155)),
        ..window::Settings::default()
    };
    let app_settings = settings::Settings {
        window: window_settings,
        ..settings::Settings::default()
    };
    RustyTubeDL::run(app_settings)
}
