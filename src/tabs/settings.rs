use tui::{
    prelude::*,
    widgets::{calendar::CalendarEventStore, *},
};

use crate::settings::Settings;

pub struct SettingsTab{
    settings: Settings
}

impl SettingsTab{
    pub fn new(settings: Settings) -> SettingsTab{
        SettingsTab { settings }
    }

    fn render_lines() {
        let options = vec![
            ""
        ];
    }
}

impl Widget for SettingsTab{
    fn render(self, area: Rect, buf: &mut Buffer) {
        todo!()
    }
}