pub mod newsroomcore; // Main newsroom code

use std::{error, sync::Arc};
/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;
use self::newsroomcore::{newsarticle::news_article, newsroomstate::NewsroomState};
use crate::app::newsroomcore::{
    datasources::DataSources, newsfetchrss::fetch_articles, newsroomstate::NewsroomTransitions,
};
use log::{info, trace, warn};
use tokio::sync::{mpsc::{self, Receiver, Sender}, Mutex};
use tui::widgets::TableState;
use rand::thread_rng;
use rand::seq::SliceRandom;
use webbrowser;

// Application
#[derive(Debug, Clone)]
pub struct App {
    pub newsroom_state: NewsroomState,
    pub state: TableState,
    /// Is the application running?
    pub running: bool,
}

impl App {
    pub fn new() -> App {
        App {
            state: TableState::default(),
            newsroom_state: NewsroomState::homescreen,
            running: true,
        }
    }

    pub async fn load(app_arc: Arc<Mutex<App>>) {
        { // Set into fetch state
            info!("Initiating article load");
            let cbc = DataSources {
                name: "cbc".to_string(),
                url: "https://www.cbc.ca/cmlink/rss-topstories".to_string(),
            };
            let cnn = DataSources {
                name: "cnn".to_string(),
                url: "http://rss.cnn.com/rss/cnn_topstories.rss".to_string(),
            };
            let globe: DataSources = DataSources {
                name: "globe and mail".to_string(),
                url: "https://www.theglobeandmail.com/arc/outboundfeeds/rss/category/canada/"
                    .to_string(),
            };
            let sources = vec![cbc, cnn, globe];

            let app_arc_local = app_arc.clone();
            app_arc_local.lock().await.collect(NewsroomTransitions::FetchMedia(sources));
        }

        { // Set into display state
            let app_arc_local = app_arc.clone();
            let mut app_local = app_arc_local.lock().await;
            match &app_local.newsroom_state{
                NewsroomState::fetch_media(sources) => {
                    // Fetch articles and add them to the app
                    let num_sources = sources.len();
                    let mut fetched_articles = fetch_articles(sources.to_vec()).await;
                    info!(
                        "Loaded {} articles from {} sources, finished article load",
                        fetched_articles.len(),
                        num_sources
                    );
                    let mut rng = thread_rng();
                    fetched_articles.shuffle(&mut rng);
                    app_local.collect(NewsroomTransitions::ReturnMedia(fetched_articles));
                },
                _ => {}
            }
        }
    }

    pub fn next(&mut self) {
        match &self.newsroom_state {
            NewsroomState::display_media(articles) => {
                let i = match self.state.selected() {
                    Some(i) => {
                        if i >= articles.len() - 1 {
                            0
                        } else {
                            i + 1
                        }
                    }
                    None => 0,
                };
                self.state.select(Some(i));
            }
            _ => {}
        }
    }

    pub fn previous(&mut self) {
        match &self.newsroom_state {
            NewsroomState::display_media(articles) => {
                let i = match self.state.selected() {
                    Some(i) => {
                        if i == 0 {
                            articles.len() - 1
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
                };
                self.state.select(Some(i));
            }
            _ => {}
        }
    }

    pub fn open_selected(&self) {
        match &self.newsroom_state{
            NewsroomState::display_media(articles) => {
                match &self.state.selected(){
                    Some(index) => {
                        let url = &articles[*index].link;
                        let _ = webbrowser::open(url);
                    },
                    None => {},
                }
            },
            _ => {},
        }
    }

    pub fn collect(&mut self, transition: NewsroomTransitions) {
        match (&self.newsroom_state, transition) {
            (NewsroomState::homescreen, NewsroomTransitions::Loaded) => todo!(),
            (NewsroomState::homescreen, NewsroomTransitions::ToSettings) => todo!(),
            (NewsroomState::homescreen, NewsroomTransitions::ExitSettings) => todo!(),
            (NewsroomState::homescreen, NewsroomTransitions::FetchMedia(sources)) => self.newsroom_state = NewsroomState::fetch_media(sources),
            (NewsroomState::homescreen, NewsroomTransitions::ReturnMedia(_)) => todo!(),
            (NewsroomState::fetch_media(_), NewsroomTransitions::Loaded) => todo!(),
            (NewsroomState::fetch_media(_), NewsroomTransitions::ToSettings) => todo!(),
            (NewsroomState::fetch_media(_), NewsroomTransitions::ExitSettings) => todo!(),
            (NewsroomState::fetch_media(_), NewsroomTransitions::FetchMedia(_)) => todo!(),
            (NewsroomState::fetch_media(_), NewsroomTransitions::ReturnMedia(media_vec)) =>  self.newsroom_state = NewsroomState::display_media(media_vec),
            (NewsroomState::display_media(_), NewsroomTransitions::Loaded) => todo!(),
            (NewsroomState::display_media(_), NewsroomTransitions::ToSettings) => todo!(),
            (NewsroomState::display_media(_), NewsroomTransitions::ExitSettings) => todo!(),
            (NewsroomState::display_media(_), NewsroomTransitions::FetchMedia(_)) => todo!(),
            (NewsroomState::display_media(_), NewsroomTransitions::ReturnMedia(_)) => {}
            (NewsroomState::manage_settings, NewsroomTransitions::Loaded) => todo!(),
            (NewsroomState::manage_settings, NewsroomTransitions::ToSettings) => todo!(),
            (NewsroomState::manage_settings, NewsroomTransitions::ExitSettings) => todo!(),
            (NewsroomState::manage_settings, NewsroomTransitions::FetchMedia(_)) => todo!(),
            (NewsroomState::manage_settings, NewsroomTransitions::ReturnMedia(_)) => todo!(),
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn tick(&self) {
        // Used whenever the tick is nessisary
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tokio::sync::mpsc::{self, Receiver};

    // // Test that we're able to run the load fn correctly
    // #[tokio::test]
    // async fn test_load() {
    //     let mut app: App = App::new();
    //     assert!(matches!(app.newsroom_state, NewsroomState::homescreen));
    //     App::load().await;
    //     assert!(matches!(
    //         app.newsroom_state,
    //         NewsroomState::display_media(_)
    //     ));
    // }
}
