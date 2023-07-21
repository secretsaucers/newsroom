use super::newsroomstate::data_sources;

// This struct represents the data that we
// Actually care about extracting from the news API
#[derive(Debug, Clone)]
pub struct news_article{
    pub authors: Vec<String>,
    pub title: String,
    pub summary: String,
    pub link: String,
    pub source: data_sources,
}

impl news_article {

}