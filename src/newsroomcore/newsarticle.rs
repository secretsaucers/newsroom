use crate::newsroomcore::datasources::DataSources;

/// This struct represents the data that we care about extracting from the rss feed
#[derive(Debug, Clone)]
pub struct news_article{
    pub authors: Vec<String>,
    pub title: String,
    pub summary: String,
    pub link: String,
    pub source: DataSources,
}