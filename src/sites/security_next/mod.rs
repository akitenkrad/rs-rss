use crate::sites::{Category, Html, Site, Text, WebArticle};
use chrono::DateTime;
use feed_parser::parsers;
pub struct SecurityNext {}

#[cfg(test)]
mod tests;

#[async_trait::async_trait]
impl Site for SecurityNext {
    fn name(&self) -> String {
        return "Security Next".to_string();
    }
    fn category(&self) -> Category {
        return Category::Security;
    }
    async fn get_articles(&self) -> Result<Vec<WebArticle>, String> {
        let url = "https://www.security-next.com/feed".to_string();
        let body = self.request(&url).await;
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
        let selector = scraper::Selector::parse("div.main div.content p").unwrap();
        let mut text = String::new();
        let a_sel = scraper::Selector::parse("a").unwrap();
        for p in document.select(&selector) {
            if p.select(&a_sel).next().is_some() {
                continue;
            }
            text.push_str(&p.text().collect::<Vec<_>>().join("\n"));
        }
        let html = document
            .select(&selector)
            .map(|x| x.html())
            .collect::<Vec<_>>()
            .join("\n");
        let html_text = self.trim_text(&html);
        let mut text = self.trim_text(&text);
        text.push_str("\n\nSecurity Next\n");
        return Ok((html_text, text));
    }
}
