use crate::models::web_article::{Category, Cookie, Html, Text, WebArticleResource, WebSiteResource};
use chrono::DateTime;
use feed_parser::parsers;
use request::Url;
use shared::{
    errors::{AppError, AppResult},
    id::WebSiteId,
};

const URL: &str = "https://rss.itmedia.co.jp/rss/2.0/enterprise.xml";

#[derive(Debug, Clone)]
pub struct ITMediaEnterprise {
    site_id: WebSiteId,
    site_name: String,
    url: Url,
}

impl ITMediaEnterprise {
    pub fn new() -> Self {
        let url = Url::parse(URL).unwrap();
        Self {
            site_id: WebSiteId::default(),
            site_name: "ITMedia Enterprise".to_string(),
            url,
        }
    }
}

impl Default for ITMediaEnterprise {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl WebSiteResource for ITMediaEnterprise {
    fn site_id(&self) -> WebSiteId {
        return self.site_id.clone();
    }
    fn site_name(&self) -> String {
        return self.site_name.clone();
    }
    fn category(&self) -> Category {
        return Category::News;
    }
    fn domain(&self) -> String {
        "www.itmedia.co.jp/enterprise".to_string()
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
                return Err(AppError::ScrapeError(format!("Failed to parse RSS feed: {}", e)));
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
        if url.starts_with("https://www.itmedia.co.jp/enterprise") == false {
            return Err(AppError::ScrapeError(format!("URL unmatch: {}", url)));
        }
        let url = Url::parse(url).unwrap();
        let cookies = self.login().await?;
        let response = self.request(url.as_str(), &cookies).await?;
        let document = scraper::Html::parse_document(response.text().await?.as_str());
        let selector = match scraper::Selector::parse("#cmsBody div.inner p") {
            Ok(selector) => selector,
            Err(e) => {
                return Err(AppError::ScrapeError(format!("Failed to parse selector (#cmsBody div.inner p): {}", e)));
            }
        };
        let mut text = String::new();
        for p in document.select(&selector) {
            text.push_str(&p.text().collect::<Vec<_>>().join("\n"));
            text.push_str("\n");
        }
        let html = document.select(&selector).next().unwrap().html().to_string();
        return Ok((self.trim_text(&html), self.trim_text(&text)));
    }
}
