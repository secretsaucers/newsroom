use tui::{
    prelude::*,
    widgets::{*},
};

use crate::settings::Settings;

pub struct SettingsTab{
    settings: Settings
}

impl SettingsTab{
    pub fn new(settings: Settings) -> SettingsTab{
        SettingsTab { settings }
    }

    /// Render the sources stored in settings
    fn render_sources (&self, area: Rect, buf: &mut Buffer) {
        let items: Vec<ListItem> = self.settings.sources
        .iter()
        .map(|x| ListItem::new(x.name.to_string()))
        .collect();
        let list_widget = List::new(items)
        .style(self.settings.theme.content).block(Block::new().borders(Borders::ALL).border_type(BorderType::Rounded).title("Sources"));
        tui::widgets::Widget::render(list_widget, area, buf);
    }

    fn render_theme (&self, area: Rect, buf: &mut Buffer) {
        let text = self.settings.theme.name.clone();
        Paragraph::new(text).style(self.settings.theme.content).block(Block::new().borders(Borders::ALL).border_type(BorderType::Rounded).title("Active theme")).render(area, buf);
    }
}

impl Widget for SettingsTab{
    /// Render the settings tab widget
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::new()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(80),
            Constraint::Percentage(20),
        ])
        .split(area);
        self.render_sources(layout[0], buf);
        self.render_theme(layout[1], buf);
    }
}