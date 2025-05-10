use crate::models::web_article::{Cookie, Html, Text, WebArticleResource, WebSiteResource};
use chrono::DateTime;
use dotenvy::dotenv;
use feed_parser::parsers;
use request::{cookie::Jar, Url};
use shared::{
    errors::{AppError, AppResult},
    id::WebSiteId,
};
use std::sync::Arc;

const URL: &str = "https://xtech.nikkei.com/rss/index.rdf";

#[derive(Debug, Clone)]
pub struct NikkeiXTech {
    site_id: WebSiteId,
    site_name: String,
    url: Url,
    cookies: Option<String>,
}

impl NikkeiXTech {
    pub fn new() -> Self {
        NikkeiXTech {
            site_id: WebSiteId::default(),
            site_name: "Nikkei XTech".to_string(),
            cookies: None,
            url: Url::parse(URL).unwrap(),
        }
    }
}

impl Default for NikkeiXTech {
    fn default() -> Self {
        NikkeiXTech::new()
    }
}

#[async_trait::async_trait]
impl WebSiteResource for NikkeiXTech {
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
        dotenv().ok();
        if let Some(cookies) = &self.cookies {
            return Ok(cookies.clone());
        }

        // Login to Nikkei XTech - ID email
        let auth_url = Url::parse("https://id.nikkei.com/login/?auth=eyJhbGciOiJFUzI1NiJ9.eyJzdWIiOiJTRklRWjlZbjlYdzRuSVBHLUZvU1dsOFRVc1lUa3MzLUpvZExBbUYyIiwiaXNzIjoiSUFNIiwiYXVkIjpbIk1XRUIiXSwiZXhwIjoxNzQ1MzAyMjk1LCJpYXQiOjE3NDUzMDA0OTUsInN0YXRlIjoibG9naW5faWRfcmVxdWlyZWQiLCJvcHQiOiJsb2dpbiIsInNjb3BlcyI6WyJvcGVuaWQiXSwiY2xpZW50X2lkIjoiTklEIiwicHJpdmFjeV9wb2xpY3kiOnsidXJsIjoiaHR0cHM6Ly93d3cubmlra2VpLmNvbS9sb3VuZ2UvcHJpdmFjeS9wcml2YWN5LXZlcjEuaHRtbCIsInZlcnNpb24iOjEsInR5cGUiOiJOSUtLRUlfUFJJVkFDWV9QT0xJQ1kifSwidGVybXNfb2Zfc2VydmljZSI6eyJ1cmwiOiJodHRwczovL3d3dy5uaWtrZWkuY29tL2xvdW5nZS9oZWxwL3Rvcy5odG1sIiwidmVyc2lvbiI6MCwidHlwZSI6Ik5JS0tFSV9JRF9URVJNU19PRl9TRVJWSUNFIn19.UVX-Zyi7pQCCR76CrPrTQnDRk1gDtb22o9BghQk-hQTN4yfrJ6gbCL08H4P0jSntD_udLr4T_DGZchrZDhSnPw").unwrap();
        let response = self.request(auth_url.as_str(), &String::default()).await?;
        if response.status() != 200 {
            return Err(AppError::ScrapeError("Failed to login".into()));
        }
        println!("Response: {:?}", response);
        let cookie_str = response
            .cookies()
            .map(|c| format!("{}={}", c.name(), c.value()))
            .collect::<Vec<_>>()
            .join("; ");
        println!("Cookies: {}", cookie_str);
        let cookies = Arc::new(Jar::default());
        let login_url = Url::parse("https://id.nikkei.com/login/id").unwrap();
        cookies.add_cookie_str(&cookie_str, &login_url);
        let client = request::Client::builder()
            .cookie_store(true)
            .cookie_provider(cookies)
            .build()?;

        let param = vec![("login-id-email", std::env::var("NIKKEI_ID_EMAIL").unwrap())];
        let response = client.post(login_url).query(&param).send().await?;
        if response.status() != 200 {
            return Err(AppError::ScrapeError("Failed to login".into()));
        }
        println!("Response: {:?}", response);

        // Login to Nikkei XTech - Password
        let password_url = Url::parse("https://id.nikkei.com/login/password").unwrap();
        let cookie_str = response
            .cookies()
            .map(|c| format!("{}={}", c.name(), c.value()))
            .collect::<Vec<_>>()
            .join("; ");
        println!("Cookies: {}", cookie_str);
        let cookies = Arc::new(Jar::default());
        cookies.add_cookie_str(&cookie_str, &password_url);
        let client = request::Client::builder()
            .cookie_store(true)
            .cookie_provider(cookies)
            .build()?;

        let param = vec![("login_password_password", std::env::var("NIKKEI_PASSWORD").unwrap())];
        let response = client.post(password_url).query(&param).send().await?;
        if response.status() != 200 {
            return Err(AppError::ScrapeError("Failed to login".into()));
        }
        let cookies = response
            .cookies()
            .map(|c| format!("{}={}", c.name(), c.value()))
            .collect::<Vec<_>>()
            .join("; ");
        self.cookies = Some(cookies.clone());
        return Ok(cookies);
    }
    async fn get_articles(&mut self) -> AppResult<Vec<WebArticleResource>> {
        let cookies = self.login().await?;
        let response = self.request(self.url.as_str(), &cookies).await?;
        let feeds = match parsers::rss1::parse(response.text().await?.as_str()) {
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
                    self.site_url().to_string(),
                    feed.title.clone(),
                    feed.link.clone(),
                    feed.description.clone().unwrap_or("".to_string()),
                    DateTime::parse_from_rfc3339(&feed.date.clone().unwrap())
                        .unwrap()
                        .into(),
                )
            })
            .collect::<Vec<WebArticleResource>>();
        return Ok(articles);
    }
    async fn parse_article(&mut self, url: &str) -> AppResult<(Html, Text)> {
        let url = Url::parse(url).unwrap();
        // let cookies = self.login().await?;
        // TODO: Login to Nikkei XTech - ID email
        let cookies = self.cookies.clone().unwrap_or_default();
        let response = self.request(url.as_str(), &cookies).await?;
        let document = scraper::Html::parse_document(response.text().await?.as_str());
        let selector = scraper::Selector::parse("main article div.p-article div.p-article_body").unwrap();
        let article = document.select(&selector).next().unwrap();
        let text = article.text().collect::<Vec<_>>().join("\n");
        let html = article.html().to_string();
        return Ok((self.trim_text(&html), self.trim_text(&text)));
    }
}
