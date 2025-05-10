use crate::models::web_article::{Cookie, Html, Text, WebArticleResource, WebSiteResource};
use chrono::DateTime;
use request::Url;
use scraper::Selector;
use shared::errors::AppResult;
use shared::id::WebSiteId;

const URL: &str = "https://stockmark-tech.hatenablog.com/";

#[derive(Debug, Clone)]
pub struct StockmarkTechBlog {
    site_id: WebSiteId,
    site_name: String,
    url: Url,
}

impl StockmarkTechBlog {
    pub fn new() -> Self {
        Self {
            site_id: WebSiteId::default(),
            site_name: "Stockmark Tech Blog".to_string(),
            url: Url::parse(URL).unwrap(),
        }
    }
}

impl Default for StockmarkTechBlog {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl WebSiteResource for StockmarkTechBlog {
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
        self.url.domain().unwrap().to_string()
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
        let doc = scraper::Html::parse_document(response.text().await?.as_str());

        // parse html
        let mut articles: Vec<WebArticleResource> = Vec::new();
        let post_selector = Selector::parse("#main").unwrap();
        let posts = doc.select(&post_selector);
        for post in posts {
            let desc_selector = Selector::parse("div.archive-entry-body p.entry-description").unwrap();
            let title_selector = Selector::parse("div.archive-entry-header").unwrap();
            let url_selector = Selector::parse("div.archive-entry-header h1 a").unwrap();
            let date_selector = Selector::parse("div.archive-entry-header div.archive-date").unwrap();

            let article = WebArticleResource::new(
                self.site_name(),
                self.site_url().to_string(),
                post.select(&title_selector).next().unwrap().text().collect(),
                post.select(&url_selector)
                    .next()
                    .unwrap()
                    .value()
                    .attr("href")
                    .unwrap()
                    .to_string(),
                post.select(&desc_selector).next().unwrap().text().collect(),
                DateTime::parse_from_str(
                    &format!(
                        "{} 00:00:00+0900",
                        post.select(&date_selector)
                            .next()
                            .unwrap()
                            .text()
                            .collect::<Vec<_>>()
                            .join("")
                    ),
                    "%Y-%m-%d %H:%M:%S%z",
                )
                .unwrap()
                .into(),
            );
            articles.push(article);
        }
        return Ok(articles);
    }
    async fn parse_article(&mut self, url: &str) -> AppResult<(Html, Text)> {
        let url = Url::parse(url).unwrap();
        let cookies = self.login().await?;
        let response = self.request(url.as_str(), &cookies).await?;
        let doc = scraper::Html::parse_document(response.text().await?.as_str());
        let selector = Selector::parse("#main div.entry-inner").unwrap();
        let article = doc.select(&selector).next().unwrap();
        let text = article.text().collect::<Vec<_>>().join("\n");
        let html = article.html().to_string();
        return Ok((self.trim_text(&html), self.trim_text(&text)));
    }
}
