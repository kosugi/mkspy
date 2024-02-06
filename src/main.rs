#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::widget::Text;
use iced::{executor, time, window, Application, Command, Element, Font, Settings, Theme};

use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, VIRTUAL_KEY, VK_CONTROL, VK_LWIN, VK_MENU, VK_RWIN, VK_SHIFT,
};

fn choose<T>(condition: bool, lhs: T, rhs: T) -> T {
    if condition {
        lhs
    } else {
        rhs
    }
}

fn is_key_pressed(key: VIRTUAL_KEY) -> bool {
    unsafe { GetAsyncKeyState(key.0.into()) & 0x8000u16 as i16 != 0 }
}

fn is_shift_pressed() -> bool {
    is_key_pressed(VK_SHIFT)
}

fn is_ctrl_pressed() -> bool {
    is_key_pressed(VK_CONTROL)
}

fn is_alt_pressed() -> bool {
    is_key_pressed(VK_MENU)
}

fn is_win_pressed() -> bool {
    is_key_pressed(VK_LWIN) || is_key_pressed(VK_RWIN)
}

struct MyApp {
    is_shift_pressed: bool,
    is_ctrl_pressed: bool,
    is_alt_pressed: bool,
    is_win_pressed: bool,
}

impl MyApp {
    fn default() -> Self {
        Self {
            is_shift_pressed: false,
            is_ctrl_pressed: false,
            is_alt_pressed: false,
            is_win_pressed: false,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Tick(chrono::DateTime<chrono::Utc>),
}

impl Application for MyApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (MyApp, Command<Message>) {
        (MyApp::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Modifier Key Spy")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick(_local_time) => {
                self.is_shift_pressed = is_shift_pressed();
                self.is_ctrl_pressed = is_ctrl_pressed();
                self.is_alt_pressed = is_alt_pressed();
                self.is_win_pressed = is_win_pressed();
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let s = format!(
            "[{}][{}][{}][{}]",
            choose(self.is_shift_pressed, "SHIFT", "     "),
            choose(self.is_ctrl_pressed, "CTRL", "    "),
            choose(self.is_alt_pressed, "ALT", "   "),
            choose(self.is_win_pressed, "WIN", "   "),
        );
        Text::new(s).font(Font::MONOSPACE).size(16).into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        time::every(std::time::Duration::from_millis(100))
            .map(|_| Message::Tick(chrono::Utc::now()))
    }
}

fn main() -> iced::Result {
    MyApp::run(Settings {
        window: window::Settings {
            resizable: false,
            size: (190, 25),
            level: window::Level::AlwaysOnTop,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}
