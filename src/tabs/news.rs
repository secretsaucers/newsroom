use crate::{newsroomcore::{newsroomstate::NewsroomState, newsarticle::NewsArticle}, settings::Theme};
use itertools::Itertools;
use log::info;
use tui::{
    prelude::*,
    widgets::*,
};
use textwrap::{wrap, Options};

pub struct NewsTab<'a>{
    nrs: &'a NewsroomState,
    list_state: ListState,
    theme: Theme,
}

impl NewsTab <'_>{
    /// Wraps text to fit in a certain line width
    /// 
    /// Arguments
    /// * text - A string representing the text to be wrapped
    /// * width - The line width to wrap to
    fn wrap_text(text_raw: String, width: usize) -> String {
        let options = Options::new(width);

        // let text_raw = format!("{}: {}", x.source.name, x.title);
        let text_wrapped = wrap(text_raw.as_str(), &options)
        .iter()
        .map(|s| s.to_string()).collect_vec();
        text_wrapped.join("\n")
    }

    pub fn new(nrs: &NewsroomState, theme: Theme, index: Option<usize>) -> NewsTab {
        let mut list_state = ListState::default();
        list_state.select(index);
        NewsTab { nrs, list_state, theme}
    }

    pub fn render_news_list(&mut self, area: Rect, buf: &mut Buffer, articles: Vec<NewsArticle>) {
            let items: Vec<ListItem> = articles
            .iter()
            .map(|x| {
                let text = Text::from(NewsTab::wrap_text(format!("{}: {}", x.source.name, x.title), area.width as usize));
                ListItem::new(text)
            }
            )
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

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use super::*;
    // Text wrapping
    #[test]
    fn test_wrap() {
        let test_string: String = "The quick brown fox jumped over the lazy dog".to_string();
        let result = NewsTab::wrap_text(test_string, 20);
        println!("{}", result);
        assert_eq!(result.lines().collect_vec().len(), 3);

        let test_string: String = "The quick".to_string();
        let result = NewsTab::wrap_text(test_string, 20);
        println!("{}", result);
        assert_eq!(result.lines().collect_vec().len(), 1)
    }
}