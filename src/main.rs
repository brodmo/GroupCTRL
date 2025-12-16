mod action;
mod app;
mod hotkeys;
mod open;
mod util;

use crate::action::Action::OpenApp;
use crate::app::App;
use crate::hotkeys::{Hotkey, HotkeyManager};
use anyhow::Result;
use global_hotkey::hotkey::{Code, Modifiers};
use iced::widget::{Button, button};
use simplelog::*;
use std::fs;
use std::fs::File;

struct GroupCtrl {
    hotkey_manager: HotkeyManager,
}

#[derive(Clone)]
enum Message {
    RegisterHotkey,
}

impl Default for GroupCtrl {
    fn default() -> Self {
        Self {
            hotkey_manager: HotkeyManager::new().unwrap(),
        }
    }
}

impl GroupCtrl {
    fn update(&mut self, message: Message) {
        match message {
            Message::RegisterHotkey => {
                let app = App::new("com.apple.finder");
                let hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyF);
                self.hotkey_manager
                    .bind_hotkey(hotkey, OpenApp(app))
                    .unwrap();
            }
        }
    }

    fn view(&self) -> Button<'_, Message> {
        button("Register Hotkey").on_press(Message::RegisterHotkey)
    }
}

fn setup_logging() -> Result<()> {
    fs::create_dir_all("logs")?;
    let log_file = File::create("logs/app.log")?;
    let config = ConfigBuilder::new().build();
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            config.clone(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(LevelFilter::Debug, config, log_file),
    ])?;
    Ok(())
}

fn main() -> iced::Result {
    setup_logging().expect("Logging setup failed");
    iced::run(GroupCtrl::update, GroupCtrl::view)
}
