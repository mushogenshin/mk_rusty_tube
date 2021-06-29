use iced::{
    button, text_input, Align, Button, Checkbox, Column, Container, Element, HorizontalAlignment,
    Length, Row, Sandbox, Text, TextInput,
};
use regex::Regex;
use shellexpand;
use std::path::PathBuf;
use std::process::Command;

#[derive(Default)]
pub struct SimpleYouTubeDL {
    url: String,
    url_input: text_input::State,
    audio_only: bool,
    show_results_button: button::State,
    debug_line: String,
    debug_line_input: text_input::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    URLInputTextChanged(String),
    URLInputReturnPressed,
    ToggleAudioOnly(bool),
    ShowResultsPressed,
    ProcessOutputTextChanged(String),
}

impl Sandbox for SimpleYouTubeDL {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("mk Rusty Tube Downloader")
    }

    fn view(&mut self) -> Element<Message> {
        let element = Column::new()
            .padding(20)
            .spacing(10)
            .align_items(Align::Center)
            .push(
                Row::new()
                    .align_items(Align::Center)
                    .spacing(13)
                    .push(
                        Checkbox::new(self.audio_only, "Audio Only", Message::ToggleAudioOnly)
                            .size(16).text_size(16),
                    )
                    .push(
                        TextInput::new(
                            &mut self.url_input,
                            "Paste the video/playlist URL here and press Enter",
                            &self.url,
                            Message::URLInputTextChanged,
                        )
                        .on_submit(Message::URLInputReturnPressed)
                        .padding(10)
                        .size(20),
                    ),
            )
            .push(
                Row::new()
                    .align_items(Align::Center)
                    .spacing(10)
                    .push(
                        TextInput::new(
                            &mut self.debug_line_input,
                            "Download status",
                            &self.debug_line,
                            Message::ProcessOutputTextChanged,
                        )
                        .padding(5)
                        .size(13)
                        .width(Length::from(110)),
                    )
                    .push(
                        Button::new(
                            &mut self.show_results_button,
                            Text::new("Show").horizontal_alignment(HorizontalAlignment::Center),
                        )
                        .padding(5)
                        .width(Length::from(100))
                        .on_press(Message::ShowResultsPressed),
                    ),
            );
    
        Container::new(element).height(Length::Fill).center_y().into()
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::ToggleAudioOnly(toggle) => self.audio_only = toggle,
            Message::URLInputTextChanged(new_value) => {
                self.debug_line = String::from(""); // clear
                self.url = new_value;
            }
            Message::URLInputReturnPressed => {
                let status = self.download();
                if status.success() {
                    self.debug_line = String::from("Completed");
                } else {
                    self.debug_line = String::from("(!)Failed");
                }
            }
            Message::ShowResultsPressed => {
                SimpleYouTubeDL::show_results(SimpleYouTubeDL::get_download_folder_path());
            }
            Message::ProcessOutputTextChanged(new_value) => self.debug_line = new_value,
        }
    }
}

impl SimpleYouTubeDL {
    fn download(&self) -> std::process::ExitStatus {
        let child: std::process::Output;
        let re1 = Regex::new(r".*playlist\?list=.*").unwrap();
        let output_path = SimpleYouTubeDL::get_download_folder_path();
        if re1.is_match(&self.url) {
            child = SimpleYouTubeDL::download_playlist(&self.url, self.audio_only, output_path);
        } else {
            child = SimpleYouTubeDL::download_single_video(&self.url, self.audio_only, output_path);
        };
        child.status
    }

    fn download_single_video(url: &str, audio_only: bool, folder: PathBuf) -> std::process::Output {
        let mut cmd = Command::new("youtube-dl");
        if audio_only {
            cmd.args(&["-f", "140"]);
        } else {
        };

        cmd.arg("-ci")
            .arg("-o")
            .arg(folder.join("%(title)s.%(ext)s"))
            .arg(url)
            .output()
            .expect("failed to execute youtube-dl command")
    }

    fn download_playlist(url: &str, audio_only: bool, folder: PathBuf) -> std::process::Output {
        let mut cmd = Command::new("youtube-dl");
        if audio_only {
            cmd.args(&["-f", "140"]);
        } else {
        };

        cmd.arg("-ci")
            .arg("-o")
            .arg(folder.join("%(channel)s-%(title)s.%(ext)s"))
            .arg(url)
            .output()
            .expect("failed to execute youtube-dl command")
    }

    fn get_download_folder_path() -> PathBuf {
        // TODO: add options for the drives on which to save downloaded file to
        let path: &str = if cfg!(target_os = "windows") {
            "$USERPROFILE/Videos"
        } else if cfg!(target_os = "macos") {
            "$HOME/Movies"
        } else {
            unimplemented!();
        };
        PathBuf::from(shellexpand::env(path).unwrap().to_string())
    }

    fn show_results(path: PathBuf) {
        if cfg!(target_os = "windows") {
            Command::new("explorer")
                .arg(path)
                .output()
                .expect("failed to open Explorer");
        } else if cfg!(target_os = "macos") {
            Command::new("open")
                .arg(path)
                .output()
                .expect("failed to reveal in Finder");
        } else if cfg!(target_os = "linux") {
            Command::new("xdg-open")
                .arg(path)
                .output()
                .expect("failed to XDG open");
        } else {
            unimplemented!();
        }
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn target_os() {
//         // assert_eq!(1, 0);
//     }
// }
