use tui::widgets::TableState;
use url::Url;

mod newsroomcore;

use newsroomcore::{newsroomstate::newsroom_state, newsroomstate::newsroom_transitions};

pub struct App<'a> {
    pub newsroom_state: newsroom_state,
    pub state: TableState,
    pub items: Vec<Vec<&'a str>>,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            state: TableState::default(),
            items: vec![
                vec!["Row11", "Row12", "Row13"],
                vec!["Row21", "Row22", "Row23"],
                vec!["Row31", "Row32", "Row33"],
                vec!["Row41", "Row42", "Row43"],
                vec!["Row51", "Row52", "Row53"],
                vec!["Row61", "Row62\nTest", "Row63"],
                vec!["Row71", "Row72", "Row73"],
                vec!["Row81", "Row82", "Row83"],
                vec!["Row91", "Row92", "Row93"],
                vec!["Row101", "Row102", "Row103"],
                vec!["Row111", "Row112", "Row113"],
                vec!["Row121", "Row122", "Row123"],
                vec!["Row131", "Row132", "Row133"],
                vec!["Row141", "Row142", "Row143"],
                vec!["Row151", "Row152", "Row153"],
                vec!["Row161", "Row162", "Row163"],
                vec!["Row171", "Row172", "Row173"],
                vec!["Row181", "Row182", "Row183"],
                vec!["Row191", "Row192", "Row193"],
            ],
            newsroom_state: newsroom_state::startup,
        }
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn collect(&mut self, transition: newsroom_transitions)
    {
        match(&self.newsroom_state, transition){
            (newsroom_state::startup, newsroom_transitions::Loaded) => todo!(),
            (newsroom_state::startup, newsroom_transitions::ToSettings) => todo!(),
            (newsroom_state::startup, newsroom_transitions::ExitSettings) => todo!(),
            (newsroom_state::startup, newsroom_transitions::FetchMedia(_)) => todo!(),
            (newsroom_state::startup, newsroom_transitions::ReturnMedia(_)) => todo!(),
            (newsroom_state::homescreen, newsroom_transitions::Loaded) => todo!(),
            (newsroom_state::homescreen, newsroom_transitions::ToSettings) => todo!(),
            (newsroom_state::homescreen, newsroom_transitions::ExitSettings) => todo!(),
            (newsroom_state::homescreen, newsroom_transitions::FetchMedia(_)) => todo!(),
            (newsroom_state::homescreen, newsroom_transitions::ReturnMedia(_)) => todo!(),
            (newsroom_state::fetch_media(_), newsroom_transitions::Loaded) => todo!(),
            (newsroom_state::fetch_media(_), newsroom_transitions::ToSettings) => todo!(),
            (newsroom_state::fetch_media(_), newsroom_transitions::ExitSettings) => todo!(),
            (newsroom_state::fetch_media(_), newsroom_transitions::FetchMedia(_)) => todo!(),
            (newsroom_state::fetch_media(_), newsroom_transitions::ReturnMedia(_)) => todo!(),
            (newsroom_state::return_media(_), newsroom_transitions::Loaded) => todo!(),
            (newsroom_state::return_media(_), newsroom_transitions::ToSettings) => todo!(),
            (newsroom_state::return_media(_), newsroom_transitions::ExitSettings) => todo!(),
            (newsroom_state::return_media(_), newsroom_transitions::FetchMedia(_)) => todo!(),
            (newsroom_state::return_media(_), newsroom_transitions::ReturnMedia(_)) => todo!(),
            (newsroom_state::display_media, newsroom_transitions::Loaded) => todo!(),
            (newsroom_state::display_media, newsroom_transitions::ToSettings) => todo!(),
            (newsroom_state::display_media, newsroom_transitions::ExitSettings) => todo!(),
            (newsroom_state::display_media, newsroom_transitions::FetchMedia(_)) => todo!(),
            (newsroom_state::display_media, newsroom_transitions::ReturnMedia(_)) => todo!(),
            (newsroom_state::manage_settings, newsroom_transitions::Loaded) => todo!(),
            (newsroom_state::manage_settings, newsroom_transitions::ToSettings) => todo!(),
            (newsroom_state::manage_settings, newsroom_transitions::ExitSettings) => todo!(),
            (newsroom_state::manage_settings, newsroom_transitions::FetchMedia(_)) => todo!(),
            (newsroom_state::manage_settings, newsroom_transitions::ReturnMedia(_)) => todo!(),
        }
    }
}