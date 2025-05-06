use crate::sites::{Category, Cookie, Html, Site, Text, WebArticle};
use anyhow::{Error, Result};
use chrono::DateTime;
use dotenv::dotenv;
use feed_parser::parsers;
use request::{cookie::Jar, Url};
use std::sync::Arc;

const URL: &str = "https://ai-data-base.com/feed";

#[derive(Debug, Clone)]
pub struct AIDB {
    cookies: Option<String>,
    url: Url,
}

impl AIDB {
    pub fn new() -> Self {
        Self {
            cookies: None,
            url: Url::parse(URL).unwrap(),
        }
    }
}

impl Default for AIDB {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl Site for AIDB {
    fn name(&self) -> String {
        return "AI DB".to_string();
    }
    fn category(&self) -> super::Category {
        return Category::Security;
    }
    fn domain(&self) -> String {
        return self.url.domain().unwrap().to_string();
    }
    async fn login(&mut self) -> Result<Cookie> {
        dotenv().ok();
        if let Some(cookies) = &self.cookies {
            return Ok(cookies.clone());
        }
        let url = Url::parse("https://ai-data-base.com/membership-login").unwrap();
        let response = self.request(url.as_str(), &String::default()).await?;
        let cookie_str = response
            .cookies()
            .map(|c| format!("{}={}", c.name(), c.value()))
            .collect::<Vec<_>>()
            .join("; ");
        let cookies = Arc::new(Jar::default());
        cookies.add_cookie_str(&cookie_str, &url);
        let client = request::Client::builder().cookie_store(true).cookie_provider(cookies).build()?;

        let param = vec![
            ("swpm_user_name", std::env::var("AI_DB_USER").unwrap()),
            ("swpm_password", std::env::var("AI_DATABASE_PASSWORD").unwrap()),
        ];
        let response = client.post(url).query(&param).send().await?;
        if response.status() != 200 {
            return Err(Error::msg("Failed to login"));
        }
        let cookies = response
            .cookies()
            .map(|c| format!("{}={}", c.name(), c.value()))
            .collect::<Vec<_>>()
            .join("; ");
        self.cookies = Some(cookies.clone());
        return Ok(cookies);
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
                    DateTime::parse_from_rfc2822(&feed.publish_date.clone().unwrap()).unwrap().into(),
                )
            })
            .collect::<Vec<WebArticle>>();
        return Ok(articles);
    }
    async fn parse_article(&mut self, url: &str) -> Result<(Html, Text)> {
        let cookies = self.login().await?;
        let response = self.request(url, &cookies).await?;
        let text = response.text().await?;

        let document = scraper::Html::parse_document(text.as_str());
        let selector = scraper::Selector::parse("#contents #main_contents").unwrap();
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
    async fn test_ai_db() {
        let mut site = AIDB::default();
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
