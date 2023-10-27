use std::{error};
/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;
use crate::newsroomcore::{newsroomstate::NewsroomState};
use crate::{newsroomcore::{
    datasources::DataSources, newsfetchrss::fetch_articles, newsroomstate::NewsroomTransitions,
}, settings::Settings};

use log::{info};
use tokio::sync::{mpsc::{UnboundedSender, UnboundedReceiver, unbounded_channel}};
use tui::widgets::TableState;
use rand::thread_rng;
use rand::seq::SliceRandom;
use webbrowser;

// Application
#[derive(Debug)]
pub struct App {
    pub newsroom_state: NewsroomState,
    pub settings: Settings,
    // Is the application running?
    pub running: bool,
    pub tx: UnboundedSender<NewsroomTransitions>,
    rx: UnboundedReceiver<NewsroomTransitions>,
    pub tab: u16,
    pub row: Option<usize>,
}

impl App {
    pub fn new() -> App {
        let (tx, rx) = unbounded_channel();
        App {
            settings: Settings::new(),
            newsroom_state: NewsroomState::homescreen,
            running: true,
            tx,
            rx,
            tab: 0,
            row: None,
        }
    }

    /// Loads rss articles in the background
    /// 
    /// Arguments
    /// * tx - A sender used to relay the 'articles loaded' transition to the app
    /// * sources - The sources which we are fetching rss for 
    async fn load(tx: UnboundedSender<NewsroomTransitions>, sources: Vec<DataSources>) {
        info!("Initiating article load");

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

    /// Advance the current widget (only used now to highlight the next article)
    fn next(&mut self) {
        match &self.newsroom_state {
            NewsroomState::display_media(articles) => {
                let i = match self.row {
                    Some(i) => {
                        if i >= articles.len() - 1 {
                            0
                        } else {
                            i + 1
                        }
                    }
                    None => 0,
                };
                self.row = Some(i);
            }
            _ => {}
        }
    }

    /// Reverse the current widget (only used now to highlight the previous article)
    fn previous(&mut self) {
        match &self.newsroom_state {
            NewsroomState::display_media(articles) => {
                let i = match self.row {
                    Some(i) => {
                        if i == 0 {
                            articles.len() - 1
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
                };
                self.row = Some(i);
            }
            _ => {}
        }
    }

    /// Opens the currently highlighted news article in the system browser
    pub fn open_selected(&self) {
        match &self.newsroom_state{
            NewsroomState::display_media(articles) => {
                match &self.row {
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

    /// Changes the context tab
    pub fn change_tab(&self) {
        let _ = match &self.newsroom_state {
            NewsroomState::manage_settings(_) => self.tx.send(NewsroomTransitions::ExitSettings),
            _ => self.tx.send(NewsroomTransitions::ToSettings),
        };
    }

    /// Collects state transitions and acts on them based on the current state
    /// # Arguments 
    /// 
    /// * `transition` - The state transition to be acted upon
    fn collect(&mut self, transition: NewsroomTransitions) {
        match (&self.newsroom_state, transition) {
            (NewsroomState::homescreen, NewsroomTransitions::Loaded) => todo!(),
            (NewsroomState::homescreen, NewsroomTransitions::ExitSettings) => todo!(),
            (NewsroomState::homescreen, NewsroomTransitions::FetchMedia(sources)) => {
                self.newsroom_state = NewsroomState::fetch_media(sources.clone());
                let local_tx = self.tx.clone();
                tokio::spawn(App::load(local_tx, sources));
            },
            (NewsroomState::homescreen, NewsroomTransitions::ReturnMedia(_)) => todo!(),
            (NewsroomState::fetch_media(_), NewsroomTransitions::Loaded) => todo!(),
            (NewsroomState::fetch_media(_), NewsroomTransitions::ExitSettings) => todo!(),
            (NewsroomState::fetch_media(_), NewsroomTransitions::FetchMedia(_)) => {},
            (NewsroomState::fetch_media(_), NewsroomTransitions::ReturnMedia(media_vec)) =>  self.newsroom_state = NewsroomState::display_media(media_vec),
            (NewsroomState::display_media(_), NewsroomTransitions::Loaded) => todo!(),
            (NewsroomState::display_media(_), NewsroomTransitions::ExitSettings) => todo!(),
            (NewsroomState::display_media(_), NewsroomTransitions::FetchMedia(_)) => todo!(),
            (NewsroomState::display_media(_), NewsroomTransitions::ReturnMedia(_)) => {}
            (NewsroomState::manage_settings(_), NewsroomTransitions::Loaded) => todo!(),
            (NewsroomState::manage_settings(maybe_articles), NewsroomTransitions::ExitSettings) => {
                self.tab = 0; 
                // If we saved the articles when transitioning to settings, change to the display state on settings exit
                // else go to homescreen
                self.newsroom_state = match maybe_articles {
                    Some(articles) => NewsroomState::display_media(articles.to_vec()),
                    None => NewsroomState::homescreen,
                }
            
            },
            (NewsroomState::display_media(articles), NewsroomTransitions::ToSettings) => {self.tab = 1; self.newsroom_state = NewsroomState::manage_settings(Some(articles.to_vec()))},
            (_, NewsroomTransitions::ToSettings) => {self.tab = 1; self.newsroom_state = NewsroomState::manage_settings(None)},
            (NewsroomState::manage_settings(_), NewsroomTransitions::FetchMedia(_)) => todo!(),
            (NewsroomState::manage_settings(_), NewsroomTransitions::ReturnMedia(_)) => todo!(),
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
            (NewsroomState::manage_settings(_), NewsroomTransitions::Up) => {},
            (NewsroomState::manage_settings(_), NewsroomTransitions::Down) => {},
            (NewsroomState::manage_settings(_), NewsroomTransitions::Left) => {},
            (NewsroomState::manage_settings(_), NewsroomTransitions::Right) => {},
            (_, NewsroomTransitions::Quit) => self.running = false,
        }
    }

    pub fn tick(&mut self) {
        // Used whenever the tick is nessasary
    }

    /// Collects state transitions from the input channel and runs collect on them. It is intended to be run in the main loop
    pub async fn poll_and_run_action(&mut self){
        let transition_maybe = self.rx.try_recv();
        match transition_maybe{
            Ok(transition) => self.collect(transition),
            Err(_) => {},
        }
    }
}

#[cfg(test)]
mod test {
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
