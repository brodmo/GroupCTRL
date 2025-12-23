use global_hotkey::hotkey::Modifiers;

use crate::os::App;

pub trait FormatTrait {
    fn modifiers() -> [(Modifiers, &'static str); 4];
}

pub trait AppPickerTrait {
    async fn pick_app() -> anyhow::Result<Option<App>>;
}

pub trait AppTrait {
    fn new(id: &str) -> Self;
    fn display(&self) -> String;
}

pub trait Openable {
    fn open(&self) -> anyhow::Result<()>;
}
