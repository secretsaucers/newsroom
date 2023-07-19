use url::Url;

use super::newsarticle::news_article;

// Enum to represent our own app state
pub enum newsroom_state{
    startup,
    homescreen,
    fetch_media(Vec<data_sources>),
    return_media(Vec<news_article>),
    display_media,
    manage_settings,
}

// Enum to represent our own state transitions
pub enum newsroom_transitions{
    Loaded,
    ToSettings,
    ExitSettings,
    FetchMedia(Vec<data_sources>),
    ReturnMedia(Vec<news_article>),
}

// This enum represents our data providers
// Later on we can store data for each API
// Inside the enum fields 
#[derive(Debug)]
pub enum data_sources {
    associated_press,
    bbc,
    cbc,
    cnn,
}

impl data_sources {
    
}