use Url::Url;

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

}

// This enum represents our data providers
// Later on we can store data for each API
// Inside the enum fields 
pub enum data_sources {
    associated_press,
    bbc,
    cbc,
    cnn,
}

impl data_sources {
    
}

// This struct represents the data that we
// Actually care about extracting from the news API
#[derive(Debug)]
pub struct news_article{
    authors: Vec<String>,
    title: String,
    summary: String,
    link: Url,
    source: data_sources,
}

impl news_article {

}