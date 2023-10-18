use tui::widgets::TableState;

pub mod newsroomcore;

use newsroomcore::{newsroomstate::newsroom_state, newsroomstate::newsroom_transitions, utils};

use crate::app::newsroomcore::newsroomstate::data_sources;

use self::newsroomcore::{newsfetchrss::fetch_rss_feed, newsarticle::news_article, utils::loadScreen};

pub struct App<'a> {
    pub newsroom_state: newsroom_state,
    pub state: TableState,
    pub items: Vec<Vec<&'a str>>,
    pub newsroom_articles: Vec<news_article>
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            state: TableState::default(),
            items: vec![
            ],
            newsroom_state: newsroom_state::startup(loadScreen.to_string()),
            newsroom_articles: vec![],
        }
    }

    pub async fn load(&mut self){
        let cbc = data_sources{name: "cbc".to_string(), url: "https://www.cbc.ca/cmlink/rss-topstories".to_string()};
        let cnn = data_sources{name: "cnn".to_string(), url: "http://rss.cnn.com/rss/cnn_topstories.rss".to_string()};
        let globe: data_sources = data_sources { name: "globe and mail".to_string(), url: "https://www.theglobeandmail.com/arc/outboundfeeds/rss/category/canada/".to_string()};
        
        let mut f_cbc = fetch_rss_feed(cbc).await.unwrap();
        let mut f_cnn = fetch_rss_feed(cnn).await.unwrap();
        let mut f_globe = fetch_rss_feed(globe).await.unwrap();

        let mut fetched_articles: Vec<news_article> = f_cbc;
        fetched_articles.append(&mut f_cnn);
        fetched_articles.append(&mut f_globe);

        self.collect(newsroom_transitions::ReturnMedia(fetched_articles));
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.newsroom_articles.len() - 1 {
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
                    self.newsroom_articles.len() - 1
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
            (newsroom_state::startup(_), newsroom_transitions::Loaded) => todo!(),
            (newsroom_state::startup(_), newsroom_transitions::ToSettings) => todo!(),
            (newsroom_state::startup(_), newsroom_transitions::ExitSettings) => todo!(),
            (newsroom_state::startup(_), newsroom_transitions::FetchMedia(_)) => todo!(),
            (newsroom_state::startup(_), newsroom_transitions::ReturnMedia(media_vec)) => {self.newsroom_articles = media_vec; self.newsroom_state = newsroom_state::display_media},
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