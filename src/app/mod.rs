pub mod newsroomcore; // Main newsroom code

use std::{error, sync::Arc};
/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;
use self::newsroomcore::{newsarticle::news_article, newsroomstate::NewsroomState};
use crate::app::newsroomcore::{
    datasources::DataSources, newsfetchrss::fetch_articles, newsroomstate::NewsroomTransitions,
};

use log::{info, trace, warn};
use tokio::sync::{mpsc::{self, Receiver, UnboundedSender, UnboundedReceiver, unbounded_channel}, Mutex};
use tui::widgets::TableState;
use rand::thread_rng;
use rand::seq::SliceRandom;
use webbrowser;

// Application
#[derive(Debug)]
pub struct App {
    pub newsroom_state: NewsroomState,
    pub state: TableState,
    /// Is the application running?
    pub running: bool,
    pub tx: UnboundedSender<NewsroomTransitions>,
    rx: UnboundedReceiver<NewsroomTransitions>,
}

impl App {
    pub fn new() -> App {
        let (tx, rx) = unbounded_channel();
        App {
            state: TableState::default(),
            newsroom_state: NewsroomState::homescreen,
            running: true,
            tx,
            rx
        }
    }

    async fn load(tx: UnboundedSender<NewsroomTransitions>) {
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

        // Send fetch media update over the channel
        tx.send(NewsroomTransitions::FetchMedia(sources.clone()));


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
        tx.send(NewsroomTransitions::ReturnMedia(fetched_articles));

    }

    fn next(&mut self) {
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

    fn previous(&mut self) {
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

    fn collect(&mut self, transition: NewsroomTransitions) {
        match (&self.newsroom_state, transition) {
            (NewsroomState::homescreen, NewsroomTransitions::Loaded) => todo!(),
            (NewsroomState::homescreen, NewsroomTransitions::ToSettings) => todo!(),
            (NewsroomState::homescreen, NewsroomTransitions::ExitSettings) => todo!(),
            (NewsroomState::homescreen, NewsroomTransitions::FetchMedia(sources)) => {
                self.newsroom_state = NewsroomState::fetch_media(sources);
                let local_tx = self.tx.clone();
                tokio::spawn(App::load(local_tx));
            },
            (NewsroomState::homescreen, NewsroomTransitions::ReturnMedia(_)) => todo!(),
            (NewsroomState::fetch_media(_), NewsroomTransitions::Loaded) => todo!(),
            (NewsroomState::fetch_media(_), NewsroomTransitions::ToSettings) => todo!(),
            (NewsroomState::fetch_media(_), NewsroomTransitions::ExitSettings) => todo!(),
            (NewsroomState::fetch_media(_), NewsroomTransitions::FetchMedia(_)) => {},
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
            (NewsroomState::homescreen, NewsroomTransitions::Up) => todo!(),
            (NewsroomState::homescreen, NewsroomTransitions::Down) => todo!(),
            (NewsroomState::homescreen, NewsroomTransitions::Left) => todo!(),
            (NewsroomState::homescreen, NewsroomTransitions::Right) => todo!(),
            (NewsroomState::fetch_media(_), NewsroomTransitions::Up) => todo!(),
            (NewsroomState::fetch_media(_), NewsroomTransitions::Down) => todo!(),
            (NewsroomState::fetch_media(_), NewsroomTransitions::Left) => todo!(),
            (NewsroomState::fetch_media(_), NewsroomTransitions::Right) => todo!(),
            (NewsroomState::display_media(_), NewsroomTransitions::Up) => self.previous(),
            (NewsroomState::display_media(_), NewsroomTransitions::Down) => self.next(),
            (NewsroomState::display_media(_), NewsroomTransitions::Left) => todo!(),
            (NewsroomState::display_media(_), NewsroomTransitions::Right) => todo!(),
            (NewsroomState::manage_settings, NewsroomTransitions::Up) => todo!(),
            (NewsroomState::manage_settings, NewsroomTransitions::Down) => todo!(),
            (NewsroomState::manage_settings, NewsroomTransitions::Left) => todo!(),
            (NewsroomState::manage_settings, NewsroomTransitions::Right) => todo!(),
            (_, NewsroomTransitions::Quit) => self.running = false,
        }
    }

    pub fn tick(&self) {
        // Used whenever the tick is nessisary
    }

    pub async fn poll_and_run_action(&mut self){
        // This function collects state transitions from the input channel and runs collect on them. It is intended to be run in the main loop
        let transition_maybe = self.rx.try_recv();
        match transition_maybe{
            Ok(transition) => self.collect(transition),
            Err(_) => {},
        }
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
