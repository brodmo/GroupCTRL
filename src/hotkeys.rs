use crate::action::Action;
use global_hotkey::hotkey::HotKey;
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager};
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

type HotkeyAction = Box<dyn Action + 'static>;

pub struct HotkeyManager {
    _global_manager: GlobalHotKeyManager,
    action_sender: mpsc::Sender<(u32, HotkeyAction)>,
}

impl HotkeyManager {
    fn listen_for_hotkeys(callback_receiver: mpsc::Receiver<(u32, HotkeyAction)>) {
        let mut hotkey_actions = HashMap::new();

        loop {
            while let Ok((id, callback)) = callback_receiver.try_recv() {
                hotkey_actions.insert(id, callback);
            }
            if let Ok(event) = GlobalHotKeyEvent::receiver().recv()
                && event.state == global_hotkey::HotKeyState::Pressed
                && let Some(action) = hotkey_actions.get(&event.id)
            {
                action.execute()
            }
        }
    }

    pub fn new() -> Self {
        let manager = GlobalHotKeyManager::new().unwrap();
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || Self::listen_for_hotkeys(rx));
        Self {
            _global_manager: manager,
            action_sender: tx,
        }
    }

    pub fn register_hotkey<T: Action + 'static>(&self, hotkey: HotKey, action: T) {
        self._global_manager.register(hotkey).unwrap();
        self.action_sender
            .send((hotkey.id(), Box::new(action)))
            .unwrap();
    }
}
