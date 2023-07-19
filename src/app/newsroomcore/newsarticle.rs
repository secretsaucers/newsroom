use url::Url;

use super::newsroomstate::data_sources;

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