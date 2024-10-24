use crate::sites::{Category, Html, Site, Text, WebArticle};
use chrono::DateTime;
use feed_parser::parsers;
pub struct AIItNow {}

#[cfg(test)]
mod tests;

#[async_trait::async_trait]
impl Site for AIItNow {
    fn name(&self) -> String {
        return "AI IT Now".to_string();
    }
    fn category(&self) -> super::Category {
        return Category::News;
    }
    async fn get_articles(&self) -> Result<Vec<WebArticle>, String> {
        let body = self.request(&"https://ainow.ai/feed/".to_string()).await;
        let feeds = if let Ok(r) = parsers::rss2::parse(&body) {
            r
        } else {
            return Err("Failed to parse RSS".to_string());
        };
        let mut articles = Vec::new();
        for feed in feeds {
            articles.push(WebArticle {
                site: self.name(),
                title: feed.title,
                url: feed.link,
                description: feed.description.unwrap_or("".to_string()),
                timestamp: DateTime::parse_from_rfc2822(&feed.publish_date.unwrap())
                    .unwrap()
                    .into(),
            });
        }
        return Ok(articles);
    }
    async fn get_article_text(&self, url: &String) -> Result<(Html, Text), String> {
        let body = self.request(url).await;
        let document = scraper::Html::parse_document(&body);
        let selector =
            scraper::Selector::parse("body div.contents div.article_area div.entry-content")
                .unwrap();
        if let Some(article) = document.select(&selector).next() {
            let text = article.text().collect::<Vec<_>>().join("\n");
            let html = article.html();
            return Ok((self.trim_text(&html), self.trim_text(&text)));
        } else {
            return Err("NO CONTENT".to_string());
        }
    }
}
