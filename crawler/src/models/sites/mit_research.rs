use crate::models::web_article::{Category, Cookie, Html, Text, WebArticleResource, WebSiteResource};
use chrono::DateTime;
use feed_parser::parsers;
use request::Url;
use shared::{
    errors::{AppError, AppResult},
    id::WebSiteId,
};

const URL: &str = "https://news.mit.edu/rss/research";

#[derive(Debug, Clone)]
pub struct MITResearch {
    site_id: WebSiteId,
    site_name: String,
    url: Url,
}

impl MITResearch {
    pub fn new() -> Self {
        Self {
            site_id: WebSiteId::default(),
            site_name: "MIT Research".to_string(),
            url: Url::parse(URL).unwrap(),
        }
    }
}

impl Default for MITResearch {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl WebSiteResource for MITResearch {
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
                println!("Error parsing RSS feed: {}", e);
                return Err(AppError::ScrapeError("Failed to parse RSS".into()));
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
        let selector = scraper::Selector::parse("article div.news-article--content--body p").unwrap();
        let text = document.select(&selector).map(|x| x.text().collect::<Vec<_>>().join("\n"));
        let html = document.select(&selector).map(|x| x.html()).collect::<Vec<_>>().join("\n");
        return Ok((self.trim_text(&html), self.trim_text(&text.collect::<Vec<_>>().join("\n"))));
    }
}
