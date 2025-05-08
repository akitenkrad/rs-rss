use crate::models::web_article::{Category, Cookie, Html, Text, WebArticleResource, WebSiteResource};
use chrono::DateTime;
use feed_parser::parsers;
use request::Url;
use shared::errors::{AppError, AppResult};
use shared::id::WebSiteId;

const URL: &str = "https://zenn.dev/feed";

#[derive(Debug, Clone)]
pub struct ZennTrend {
    site_id: WebSiteId,
    site_name: String,
    url: Url,
}

impl ZennTrend {
    pub fn new() -> Self {
        Self {
            site_id: WebSiteId::default(),
            site_name: "Zenn Trend".to_string(),
            url: Url::parse(URL).unwrap(),
        }
    }
}

impl Default for ZennTrend {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl WebSiteResource for ZennTrend {
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
        self.url.domain().unwrap().to_string()
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
                    DateTime::parse_from_rfc2822(&feed.publish_date.clone().unwrap()).unwrap().into(),
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
        let selector = scraper::Selector::parse("article section").unwrap();
        let article = document.select(&selector).next().unwrap();
        let text = article.text().collect::<Vec<_>>().join("\n");
        let html = article.html().to_string();
        return Ok((self.trim_text(&html), self.trim_text(&text)));
    }
}
