use crate::models::web_article::{Cookie, Html, Text, WebArticleResource, WebSiteResource};
use chrono::DateTime;
use request::Url;
use scraper::Selector;
use shared::errors::AppResult;
use shared::id::WebSiteId;

const URL: &str = "https://supership.jp/news/";

#[derive(Debug, Clone)]
pub struct Supership {
    site_id: WebSiteId,
    site_name: String,
    url: Url,
}

impl Supership {
    pub fn new() -> Self {
        Self {
            site_id: WebSiteId::default(),
            site_name: "Supership".to_string(),
            url: Url::parse(URL).unwrap(),
        }
    }
}

impl Default for Supership {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl WebSiteResource for Supership {
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
        let sel = Selector::parse("main article ul.p-magazine__archive li.p-magazine__card").unwrap();
        for li in doc.select(&sel) {
            let title_sel = Selector::parse("p.p-magazine__card_title").unwrap();
            let title_text = li
                .select(&title_sel)
                .next()
                .unwrap()
                .text()
                .collect::<Vec<_>>()
                .join("");
            let url_sel = Selector::parse("a").unwrap();
            let url = li
                .select(&url_sel)
                .next()
                .unwrap()
                .value()
                .attr("href")
                .unwrap()
                .to_string();
            let pubdate_sel = Selector::parse("time.p-magazine__card_time").unwrap();
            let publish_date_text = li
                .select(&pubdate_sel)
                .next()
                .unwrap()
                .text()
                .collect::<Vec<_>>()
                .join("")
                + " 00:00:00+09:00";
            let publish_date = match DateTime::parse_from_str(&publish_date_text, "%Y.%m.%d %H:%M:%S%z") {
                Ok(x) => x,
                Err(e) => {
                    println!("Got ERROR {}: {}", e, publish_date_text);
                    continue;
                }
            };
            let article = WebArticleResource::new(
                self.site_name(),
                self.site_url().to_string(),
                title_text,
                url,
                "".to_string(),
                publish_date.into(),
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
        let sel = Selector::parse("main article div.c-grid__block--content").unwrap();
        let text = doc.select(&sel).next().unwrap().text().collect::<Vec<_>>().join("\n");
        let html = doc.select(&sel).next().unwrap().html().to_string();
        return Ok((self.trim_text(&html), self.trim_text(&text)));
    }
}
