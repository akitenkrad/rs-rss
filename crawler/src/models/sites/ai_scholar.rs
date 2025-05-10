use crate::models::web_article::{Cookie, Html, Text, WebArticleResource, WebSiteResource};
use chrono::{DateTime, Local};
use request::Url;
use scraper::Selector;
use shared::{
    errors::{AppError, AppResult},
    id::WebSiteId,
};

const URL: &str = "https://ai-scholar.tech/";

#[derive(Debug, Clone)]
pub struct AIScholar {
    site_id: WebSiteId,
    site_name: String,
    url: Url,
}

impl AIScholar {
    pub fn new() -> Self {
        Self {
            site_id: WebSiteId::default(),
            site_name: "AI Scholar".to_string(),
            url: Url::parse(URL).unwrap(),
        }
    }
}

impl Default for AIScholar {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl WebSiteResource for AIScholar {
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
        // No login required
        return Ok(String::new());
    }
    async fn get_articles(&mut self) -> AppResult<Vec<WebArticleResource>> {
        let mut cookies = self.login().await?;
        cookies.push_str("display_language=ja;");
        let response = self.request(self.url.as_str(), &cookies).await?;

        // parse html
        let doc = scraper::Html::parse_document(response.text().await?.as_str());
        let sel = Selector::parse("body div.content main.main section.indexlists article.list-item").unwrap();
        let articles = doc
            .select(&sel)
            .map(|article| {
                let a_sel = Selector::parse("a").unwrap();
                let title_text = article
                    .select(&a_sel)
                    .next()
                    .unwrap()
                    .text()
                    .collect::<Vec<_>>()
                    .join("");
                let url = article.select(&a_sel).next().unwrap().value().attr("href").unwrap();
                let date_sel = Selector::parse("a div.list-item__description time").unwrap();
                let mut date_text = match article.select(&date_sel).next() {
                    Some(x) => x.value().attr("datetime").unwrap().to_string(),
                    None => String::default(),
                };
                date_text.push_str("+09:00");
                let desc_sel = Selector::parse("a div.list-item__description span").unwrap();
                let desc_text = match article.select(&desc_sel).next() {
                    Some(x) => x.text().collect::<Vec<_>>().join(""),
                    None => String::default(),
                };
                let date = match DateTime::parse_from_str(&date_text, "%Y-%m-%d %H:%M:%S%z") {
                    Ok(x) => x.with_timezone(&Local),
                    Err(_) => DateTime::<Local>::default(),
                };
                WebArticleResource::new(
                    self.site_name(),
                    self.site_url().to_string(),
                    title_text,
                    url.to_string(),
                    desc_text,
                    date,
                )
            })
            .collect::<Vec<WebArticleResource>>();
        return Ok(articles);
    }

    async fn parse_article(&mut self, url: &str) -> AppResult<(Html, Text)> {
        let cookies = self.login().await?;
        let response = self.request(url, &cookies).await?;
        let doc = scraper::Html::parse_document(response.text().await?.as_str());
        let sel = Selector::parse("article").unwrap();
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
