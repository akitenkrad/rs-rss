use crate::sites::{Category, Cookie, Html, Site, Text, WebArticle};
use anyhow::{Error, Result};
use chrono::DateTime;
use feed_parser::parsers;
use request::Url;

const URL: &str = "https://www.crowdstrike.com/en-us/blog/feed";

#[derive(Debug, Clone)]
pub struct CrowdStrikeBlog {
    url: Url,
}

impl CrowdStrikeBlog {
    pub fn new() -> Self {
        let url = Url::parse(URL).unwrap();
        CrowdStrikeBlog { url }
    }
}

impl Default for CrowdStrikeBlog {
    fn default() -> Self {
        CrowdStrikeBlog::new()
    }
}

#[async_trait::async_trait]
impl Site for CrowdStrikeBlog {
    fn name(&self) -> String {
        return "CrowdStrike Blog".to_string();
    }
    fn category(&self) -> Category {
        return Category::Security;
    }
    fn domain(&self) -> String {
        return self.url.domain().unwrap().to_string();
    }
    async fn login(&mut self) -> Result<Cookie> {
        return Ok(Cookie::default());
    }
    async fn get_articles(&mut self) -> Result<Vec<WebArticle>> {
        let cookies = self.login().await?;
        let response = self.request(self.url.as_str(), &cookies).await?;
        let feeds = match parsers::rss2::parse(response.text().await?.as_str()) {
            Ok(feeds) => feeds,
            Err(e) => {
                return Err(Error::msg(format!("Failed to parse RSS: {}", e)));
            }
        };
        let articles = feeds
            .iter()
            .map(|feed| {
                WebArticle::new(
                    self.name(),
                    feed.title.clone(),
                    feed.link.clone(),
                    feed.description.clone().unwrap_or("".to_string()),
                    DateTime::parse_from_str(&feed.publish_date.clone().unwrap(), "%b %d, %Y %H:%M:%S%z")
                        .unwrap()
                        .into(),
                )
            })
            .collect::<Vec<WebArticle>>();
        return Ok(articles);
    }
    async fn parse_article(&mut self, url: &str) -> Result<(Html, Text)> {
        let url = Url::parse(url).unwrap();
        let cookies = self.login().await?;
        let response = self.request(url.as_str(), &cookies).await?;
        let document = scraper::Html::parse_document(response.text().await?.as_str());
        let selector = scraper::Selector::parse("div.root div.cmp-container-wp").unwrap();
        match document.select(&selector).next() {
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
    async fn test_crowdstrike_blog() {
        let mut site = CrowdStrikeBlog::default();
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
