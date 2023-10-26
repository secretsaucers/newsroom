use std::rc::Rc;

use itertools::Itertools;
use tui::{prelude::*, widgets::*};

use crate::{tabs::*, app::App};

pub struct Root<'a> {
    context: &'a App,
}

impl<'a> Root<'a> {
    pub fn new(context: &'a App) -> Self {
        Root { context }
    }
}

impl Widget for Root<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::new().style(self.context.settings.theme.root).render(area, buf);
        let area = layout(area, Direction::Vertical, vec![1, 0, 1]);
        self.render_title_bar(area[0], buf);
        self.render_selected_tab(area[1], buf);
        self.render_bottom_bar(area[2], buf);
    }
}

impl Root<'_> {
    fn render_title_bar(&self, area: Rect, buf: &mut Buffer) {
        let area = layout(area, Direction::Horizontal, vec![0, 45]);

        Paragraph::new(Span::styled("Newsroom", self.context.settings.theme.app_title)).render(area[0], buf);
        let titles = vec!["", " News ", " Settings "];
        Tabs::new(titles)
            .style(self.context.settings.theme.tabs)
            .highlight_style(self.context.settings.theme.tabs_selected)
            .select(self.context.tab.into())
            .divider("")
            .render(area[1], buf);
    }

    fn render_selected_tab(&self, area: Rect, buf: &mut Buffer) {
        match self.context.tab {
            0 => NewsTab::new(&self.context.newsroom_state, self.context.settings.theme.clone(), self.context.row).render(area, buf),
            1 => SettingsTab::new(self.context.settings.clone()).render(area, buf),
            _ => unreachable!(),
        };
    }

    fn render_bottom_bar(&self, area: Rect, buf: &mut Buffer) {
        let keys = [
            ("Q/Esc", "Quit"),
            ("Tab", "Next Tab"),
            ("↑/k", "Up"),
            ("↓/j", "Down"),
        ];
        let spans = keys
            .iter()
            .flat_map(|(key, desc)| {
                let key = Span::styled(format!(" {} ", key), self.context.settings.theme.keybinding.key);
                let desc = Span::styled(format!(" {} ", desc), self.context.settings.theme.keybinding.description);
                [key, desc]
            })
            .collect_vec();
        Paragraph::new(Line::from(spans))
            .alignment(Alignment::Center)
            .fg(Color::Indexed(236))
            .bg(Color::Indexed(232))
            .render(area, buf);
    }
}

/// simple helper method to split an area into multiple sub-areas
pub fn layout(area: Rect, direction: Direction, heights: Vec<u16>) -> Rc<[Rect]> {
    let constraints = heights
        .iter()
        .map(|&h| {
            if h > 0 {
                Constraint::Length(h)
            } else {
                Constraint::Min(0)
            }
        })
        .collect_vec();
    Layout::default()
        .direction(direction)
        .constraints(constraints)
        .split(area)
}