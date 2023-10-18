use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, Paragraph, Wrap},
    Frame,
    text::Text,
};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let rects = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(5)
        .split(f.size());

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::Blue);
    let header_cells = ["Title"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);
    let rows = app.newsroom_articles.iter().map(|item| {
        let height = item.title
            .chars().filter(|c| *c == '\n').count()
            + 1;
        let cells = item.authors.iter().map(|c| Cell::from(c.clone()));
        Row::new(cells).height(height as u16).bottom_margin(1)
    });

    let mut rows_news: Vec<Row<'_>> = Vec::new();

    for news_article in &app.newsroom_articles {
        rows_news.push(Row::new(
            Text::from(news_article.title.clone()),
        ).bottom_margin(1));
    }

    let t = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Newsroom"))
        .highlight_style(selected_style)
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(50),
            Constraint::Length(30),
            Constraint::Min(10),
        ]);

    

    match &app.newsroom_state {
        crate::app::newsroomcore::newsroomstate::NewsroomState::Startup(splash_text) => f.render_widget(Paragraph::new(splash_text.clone()), rects[0]),
        crate::app::newsroomcore::newsroomstate::NewsroomState::homescreen => todo!(),
        crate::app::newsroomcore::newsroomstate::NewsroomState::fetch_media(_) => todo!(),
        crate::app::newsroomcore::newsroomstate::NewsroomState::return_media(_) => todo!(),
        crate::app::newsroomcore::newsroomstate::NewsroomState::display_media => f.render_stateful_widget(t, rects[0], &mut app.state),
        crate::app::newsroomcore::newsroomstate::NewsroomState::manage_settings => todo!(),
    }

}
