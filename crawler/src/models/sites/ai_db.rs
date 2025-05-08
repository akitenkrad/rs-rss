use crate::models::web_article::{Category, Cookie, Html, Text, WebArticleResource, WebSiteResource};
use chrono::DateTime;
use dotenvy::dotenv;
use feed_parser::parsers;
use request::{cookie::Jar, Url};
use shared::errors::{AppError, AppResult};
use shared::id::WebSiteId;
use std::sync::Arc;

const URL: &str = "https://ai-data-base.com/feed";

#[derive(Debug, Clone)]
pub struct AIDB {
    site_id: WebSiteId,
    site_name: String,
    site_url: Url,
    cookies: Option<String>,
}

impl AIDB {
    pub fn new() -> Self {
        Self {
            site_id: WebSiteId::new(),
            site_name: "AI DB".to_string(),
            site_url: Url::parse(URL).unwrap(),
            cookies: None,
        }
    }
}

impl Default for AIDB {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl WebSiteResource for AIDB {
    fn site_id(&self) -> WebSiteId {
        return self.site_id.clone();
    }
    fn site_name(&self) -> String {
        return self.site_name.clone();
    }
    fn category(&self) -> Category {
        return Category::Security;
    }
    fn domain(&self) -> String {
        return self.site_url.domain().unwrap().to_string();
    }
    async fn login(&mut self) -> AppResult<Cookie> {
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
            ("swpm_password", std::env::var("AI_DB_PASSWORD").unwrap()),
        ];
        let response = match client.post(url).query(&param).send().await {
            Ok(response) => response,
            Err(e) => {
                return Err(AppError::RequestError(e));
            }
        };
        let cookies = response
            .cookies()
            .map(|c| format!("{}={}", c.name(), c.value()))
            .collect::<Vec<_>>()
            .join("; ");
        self.cookies = Some(cookies.clone());
        return Ok(cookies);
    }
    async fn get_articles(&mut self) -> AppResult<Vec<WebArticleResource>> {
        let cookies = self.login().await?;
        let response = self.request(self.site_url.as_str(), &cookies).await?;
        let feeds = match parsers::rss2::parse(response.text().await?.as_str()) {
            Ok(feeds) => feeds,
            Err(e) => {
                return Err(AppError::RssParseError(e));
            }
        };
        let articles = feeds
            .iter()
            .map(|feed| {
                WebArticleResource::new(
                    self.site_name(),
                    feed.title.clone(),
                    feed.link.clone(),
                    feed.description.clone().unwrap_or("".to_string()),
                    DateTime::parse_from_rfc2822(&feed.publish_date.clone().unwrap()).unwrap().into(),
                )
            })
            .collect::<Vec<WebArticleResource>>();
        return Ok(articles);
    }
    async fn parse_article(&mut self, url: &str) -> AppResult<(Html, Text)> {
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
                return Err(AppError::ScrapeError("Failed to parse article text".into()));
            }
        }
    }
}
