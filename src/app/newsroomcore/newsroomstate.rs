use super::newsarticle::news_article;

// Enum to represent our own app state
pub enum NewsroomState{
    Startup(String),
    homescreen,
    fetch_media(Vec<DataSources>),
    return_media(Vec<news_article>),
    display_media,
    manage_settings,
}

// Enum to represent our own state transitions
pub enum NewsroomTransitions{
    Loaded,
    ToSettings,
    ExitSettings,
    FetchMedia(Vec<DataSources>),
    ReturnMedia(Vec<news_article>),
}

// This enum represents our data providers
// Later on we can store data for each API
// Inside the enum fields 
#[derive(Debug, Clone)]
pub struct DataSources {
    pub name: String,
    pub url: String,
}

impl DataSources {
    
}