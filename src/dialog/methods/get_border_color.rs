use crate::dialog::{Dialog, DialogType};
use ratatui::style::Color;

impl<'a> Dialog<'a> {
    pub fn get_border_color(&self) -> Color {
        match self.dialog_type {
            DialogType::Info => Color::Cyan,
            DialogType::Success => Color::Green,
            DialogType::Warning => Color::Yellow,
            DialogType::Error => Color::Red,
            DialogType::Confirm => Color::Blue,
        }
    }
}
