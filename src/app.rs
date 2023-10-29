use std::error;
/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;
use crate::newsroomcore::newsroomstate::NewsroomState;
use crate::{newsroomcore::{
    datasources::DataSources, newsfetchrss::fetch_articles, newsroomstate::NewsroomTransitions,
}, settings::Settings};

use log::info;
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver, unbounded_channel};
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
            newsroom_state: NewsroomState::Homescreen,
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
        let _ = tx.send(NewsroomTransitions::FetchMedia(sources.clone()));


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
        let _ = tx.send(NewsroomTransitions::ReturnMedia(fetched_articles));

    }

    /// Advance the current widget (only used now to highlight the next article)
    fn next(&mut self) {
        match &self.newsroom_state {
            NewsroomState::DisplayMedia(articles) => {
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
            NewsroomState::DisplayMedia(articles) => {
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
            NewsroomState::DisplayMedia(articles) => {
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
            NewsroomState::ManageSettings(_) => self.tx.send(NewsroomTransitions::ExitSettings),
            _ => self.tx.send(NewsroomTransitions::ToSettings),
        };
    }

    /// Collects state transitions and acts on them based on the current state
    /// # Arguments 
    /// 
    /// * `transition` - The state transition to be acted upon
    fn collect(&mut self, transition: NewsroomTransitions) {
        match (&self.newsroom_state, transition) {
            (NewsroomState::Homescreen, NewsroomTransitions::Loaded) => todo!(),
            (NewsroomState::Homescreen, NewsroomTransitions::ExitSettings) => todo!(),
            (NewsroomState::Homescreen, NewsroomTransitions::FetchMedia(sources)) => {
                self.newsroom_state = NewsroomState::FetchMedia(sources.clone());
                let local_tx = self.tx.clone();
                tokio::spawn(App::load(local_tx, sources));
            },
            (NewsroomState::Homescreen, NewsroomTransitions::ReturnMedia(_)) => todo!(),
            (NewsroomState::FetchMedia(_), NewsroomTransitions::Loaded) => todo!(),
            (NewsroomState::FetchMedia(_), NewsroomTransitions::ExitSettings) => todo!(),
            (NewsroomState::FetchMedia(_), NewsroomTransitions::FetchMedia(_)) => {},
            (NewsroomState::FetchMedia(_), NewsroomTransitions::ReturnMedia(media_vec)) =>  self.newsroom_state = NewsroomState::DisplayMedia(media_vec),
            (NewsroomState::DisplayMedia(_), NewsroomTransitions::Loaded) => todo!(),
            (NewsroomState::DisplayMedia(_), NewsroomTransitions::ExitSettings) => todo!(),
            (NewsroomState::DisplayMedia(_), NewsroomTransitions::FetchMedia(_)) => todo!(),
            (NewsroomState::DisplayMedia(_), NewsroomTransitions::ReturnMedia(_)) => {}
            (NewsroomState::ManageSettings(_), NewsroomTransitions::Loaded) => todo!(),
            (NewsroomState::ManageSettings(maybe_articles), NewsroomTransitions::ExitSettings) => {
                self.tab = 0; 
                // If we saved the articles when transitioning to settings, change to the display state on settings exit
                // else go to homescreen
                self.newsroom_state = match maybe_articles {
                    Some(articles) => NewsroomState::DisplayMedia(articles.to_vec()),
                    None => NewsroomState::Homescreen,
                }
            
            },
            (NewsroomState::DisplayMedia(articles), NewsroomTransitions::ToSettings) => {self.tab = 1; self.newsroom_state = NewsroomState::ManageSettings(Some(articles.to_vec()))},
            (_, NewsroomTransitions::ToSettings) => {self.tab = 1; self.newsroom_state = NewsroomState::ManageSettings(None)},
            (NewsroomState::ManageSettings(_), NewsroomTransitions::FetchMedia(_)) => todo!(),
            (NewsroomState::ManageSettings(_), NewsroomTransitions::ReturnMedia(_)) => todo!(),
            (NewsroomState::Homescreen, NewsroomTransitions::Up) => todo!(),
            (NewsroomState::Homescreen, NewsroomTransitions::Down) => todo!(),
            (NewsroomState::Homescreen, NewsroomTransitions::Left) => todo!(),
            (NewsroomState::Homescreen, NewsroomTransitions::Right) => todo!(),
            (NewsroomState::FetchMedia(_), NewsroomTransitions::Up) => todo!(),
            (NewsroomState::FetchMedia(_), NewsroomTransitions::Down) => todo!(),
            (NewsroomState::FetchMedia(_), NewsroomTransitions::Left) => todo!(),
            (NewsroomState::FetchMedia(_), NewsroomTransitions::Right) => todo!(),
            (NewsroomState::DisplayMedia(_), NewsroomTransitions::Up) => self.previous(),
            (NewsroomState::DisplayMedia(_), NewsroomTransitions::Down) => self.next(),
            (NewsroomState::DisplayMedia(_), NewsroomTransitions::Left) => todo!(),
            (NewsroomState::DisplayMedia(_), NewsroomTransitions::Right) => todo!(),
            (NewsroomState::ManageSettings(_), NewsroomTransitions::Up) => {},
            (NewsroomState::ManageSettings(_), NewsroomTransitions::Down) => {},
            (NewsroomState::ManageSettings(_), NewsroomTransitions::Left) => {},
            (NewsroomState::ManageSettings(_), NewsroomTransitions::Right) => {},
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
    use super::*;
    /// Test that we're able to run the load fn correctly
    #[tokio::test]
    async fn test_load() {
        let mut app: App = App::new();
        assert!(matches!(app.newsroom_state, NewsroomState::Homescreen));
        App::load(app.tx.clone() ,vec![app.settings.sources[0].clone()]).await;
        
        // Check that we first get a state change to the fetch state
        app.poll_and_run_action().await;
        assert!(matches!(
            app.newsroom_state,
            NewsroomState::FetchMedia(_)
        ));
        // Check that we then get a state change to the display state
        app.poll_and_run_action().await;
        assert!(matches!(
            app.newsroom_state,
            NewsroomState::DisplayMedia(_)
        ));
    }

    #[tokio::test]
    async fn test_up_down(){
        let mut app: App = App::new();
        assert!(matches!(app.newsroom_state, NewsroomState::Homescreen));
        App::load(app.tx.clone() ,vec![app.settings.sources[0].clone()]).await;

        app.poll_and_run_action().await; // Wait fetch state
        app.poll_and_run_action().await; // Wait display state

        // We now should be in a display state
        // Check that the up/down keys work now to advance the rows

        assert_eq!(app.row, None); // Check row initially not selected

        app.next(); // Key down
        assert_eq!(app.row, Some(0));

        app.next(); // Key down
        assert_eq!(app.row, Some(1));

        app.previous(); // Key down
        assert_eq!(app.row, Some(0));
    }

    #[tokio::test]
    async fn test_tab(){
        let mut app: App = App::new();
        assert!(matches!(app.newsroom_state, NewsroomState::Homescreen));
        App::load(app.tx.clone() ,vec![app.settings.sources[0].clone()]).await;

        app.poll_and_run_action().await; // Wait fetch state
        app.poll_and_run_action().await; // Wait display state

        // We now should be in a display state
        // Check that we can tab out to the settings screen and then back to the main screen

        app.change_tab();
        app.poll_and_run_action().await;
        assert!(matches!(app.newsroom_state, NewsroomState::ManageSettings(_)));
        app.change_tab();
        app.poll_and_run_action().await;
        assert!(matches!(app.newsroom_state, NewsroomState::DisplayMedia(_)));
    }
}
