use crate::sites::{Site, WebArticle};
use chrono::DateTime;
use feed_parser::parsers;
use scraper::Selector;
pub struct LineTechBlog {}

#[cfg(test)]
mod tests;

impl Site for LineTechBlog {
    fn name(&self) -> String {
        return "LINE Engineering Blog".to_string();
    }
    async fn get_articles(&self) -> Vec<WebArticle> {
        let body = reqwest::get("https://techblog.lycorp.co.jp/ja/feed/index.xml")
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
        let html = reqwest::get(url).await.unwrap().text().await.unwrap();
        let doc = scraper::Html::parse_document(&html);
        let sel = Selector::parse("main div.content").unwrap();
        let text = doc.select(&sel).next().unwrap().text().collect();
        return self.trim_text(&text);
    }
}
