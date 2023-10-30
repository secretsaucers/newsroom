use crate::{newsroomcore::{newsroomstate::NewsroomState, newsarticle::NewsArticle}, settings::Theme};
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

    pub fn render_news_list(&mut self, area: Rect, buf: &mut Buffer, articles: Vec<NewsArticle>) {
            let items: Vec<ListItem> = articles
            .iter()
            .map(|x| ListItem::new(format!("{}: {}", x.source.name,x.title)))
            .collect();
            let list_widget = List::new(items)
                .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
                .style(self.theme.content).block(Block::new().borders(Borders::ALL).border_type(BorderType::Rounded));

            tui::widgets::StatefulWidget::render(list_widget, area, buf, &mut self.list_state); // Render widget
    }

    pub fn render_selected_description(&self, area: Rect, buf: &mut Buffer, article: NewsArticle){
        let text = article.summary;
        Paragraph::new(text).wrap(Wrap { trim: true }).style(self.theme.content).block(Block::new().borders(Borders::ALL).border_type(BorderType::Rounded).title("Description")).render(area, buf);
    }
}

impl Widget for NewsTab <'_>{
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        match self.nrs {
            NewsroomState::DisplayMedia(articles) => {
                match self.list_state.selected() {
                    Some(index) => {
                        // The user has selected a row, display an extended description below the list
                        let layout = Layout::new()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Percentage(80),
                            Constraint::Percentage(20),
                        ])
                        .split(area);
                        
                        // Render using layout
                        self.render_news_list(layout[0], buf, articles.clone().to_vec());
                        self.render_selected_description(layout[1], buf, articles[index].clone());
                    },
                    None => {
                        // The user has not selected a row yet, do not render an extended description
                        // Use the whole area for the list
                        self.render_news_list(area, buf, articles.clone().to_vec());
                    },
                }
            },
            NewsroomState::FetchMedia(_) => Paragraph::new("Loading news . . .").style(self.theme.description).alignment(Alignment::Center).render(area, buf),
            NewsroomState::Homescreen => Paragraph::new("Press 'l' to load news articles").alignment(Alignment::Center).render(area, buf),
            _ => {},
        }
    }
}