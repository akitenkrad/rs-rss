use chrono::{DateTime, Local};
use kernel::models::web_article::{WebArticle, WebSite};
use regex::Regex;
use registry::AppRegistryImpl;
use request::{Response, Url};
use serde::{Deserialize, Serialize};
use shared::errors::{AppError, AppResult};
use shared::id::{WebArticleId, WebSiteId};
use std::boxed::Box;

pub type Html = String;
pub type Text = String;
pub type Cookie = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebArticleResource {
    pub site_name: String,
    pub site_url: String,
    pub title: String,
    pub article_url: String,
    pub description: String,
    pub timestamp: DateTime<Local>,
    pub text: String,
    pub html: String,
}

impl WebArticleResource {
    pub fn new(
        site_name: String,
        site_url: String,
        title: String,
        article_url: String,
        description: String,
        timestamp: DateTime<Local>,
    ) -> Self {
        Self {
            site_name,
            site_url,
            title,
            article_url,
            description,
            timestamp,
            text: "".to_string(),
            html: "".to_string(),
        }
    }
}

impl From<WebArticleResource> for WebArticle {
    fn from(article: WebArticleResource) -> Self {
        let WebArticleResource {
            site_name,
            site_url,
            title,
            article_url,
            description,
            timestamp,
            text,
            html,
        } = article;
        let reg_cdata = Regex::new(r"<!\[CDATA\[(?<text>.+?)\]\]>").unwrap();
        let title = reg_cdata
            .captures(&title)
            .and_then(|cap| cap.name("text").map(|m| m.as_str().to_string()))
            .unwrap_or(title);
        let description = reg_cdata
            .captures(&description)
            .and_then(|cap| cap.name("text").map(|m| m.as_str().to_string()))
            .unwrap_or(description);
        let description = html2md::rewrite_html(&description, false);
        WebArticle {
            site: WebSite::new(WebSiteId::new(), site_name, site_url),
            article_id: WebArticleId::new(),
            title,
            url: article_url,
            description,
            timestamp: timestamp.date_naive(),
            text,
            html,
            ..Default::default()
        }
    }
}

#[async_trait::async_trait]
pub trait WebSiteResource: Send + Sync {
    fn site_id(&self) -> WebSiteId;
    fn site_name(&self) -> String;
    fn site_url(&self) -> Url;
    async fn get_articles(&mut self) -> AppResult<Vec<WebArticleResource>>;
    async fn parse_article(&mut self, url: &str) -> AppResult<(Html, Text)>;
    async fn login(&mut self) -> AppResult<Cookie>;
    fn domain(&self) -> String;
    fn trim_text(&self, text: &str) -> String {
        let re = Regex::new(r"\s\s+").unwrap();
        re.replace_all(text, "\n").to_string()
    }
    async fn get_site_id(&self, registry: &AppRegistryImpl) -> AppResult<WebSiteId> {
        let name = self.site_name();
        let url = self.site_url();
        let site = registry
            .web_site_repository()
            .select_or_create_web_site(&name, url.as_str())
            .await?;
        Ok(site.site_id)
    }
    fn set_site_id(&mut self, site_id: WebSiteId);
    fn get_domain(&self, url: &str) -> AppResult<String> {
        Ok(Url::parse(url)?.domain().unwrap_or_default().to_string())
    }
    async fn request(&self, url: &str, cookie_str: &str) -> AppResult<Response> {
        let url = request::Url::parse(url).unwrap();

        // Set Header
        let mut headers = request::header::HeaderMap::new();
        headers.insert(
            request::header::USER_AGENT,
            format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
                .parse()
                .unwrap(),
        );

        // Set Cookie
        let cookies = std::sync::Arc::new(request::cookie::Jar::default());
        cookies.add_cookie_str(cookie_str, &url);

        let client_builder = request::ClientBuilder::new();
        let client = client_builder
            .default_headers(headers)
            .cookie_provider(cookies)
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .unwrap();

        let response = match client.get(url).send().await {
            Ok(response) => response,
            Err(e) => return Err(AppError::RequestError(e)),
        };
        Ok(response)
    }
}

impl From<Box<dyn WebSiteResource>> for WebSite {
    fn from(site: Box<dyn WebSiteResource>) -> Self {
        Self {
            site_id: site.site_id(),
            name: site.site_name(),
            url: site.domain(),
        }
    }
}
