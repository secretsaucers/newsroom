use crate::app::newsroomcore::datasources::DataSources;

// This struct represents the data that we
// Actually care about extracting from the news API
#[derive(Debug, Clone)]
pub struct news_article{
    pub authors: Vec<String>,
    pub title: String,
    pub summary: String,
    pub link: String,
    pub source: DataSources,
}

impl news_article {

}