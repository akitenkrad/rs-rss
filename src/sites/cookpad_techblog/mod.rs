use crate::sites::{Site, WebArticle};
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
    async fn get_articles(&self) -> Vec<WebArticle> {
        let body = reqwest::get("https://techlife.cookpad.com/feed")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
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
        return articles;
    }

    async fn get_article_text(&self, url: &String) -> String {
        let html = reqwest::get(url).await.unwrap().text().await.unwrap();
        let doc = scraper::Html::parse_document(&html);
        let sel = Selector::parse("#main article div.entry-content").unwrap();
        let text = doc.select(&sel).next().unwrap().text().collect();
        return self.trim_text(&text);
    }
}
