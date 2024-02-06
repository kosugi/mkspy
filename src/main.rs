use iced::widget::{button, Button, Column, Container, Text};
use iced::{executor, time, Application, Command, Element, Settings, Theme};

use windows::core::*;
use windows::Win32::UI::WindowsAndMessaging::*;

struct MyApp {
    key_state: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(chrono::DateTime<chrono::Utc>),
}

impl Application for MyApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (MyApp, Command<Message>) {
        (MyApp { key_state: 0 }, Command::none())
    }

    fn title(&self) -> String {
        String::from("MyApp")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick(local_time) => println!("pressed"),
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        Text::new("abc").into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        time::every(std::time::Duration::from_secs(1)).map(|_| Message::Tick(chrono::Utc::now()))
    }
}

fn main() -> iced::Result {
    MyApp::run(Settings::default())
}
