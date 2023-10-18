use futures::TryFutureExt;
use tokio::sync::mpsc::{Sender, Receiver, self};
use tui::widgets::TableState;

pub mod newsroomcore;

use newsroomcore::{newsroomstate::NewsroomState, newsroomstate::NewsroomTransitions, utils};

use crate::app::newsroomcore::datasources::DataSources;

use self::newsroomcore::{newsfetchrss::{self, fetch_articles}, newsarticle::news_article, utils::loadScreen};

#[derive(Debug, Clone)]
pub struct App {
    pub newsroom_state: NewsroomState,
    pub state: TableState,
    pub newsroom_articles: Vec<news_article>,
}

impl App {
    pub fn new() -> App {
        App {
            state: TableState::default(),
            newsroom_state: NewsroomState::Startup(loadScreen.to_string()),
            newsroom_articles: vec![],
        }
    }

    pub async fn load(&mut self){
        let cbc = DataSources{name: "cbc".to_string(), url: "https://www.cbc.ca/cmlink/rss-topstories".to_string()};
        let cnn = DataSources{name: "cnn".to_string(), url: "http://rss.cnn.com/rss/cnn_topstories.rss".to_string()};
        let globe: DataSources = DataSources { name: "globe and mail".to_string(), url: "https://www.theglobeandmail.com/arc/outboundfeeds/rss/category/canada/".to_string()};
    
        let sources = vec![cbc, cnn, globe];

        // Fetch articles and add them to the app
        let fetched_articles = fetch_articles(sources).await;
        NewsroomTransitions::ReturnMedia(fetched_articles);
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

    pub fn collect(&mut self, transition: NewsroomTransitions)
    {
        match(&self.newsroom_state, transition){
            (NewsroomState::Startup(_), NewsroomTransitions::Loaded) => todo!(),
            (NewsroomState::Startup(_), NewsroomTransitions::ToSettings) => todo!(),
            (NewsroomState::Startup(_), NewsroomTransitions::ExitSettings) => todo!(),
            (NewsroomState::Startup(_), NewsroomTransitions::FetchMedia(_)) => todo!(),
            (NewsroomState::Startup(_), NewsroomTransitions::ReturnMedia(media_vec)) => {self.newsroom_articles = media_vec; self.newsroom_state = NewsroomState::display_media},
            (NewsroomState::homescreen, NewsroomTransitions::Loaded) => todo!(),
            (NewsroomState::homescreen, NewsroomTransitions::ToSettings) => todo!(),
            (NewsroomState::homescreen, NewsroomTransitions::ExitSettings) => todo!(),
            (NewsroomState::homescreen, NewsroomTransitions::FetchMedia(_)) => todo!(),
            (NewsroomState::homescreen, NewsroomTransitions::ReturnMedia(_)) => todo!(),
            (NewsroomState::fetch_media(_), NewsroomTransitions::Loaded) => todo!(),
            (NewsroomState::fetch_media(_), NewsroomTransitions::ToSettings) => todo!(),
            (NewsroomState::fetch_media(_), NewsroomTransitions::ExitSettings) => todo!(),
            (NewsroomState::fetch_media(_), NewsroomTransitions::FetchMedia(_)) => todo!(),
            (NewsroomState::fetch_media(_), NewsroomTransitions::ReturnMedia(_)) => todo!(),
            (NewsroomState::return_media(_), NewsroomTransitions::Loaded) => todo!(),
            (NewsroomState::return_media(_), NewsroomTransitions::ToSettings) => todo!(),
            (NewsroomState::return_media(_), NewsroomTransitions::ExitSettings) => todo!(),
            (NewsroomState::return_media(_), NewsroomTransitions::FetchMedia(_)) => todo!(),
            (NewsroomState::return_media(_), NewsroomTransitions::ReturnMedia(_)) => todo!(),
            (NewsroomState::display_media, NewsroomTransitions::Loaded) => todo!(),
            (NewsroomState::display_media, NewsroomTransitions::ToSettings) => todo!(),
            (NewsroomState::display_media, NewsroomTransitions::ExitSettings) => todo!(),
            (NewsroomState::display_media, NewsroomTransitions::FetchMedia(_)) => todo!(),
            (NewsroomState::display_media, NewsroomTransitions::ReturnMedia(_)) => todo!(),
            (NewsroomState::manage_settings, NewsroomTransitions::Loaded) => todo!(),
            (NewsroomState::manage_settings, NewsroomTransitions::ToSettings) => todo!(),
            (NewsroomState::manage_settings, NewsroomTransitions::ExitSettings) => todo!(),
            (NewsroomState::manage_settings, NewsroomTransitions::FetchMedia(_)) => todo!(),
            (NewsroomState::manage_settings, NewsroomTransitions::ReturnMedia(_)) => todo!(),
        }
    }
}