use std::sync::{Arc, Mutex};

use crate::models::Hotkey;

pub type HotkeyCallback = Arc<dyn Fn(Hotkey) + Send + Sync>;

#[derive(Clone, Default)]
pub struct SharedHotkeyCallback(Arc<Mutex<Option<HotkeyCallback>>>);

impl SharedHotkeyCallback {
    pub fn set(&self, callback: Option<HotkeyCallback>) {
        *self.0.lock().unwrap() = callback;
    }

    pub(super) fn get(&self) -> Option<HotkeyCallback> {
        self.0.lock().unwrap().clone()
    }
}
