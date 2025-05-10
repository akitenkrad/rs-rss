use crate::models::web_article::{Cookie, Html, Text, WebArticleResource, WebSiteResource};
use chrono::DateTime;
use feed_parser::parsers;
use request::Url;
use scraper::Selector;
use shared::{
    errors::{AppError, AppResult},
    id::WebSiteId,
};

const URL: &str = "https://techlife.cookpad.com/rss";

#[derive(Debug, Clone)]
pub struct CookpadTechBlog {
    site_id: WebSiteId,
    site_name: String,
    url: Url,
}

impl CookpadTechBlog {
    pub fn new() -> Self {
        let url = Url::parse(URL).unwrap();
        CookpadTechBlog {
            site_id: WebSiteId::default(),
            site_name: "Cookpad Tech Blog".to_string(),
            url,
        }
    }
}

impl Default for CookpadTechBlog {
    fn default() -> Self {
        CookpadTechBlog::new()
    }
}

#[async_trait::async_trait]
impl WebSiteResource for CookpadTechBlog {
    fn site_id(&self) -> WebSiteId {
        return self.site_id.clone();
    }
    fn site_name(&self) -> String {
        return self.site_name.clone();
    }
    fn site_url(&self) -> Url {
        return self.url.clone();
    }
    fn domain(&self) -> String {
        return self.url.domain().unwrap().to_string();
    }
    fn set_site_id(&mut self, site_id: WebSiteId) {
        self.site_id = site_id;
    }
    async fn login(&mut self) -> AppResult<Cookie> {
        return Ok(Cookie::default());
    }
    async fn get_articles(&mut self) -> AppResult<Vec<WebArticleResource>> {
        let cookies = self.login().await?;
        let response = self.request(self.url.as_str(), &cookies).await?;
        let feeds = parsers::atom::parse(response.text().await?.as_str()).expect("Failed to parse Atom feed");
        let articles = feeds
            .iter()
            .map(|feed| {
                WebArticleResource::new(
                    self.site_name(),
                    self.site_url().to_string(),
                    feed.title.clone(),
                    feed.link.clone(),
                    feed.description.clone().unwrap_or("".to_string()),
                    DateTime::parse_from_rfc3339(&feed.updated.clone().unwrap())
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
        let doc = scraper::Html::parse_document(response.text().await?.as_str());
        let sel = Selector::parse("#main article div.entry-content").unwrap();
        match doc.select(&sel).next() {
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
