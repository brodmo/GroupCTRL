mod action;
mod app;
mod hotkeys;
mod open;

use crate::action::OpenApp;
use crate::app::App;
use crate::hotkeys::HotkeyManager;
use eframe::egui;
use eframe::egui::Button;
use global_hotkey::hotkey::{Code, HotKey, Modifiers};

struct GroupCtrl {
    hotkey_manager: HotkeyManager,
}

impl GroupCtrl {
    fn new() -> Self {
        Self {
            hotkey_manager: HotkeyManager::new(),
        }
    }
}

impl eframe::App for GroupCtrl {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let button = Button::new("Register Finder hotkey");
            if ui.add(button).clicked() {
                let hotkey = HotKey::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::KeyF);
                let action = OpenApp(App::new("com.apple.finder"));
                self.hotkey_manager.register_hotkey(hotkey, action);
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "GroupCtrl",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(GroupCtrl::new()))),
    )
}
