use crate::sites::{Site, WebArticle};
use chrono::DateTime;
use feed_parser::parsers;
pub struct StockmarkNews {}

#[cfg(test)]
mod tests;

impl Site for StockmarkNews {
    fn name(&self) -> String {
        return "Stockmark News".to_string();
    }
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
    async fn get_article_text(&self, url: &String) -> String {
        let body = reqwest::get(url).await.unwrap().text().await.unwrap();
        let document = scraper::Html::parse_document(&body);
        let selector = scraper::Selector::parse("main div.l-body").unwrap();
        let article = document.select(&selector).next().unwrap();
        let text = article.text().collect::<Vec<_>>().join("\n");
        return self.trim_text(&text);
    }
}
