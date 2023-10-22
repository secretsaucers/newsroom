



use super::{newsarticle::news_article, datasources::DataSources};

// Enum to represent our own app state
#[derive(Debug, Clone)]
pub enum NewsroomState{
    homescreen,
    fetch_media(Vec<DataSources>),
    display_media(Vec<news_article>),
    manage_settings,
}

// Enum to represent our own state transitions
pub enum NewsroomTransitions{
    Loaded,
    ToSettings,
    ExitSettings,
    FetchMedia(Vec<DataSources>),
    ReturnMedia(Vec<news_article>),
    Up,
    Down,
    Left,
    Right,
    Quit,
}