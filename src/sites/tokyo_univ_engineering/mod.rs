use crate::sites::{Category, Html, Site, Text, WebArticle};
use chrono::DateTime;
use feed_parser::parsers;
pub struct TokyoUniversityEngineering {}

#[cfg(test)]
mod tests;

#[async_trait::async_trait]
impl Site for TokyoUniversityEngineering {
    fn name(&self) -> String {
        return "Tokyo University Enginerring".to_string();
    }
    fn category(&self) -> Category {
        return Category::Organization;
    }
    async fn get_articles(&self) -> Result<Vec<WebArticle>, String> {
        let url = "https://www.t.u-tokyo.ac.jp/press/rss.xml".to_string();
        let body = self.request(&url).await;
        let feeds = parsers::rss2::parse(&body).unwrap();
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
            scraper::Selector::parse("main div.ly_cont div.blog_title,div.bl_wysiwyg").unwrap();
        let mut text = String::new();
        for p in document.select(&selector) {
            text.push_str(&p.text().collect::<Vec<_>>().join("\n"));
            text.push_str("\n");
        }
        let html = document
            .select(&selector)
            .next()
            .unwrap()
            .html()
            .to_string();
        return Ok((self.trim_text(&html), self.trim_text(&text)));
    }
}
