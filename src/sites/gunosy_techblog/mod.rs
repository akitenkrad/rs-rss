use crate::sites::{Category, Site, WebArticle};
use chrono::DateTime;
use feed_parser::parsers;
pub struct GunosyTechBlog {}

#[cfg(test)]
mod tests;

impl Site for GunosyTechBlog {
    fn name(&self) -> String {
        return "Gunosy Tech Blog".to_string();
    }
    fn category(&self) -> Category {
        return Category::Blog;
    }
    async fn get_articles(&self) -> Result<Vec<WebArticle>, String> {
        let url = "https://tech.gunosy.io/feed".to_string();
        let body = self.request(&url).await;
        let feeds = parsers::atom::parse(&body).unwrap();
        let mut articles = Vec::new();
        for feed in feeds {
            articles.push(WebArticle {
                title: feed.title,
                url: feed.link,
                text: feed.description.unwrap_or("".to_string()),
                timestamp: DateTime::parse_from_rfc3339(&feed.updated.unwrap())
                    .unwrap()
                    .into(),
            });
        }
        return Ok(articles);
    }
    async fn get_article_text(&self, url: &String) -> Result<String, String> {
        let body = self.request(url).await;
        let document = scraper::Html::parse_document(&body);
        let selector = scraper::Selector::parse("#main article div.entry-content").unwrap();
        let article = document.select(&selector).next().unwrap();
        let text = article.text().collect::<Vec<_>>().join("\n");
        return Ok(self.trim_text(&text));
    }
}
