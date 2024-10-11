use crate::sites::{Site, WebArticle};
use chrono::DateTime;
use feed_parser::parsers;
pub struct StockmarkNews {}

#[cfg(test)]
mod tests;

impl Site for StockmarkNews {
    async fn get_articles(&self) -> Vec<WebArticle> {
        let body = reqwest::get("https://stockmark.co.jp/news/feed/")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        let feeds = parsers::rss2::parse(&body).unwrap();
        let mut articles = Vec::new();
        for feed in feeds {
            println!("{:?}", feed);
            articles.push(WebArticle {
                title: feed.title,
                url: feed.link,
                text: feed.description.unwrap_or("".to_string()),
                timestamp: DateTime::parse_from_rfc2822(&feed.publish_date.unwrap())
                    .unwrap()
                    .into(),
            });
        }
        return articles;
    }
}
