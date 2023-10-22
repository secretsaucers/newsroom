


use tui::{
    backend::Backend,
    layout::Alignment,
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, ListState},
    Frame,
};

use crate::app::{newsroomcore, App};

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &App, frame: &mut Frame<'_, B>) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples

    let start_widget = Paragraph::new(
        "Welcome to Newsroom.\n\
                Press `l` to load some news \n\
                Press `Esc`, `Ctrl-C` or `q` to stop running.\n ",
    )
    .block(
        Block::default()
            .title("Newsroom")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    )
    .style(Style::default().fg(Color::Cyan).bg(Color::Black))
    .alignment(Alignment::Center);

    let loading_widget = Paragraph::new(
        "Loading news.\n\
                Press `Esc`, `Ctrl-C` or `q` to stop running.\n ",
    )
    .block(
        Block::default()
            .title("Newsroom")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    )
    .style(Style::default().fg(Color::Cyan).bg(Color::Black))
    .alignment(Alignment::Center);

    match &app.newsroom_state {
        newsroomcore::newsroomstate::NewsroomState::homescreen =>
        // Render
        {
            frame.render_widget(start_widget, frame.size())
        }
        newsroomcore::newsroomstate::NewsroomState::fetch_media(_) =>
        // Render
        {
            frame.render_widget(loading_widget, frame.size())
        }
        newsroomcore::newsroomstate::NewsroomState::display_media(articles) => {
            let items: Vec<ListItem> = articles
                .iter()
                .map(|x| ListItem::new(format!("{}: {}", x.source.name,x.title)))
                .collect();
            let active_widget = List::new(items)
                .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
                .block(
                    Block::default()
                        .title("Newsroom")
                        .title_alignment(Alignment::Center)
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                )
                .style(Style::default().fg(Color::Cyan).bg(Color::Black));
            // Render
            let mut state = ListState::default();
            state.select(app.state.selected());
            frame.render_stateful_widget(active_widget, frame.size(), &mut state);
            // frame.render_widget(active_widget, frame.size());
        }
        newsroomcore::newsroomstate::NewsroomState::manage_settings => todo!(),
    };
}
