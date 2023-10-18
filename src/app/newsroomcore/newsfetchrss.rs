// Code section to fetch RSS data in a way we can understand
use rss::Channel;
use std::error::Error;
use reqwest::Request;

use super::{newsarticle::news_article, newsroomstate::DataSources};


async fn get_channel(url : &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url)
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

fn channel_to_articles(channel: Channel, data_source: DataSources) -> Result<Vec<news_article>, Box<dyn Error>>{
    // Take in a channel and reformat into a vector of news articles

    let mut articles: Vec<news_article> = Vec::new();
    for item in channel.items(){
        match item.description(){
            Some(description) => {
                // Extract the data we need from item

                // Author
                let author: Vec<String> = match item.author(){
                    Some(auth) => vec![auth.to_string()],
                    None => vec!["".to_string()],
                };

                // Title
                let title: String = match item.title() {
                    Some(tit) => tit.to_string(),
                    None => "".to_string(),
                };
                
                // Summary (We've already unwrapped this)
                let summary = description.to_string();

                // Link
                let link = match item.link() {
                    Some(url) => url.to_string(),
                    None => "".to_string(),
                };

                let article_to_push: news_article = news_article{ authors: author, title: title, summary: summary, link, source: data_source.clone() };
                articles.push(article_to_push);
            },
            None => {},
        }
    }
    Ok(articles)
}

pub async fn fetch_rss_feed(source: DataSources) -> Result<Vec<news_article>, Box<dyn Error>>{
    let channel = get_channel(&source.url).await?;
    channel_to_articles(channel, source)
}

#[cfg(test)]
mod test {
    use super::*;

    // Test that we're able to correctly read from the CBC rss channel
    #[tokio::test]
    async fn test_rss_fetch(){
        let ch = get_channel("https://www.cbc.ca/cmlink/rss-topstories").await.unwrap();
        let items = ch.items();
        println!("{}", items.len());
        let entry = &items[0];
        // let entry_str = entry.content().unwrap();
        // println!("{}", entry_str);
        assert_eq!(ch.title(), "CBC | Top Stories News ");
    }

    #[tokio::test]
    async fn test_channel_to_articles(){
        let source: DataSources = DataSources { name: "CBC".to_string(), url: "https://www.cbc.ca/cmlink/rss-topstories".to_string() };
        let ch = get_channel(&source.url).await.unwrap();
        let articles = channel_to_articles(ch, source).unwrap();
        assert!(articles.len() > 0); //Make sure that we can pull some articles
        // println!("{:#?}", articles);
    }

    #[tokio::test]
    async fn test_public_fn(){
        let source: DataSources = DataSources { name: "CBC".to_string(), url: "https://www.cbc.ca/cmlink/rss-topstories".to_string() };
        let articles = fetch_rss_feed(source).await.unwrap();
        assert!(articles.len() > 0); //Make sure that we can pull some articles
        println!("{:#?}", articles);
    }

}