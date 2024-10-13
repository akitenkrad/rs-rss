use crate::sites::{Category, Site, WebArticle};
use chrono::DateTime;
use feed_parser::parsers;
use scraper::Selector;
pub struct CookpadTechBlog {}

#[cfg(test)]
mod tests;

impl Site for CookpadTechBlog {
    fn name(&self) -> String {
        return "Cookpad Tech Blog".to_string();
    }
    fn category(&self) -> super::Category {
        return Category::Blog;
    }
    async fn get_articles(&self) -> Result<Vec<WebArticle>, String> {
        let url = "https://techlife.cookpad.com/feed".to_string();
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
        let doc = scraper::Html::parse_document(&body);
        let sel = Selector::parse("#main article div.entry-content").unwrap();
        let text = doc.select(&sel).next().unwrap().text().collect();
        return Ok(self.trim_text(&text));
    }
}
