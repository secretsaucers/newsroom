use super::{newsarticle::NewsArticle, datasources::DataSources};

/// Enum to represent our own app state
#[derive(Debug, Clone)]
pub enum NewsroomState{
    Homescreen,
    FetchMedia(Vec<DataSources>),
    DisplayMedia(Vec<NewsArticle>),
    ManageSettings(Option<Vec<NewsArticle>>),
}

/// Enum to represent our own state transitions
pub enum NewsroomTransitions{
    Loaded,
    ToSettings,
    ExitSettings,
    FetchMedia(Vec<DataSources>),
    ReturnMedia(Vec<NewsArticle>),
    Up,
    Down,
    Left,
    Right,
    Quit,
}