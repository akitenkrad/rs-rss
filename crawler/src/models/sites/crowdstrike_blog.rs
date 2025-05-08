use crate::models::web_article::{Category, Cookie, Html, Text, WebArticleResource, WebSiteResource};
use chrono::DateTime;
use feed_parser::parsers;
use request::Url;
use shared::{
    errors::{AppError, AppResult},
    id::WebSiteId,
};

const URL: &str = "https://www.crowdstrike.com/en-us/blog/feed";

#[derive(Debug, Clone)]
pub struct CrowdStrikeBlog {
    site_id: WebSiteId,
    site_name: String,
    url: Url,
}

impl CrowdStrikeBlog {
    pub fn new() -> Self {
        let url = Url::parse(URL).unwrap();
        CrowdStrikeBlog {
            site_id: WebSiteId::default(),
            site_name: "CrowdStrike Blog".to_string(),
            url,
        }
    }
}

impl Default for CrowdStrikeBlog {
    fn default() -> Self {
        CrowdStrikeBlog::new()
    }
}

#[async_trait::async_trait]
impl WebSiteResource for CrowdStrikeBlog {
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
        return self.url.domain().unwrap().to_string();
    }
    async fn login(&mut self) -> AppResult<Cookie> {
        return Ok(Cookie::default());
    }
    async fn get_articles(&mut self) -> AppResult<Vec<WebArticleResource>> {
        let cookies = self.login().await?;
        let response = self.request(self.url.as_str(), &cookies).await?;
        let feeds = match parsers::rss2::parse(response.text().await?.as_str()) {
            Ok(feeds) => feeds,
            Err(e) => {
                return Err(AppError::ScrapeError(format!("Failed to parse RSS: {}", e)));
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
                    DateTime::parse_from_str(&feed.publish_date.clone().unwrap(), "%b %d, %Y %H:%M:%S%z")
                        .unwrap()
                        .into(),
                )
            })
            .collect::<Vec<WebArticleResource>>();
        return Ok(articles);
    }
    async fn parse_article(&mut self, url: &str) -> AppResult<(Html, Text)> {
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
                return Err(AppError::ScrapeError("Failed to parse article text".into()));
            }
        }
    }
}
