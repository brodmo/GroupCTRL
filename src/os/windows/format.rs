use global_hotkey::hotkey::Modifiers;

use crate::os::prelude::FormatTrait;

pub struct Format;

impl FormatTrait for Format {
    fn modifiers() -> [(Modifiers, &'static str); 4] {
        [
            (Modifiers::CONTROL, "Ctrl+"),
            (Modifiers::SUPER, "Win+"),
            (Modifiers::ALT, "Alt+"),
            (Modifiers::SHIFT, "Shift+"),
        ]
    }
}
