use crate::{newsroomcore::newsroomstate::NewsroomState, settings::Theme};
use tui::{
    prelude::*,
    widgets::*,
};

pub struct NewsTab<'a>{
    nrs: &'a NewsroomState,
    list_state: ListState,
    theme: Theme,
}

impl NewsTab <'_>{
    pub fn new(nrs: &NewsroomState, theme: Theme, index: Option<usize>) -> NewsTab {
        let mut list_state = ListState::default();
        list_state.select(index);
        NewsTab { nrs, list_state, theme}
    }
}

impl Widget for NewsTab <'_>{
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        match self.nrs {
            NewsroomState::DisplayMedia(articles) => {
                let items: Vec<ListItem> = articles
                .iter()
                .map(|x| ListItem::new(format!("{}: {}", x.source.name,x.title)))
                .collect();
                let list_widget = List::new(items)
                    .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
                    .style(self.theme.content).block(Block::new().borders(Borders::ALL).border_type(BorderType::Rounded));

                tui::widgets::StatefulWidget::render(list_widget, area, buf, &mut self.list_state); // Render widget
            },
            NewsroomState::FetchMedia(_) => Paragraph::new("Loading news . . .").style(self.theme.description).alignment(Alignment::Center).render(area, buf),
            NewsroomState::Homescreen => Paragraph::new("Press 'l' to load news articles").alignment(Alignment::Center).render(area, buf),
            _ => {},
        }
    }
}