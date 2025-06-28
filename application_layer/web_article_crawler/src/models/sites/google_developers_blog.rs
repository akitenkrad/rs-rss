use crate::models::web_article::{Cookie, Html, Text, WebArticleResource, WebSiteResource};
use chrono::DateTime;
use feed_parser::parsers;
use request::Url;
use shared::errors::{AppError, AppResult};
use shared::id::WebSiteId;

const URL: &str = "https://developers-jp.googleblog.com/atom.xml";

#[derive(Debug, Clone)]
pub struct GoogleDevelopersBlog {
    site_id: WebSiteId,
    site_name: String,
    url: Url,
}

impl GoogleDevelopersBlog {
    pub fn new() -> Self {
        Self {
            site_id: WebSiteId::default(),
            site_name: "Google Developers Blog".to_string(),
            url: Url::parse(URL).unwrap(),
        }
    }
}

impl Default for GoogleDevelopersBlog {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl WebSiteResource for GoogleDevelopersBlog {
    fn site_id(&self) -> WebSiteId {
        self.site_id.clone()
    }
    fn site_name(&self) -> String {
        self.site_name.clone()
    }
    fn site_url(&self) -> Url {
        self.url.clone()
    }
    fn domain(&self) -> String {
        self.url.domain().unwrap().to_string()
    }
    fn set_site_id(&mut self, site_id: WebSiteId) {
        self.site_id = site_id;
    }
    async fn login(&mut self) -> AppResult<Cookie> {
        Ok(Cookie::default())
    }
    async fn get_articles(&mut self) -> AppResult<Vec<WebArticleResource>> {
        let cookies = self.login().await?;
        let response = self.request(self.url.as_str(), &cookies).await?;
        let feeds = match parsers::atom::parse(response.text().await?.as_str()) {
            Ok(feed) => feed,
            Err(e) => {
                return Err(AppError::ScrapeError(format!("Failed to parse RSS: {}", e)));
            }
        };
        let articles = feeds
            .iter()
            .map(|feed| {
                WebArticleResource::new(
                    self.site_name(),
                    self.site_url().to_string(),
                    feed.title.clone(),
                    feed.link.clone(),
                    feed.description.clone().unwrap_or("".to_string()),
                    DateTime::parse_from_rfc3339(&feed.publish_date.clone().unwrap())
                        .unwrap()
                        .into(),
                )
            })
            .collect::<Vec<WebArticleResource>>();
        Ok(articles)
    }
    async fn parse_article(&mut self, url: &str) -> AppResult<(Html, Text)> {
        let url = Url::parse(url).unwrap();
        let cookies = self.login().await?;
        let response = self.request(url.as_str(), &cookies).await?;
        let document = scraper::Html::parse_document(response.text().await?.as_str());
        let selector = scraper::Selector::parse("#main div.post div.post-body div.post-content").unwrap();
        match document.select(&selector).next() {
            Some(elem) => {
                let html = elem.html().to_string();
                let text = html2md::rewrite_html(&html, false);
                Ok((self.trim_text(&html), self.trim_text(&text)))
            }
            None => Err(AppError::ScrapeError("Failed to parse article text".into())),
        }
    }
}
