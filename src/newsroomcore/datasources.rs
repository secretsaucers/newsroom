use tokio::sync::mpsc::Sender;
use serde::{Deserialize, Serialize};
use super::{newsfetchrss::get_channel, newsarticle::NewsArticle};

// Represents our data providers
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DataSources {
    pub name: String,
    pub url: String,
}

impl DataSources {
    /// Take in a channel and reformat into a vector of news articles, streaming them async over a channel
    ///
    /// Arguments
    /// * tx - A channel which we can send fetched articles over
    pub(crate) async fn stream_articles(self, tx: Sender<NewsArticle>) -> Result<(), ()>{
        if let Ok(channel) = get_channel(&self.url).await {
            let _articles: Vec<NewsArticle> = Vec::new();
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
        
                        let article_to_push: NewsArticle = NewsArticle{ authors: author, title, summary, link, source: self.clone() };
                        let _ = tx.send(article_to_push).await;
                    },
                    None => {},
                }
            }
        } else {return Err(())}
    
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use tokio::sync::mpsc::{Receiver, self};

    use super::*;

    #[tokio::test]
    async fn streaming_test(){
        let cbc = DataSources{name: "cbc".to_string(), url: "https://www.cbc.ca/cmlink/rss-topstories".to_string()};
        let (tx, mut rx): (Sender<NewsArticle>, Receiver<NewsArticle>) = mpsc::channel(100);

        tokio::spawn(cbc.stream_articles(tx));
        let article = rx.recv().await;
        println!("{:#?}", article.unwrap());
        assert!(rx.recv().await.is_some());
    }
}