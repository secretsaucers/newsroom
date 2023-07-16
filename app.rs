use tui::widgets::TableState;
use url::Url;

// Enum to represent our own app state
pub enum NewsroomState{
    Startup,
    Homescreen,
    Fetching,
    DisplayMedia(Vec<NewsArticle>),
    ManageSettings,
}

// Enum to represent our own state transitions
pub enum NewsroomTransitions{
    Loaded,
    ToSettings,
    ExitSettings,
    FetchMedia(Vec<DataSources>),
    ReturnMedia(Vec<NewsArticle>),
}

// This enum represents our data providers
// Later on we can store data for each API
// Inside the enum fields 
#[derive(Debug)]
pub enum DataSources {
    AssociatedPress,
    Bbc,
    Cbc,
    Cnn,
}

impl DataSources {
    
}

// This struct represents the data that we
// Actually care about extracting from the news API
#[derive(Debug)]
pub struct NewsArticle{
    authors: Vec<String>,
    title: String,
    summary: String,
    link: Url,
    source: DataSources,
}

impl NewsArticle {

}

pub struct App<'a> {
    pub newsroomstate: NewsroomState,
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
            newsroomstate: NewsroomState::Startup,
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

    pub fn collect(&mut self, transition: NewsroomTransitions)
    {
        match(&self.newsroomstate, transition){
            (NewsroomState::Startup, NewsroomTransitions::Loaded) => todo!(),
            (NewsroomState::Startup, NewsroomTransitions::ToSettings) => todo!(),
            (NewsroomState::Startup, NewsroomTransitions::ExitSettings) => todo!(),
            (NewsroomState::Startup, NewsroomTransitions::FetchMedia(_)) => todo!(),
            (NewsroomState::Startup, NewsroomTransitions::ReturnMedia(_)) => todo!(),
            (NewsroomState::Homescreen, NewsroomTransitions::Loaded) => todo!(),
            (NewsroomState::Homescreen, NewsroomTransitions::ToSettings) => todo!(),
            (NewsroomState::Homescreen, NewsroomTransitions::ExitSettings) => todo!(),
            (NewsroomState::Homescreen, NewsroomTransitions::FetchMedia(_)) => todo!(),
            (NewsroomState::Homescreen, NewsroomTransitions::ReturnMedia(_)) => todo!(),
            (NewsroomState::Fetching, NewsroomTransitions::Loaded) => todo!(),
            (NewsroomState::Fetching, NewsroomTransitions::ToSettings) => todo!(),
            (NewsroomState::Fetching, NewsroomTransitions::ExitSettings) => todo!(),
            (NewsroomState::Fetching, NewsroomTransitions::FetchMedia(_)) => todo!(),
            (NewsroomState::Fetching, NewsroomTransitions::ReturnMedia(_)) => todo!(),
            (NewsroomState::DisplayMedia(_), NewsroomTransitions::Loaded) => todo!(),
            (NewsroomState::DisplayMedia(_), NewsroomTransitions::ToSettings) => todo!(),
            (NewsroomState::DisplayMedia(_), NewsroomTransitions::ExitSettings) => todo!(),
            (NewsroomState::DisplayMedia(_), NewsroomTransitions::FetchMedia(_)) => todo!(),
            (NewsroomState::DisplayMedia(_), NewsroomTransitions::ReturnMedia(_)) => todo!(),
            (NewsroomState::ManageSettings, NewsroomTransitions::Loaded) => todo!(),
            (NewsroomState::ManageSettings, NewsroomTransitions::ToSettings) => todo!(),
            (NewsroomState::ManageSettings, NewsroomTransitions::ExitSettings) => todo!(),
            (NewsroomState::ManageSettings, NewsroomTransitions::FetchMedia(_)) => todo!(),
            (NewsroomState::ManageSettings, NewsroomTransitions::ReturnMedia(_)) => todo!(),
        }
    }
}