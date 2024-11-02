use crate::sites::{Category, Html, Site, Text, WebArticle};
use chrono::DateTime;
use feed_parser::parsers;
pub struct MITAI {}

#[cfg(test)]
mod tests;

#[async_trait::async_trait]
impl Site for MITAI {
    fn name(&self) -> String {
        return "MIT Research".to_string();
    }
    fn category(&self) -> super::Category {
        return Category::News;
    }
    async fn get_articles(&self) -> Result<Vec<WebArticle>, String> {
        let body = self
            .request(&"https://news.mit.edu/topic/mitartificial-intelligence2-rss.xml".to_string())
            .await;
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
            scraper::Selector::parse("article div.news-article--content--body p").unwrap();
        let text = document
            .select(&selector)
            .map(|x| x.text().collect::<Vec<_>>().join("\n"));
        let html = document
            .select(&selector)
            .map(|x| x.html())
            .collect::<Vec<_>>()
            .join("\n");
        return Ok((
            self.trim_text(&html),
            self.trim_text(&text.collect::<Vec<_>>().join("\n")),
        ));
    }
}
