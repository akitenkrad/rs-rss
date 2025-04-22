use crate::sites::{Category, Cookie, Html, Site, Text, WebArticle};
use anyhow::{Error, Result};
use chrono::DateTime;
use feed_parser::parsers;
use request::Url;
use scraper::Selector;

const URL: &str = "https://ai-news.dev/feeds/";

#[derive(Debug, Clone)]
pub struct AINews {
    url: Url,
}

impl AINews {
    pub fn new() -> Self {
        Self {
            url: Url::parse(URL).unwrap(),
        }
    }
}

impl Default for AINews {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl Site for AINews {
    fn name(&self) -> String {
        return "AI News".to_string();
    }
    fn category(&self) -> super::Category {
        return Category::News;
    }
    fn domain(&self) -> String {
        return self.url.domain().unwrap().to_string();
    }
    async fn login(&mut self) -> Result<Cookie> {
        return Ok(String::default());
    }
    async fn get_articles(&mut self) -> Result<Vec<WebArticle>> {
        let cookies = self.login().await?;
        let response = self.request(self.url.as_str(), &cookies).await?;

        let feeds = parsers::atom::parse(response.text().await?.as_str()).expect("Failed to parse Atom feed");
        let articles = feeds
            .iter()
            .filter_map(|feed| {
                Some(WebArticle::new(
                    self.name(),
                    feed.title.clone(),
                    feed.link.clone(),
                    feed.description.clone().unwrap_or("".to_string()),
                    DateTime::parse_from_rfc3339(&feed.publish_date.clone().unwrap()).unwrap().into(),
                ))
            })
            .collect::<Vec<WebArticle>>();
        return Ok(articles);
    }

    async fn parse_article(&mut self, url: &str) -> Result<(Html, Text)> {
        let cookies = self.login().await?;
        let response = self.request(url, &cookies).await?;
        let doc = scraper::Html::parse_document(response.text().await?.as_str());
        let sel = Selector::parse("body").unwrap();
        match doc.select(&sel).next() {
            Some(elem) => {
                let text = elem.text().collect::<Vec<_>>().join("\n");
                let html = elem.html().to_string();
                return Ok((self.trim_text(&html), self.trim_text(&text)));
            }
            None => {
                return Err(Error::msg("Failed to parse article text"));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ai_news() {
        let mut site = AINews::default();
        let articles = site.get_articles().await;
        if let Ok(articles) = articles {
            if articles.len() == 0 {
                println!("No articles found");
                assert!(true);
                return;
            }

            let article = articles.get(0).unwrap();
            println!("Article: {:?}", article);
            let html_and_text = site.parse_article(&article.url).await;
            match html_and_text {
                Ok(html_and_text) => {
                    let (html, text) = html_and_text;
                    println!("HTML: {}", html);
                    println!("Text: {}", text);
                    assert!(html.len() > 0);
                    assert!(text.len() > 0);
                }
                Err(e) => {
                    println!("Error: {}", e);
                    assert!(false);
                }
            }
        } else {
            assert!(false);
        }
    }
}
