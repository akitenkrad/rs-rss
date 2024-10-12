use crate::sites::{Site, WebArticle};
use chrono::DateTime;
use feed_parser::parsers;
pub struct AIItNow {}

#[cfg(test)]
mod tests;

impl Site for AIItNow {
    fn name(&self) -> String {
        return "AI IT Now".to_string();
    }
    async fn get_articles(&self) -> Vec<WebArticle> {
        let client = reqwest::Client::new();
        let body = client
            .get("https://ainow.ai/feed/")
            .header(reqwest::header::USER_AGENT, self.user_agent())
            .send()
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
        let client = reqwest::Client::new();
        let body = client
            .get(url)
            .header(reqwest::header::USER_AGENT, self.user_agent())
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        let document = scraper::Html::parse_document(&body);
        let selector =
            scraper::Selector::parse("body div.contents div.article_area div.entry-content")
                .unwrap();
        if let Some(article) = document.select(&selector).next() {
            let text = article.text().collect::<Vec<_>>().join("\n");
            return self.trim_text(&text);
        } else {
            return "NO CONTENT".to_string();
        }
    }
}
