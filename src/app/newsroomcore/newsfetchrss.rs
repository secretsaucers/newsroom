// Code section to fetch RSS data in a way we can understand
use rss::Channel;
use url::Url;
use std::error::Error;
use reqwest::Request;

use super::newsarticle::news_article;

// struct Contents {
//     contents: str,
// }

// fn fetch(url: Url) -> Vec<Contents> {
    
// }

async fn get_channel(url : &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url)
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

pub fn fetch_rss_feed(url: Url) -> Vec<news_article>{
    todo!()
}