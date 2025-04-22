use crate::sites::{Category, Cookie, Html, Site, Text, WebArticle};
use anyhow::{Error, Result};
use chrono::DateTime;
use dotenv::dotenv;
use feed_parser::parsers;
use request::{cookie::Jar, Url};
use std::sync::Arc;

const URL: &str = "https://xtech.nikkei.com/rss/index.rdf";

#[derive(Debug, Clone)]
pub struct NikkeiXTech {
    cookies: Option<String>,
    url: Url,
}

impl NikkeiXTech {
    pub fn new() -> Self {
        NikkeiXTech {
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
impl Site for NikkeiXTech {
    fn name(&self) -> String {
        return "Nikkei XTech".to_string();
    }
    fn category(&self) -> Category {
        return Category::News;
    }
    fn domain(&self) -> String {
        self.url.domain().unwrap().to_string()
    }
    async fn login(&mut self) -> Result<Cookie> {
        dotenv().ok();
        if let Some(cookies) = &self.cookies {
            return Ok(cookies.clone());
        }

        // Login to Nikkei XTech - ID email
        let auth_url = Url::parse("https://id.nikkei.com/login/?auth=eyJhbGciOiJFUzI1NiJ9.eyJzdWIiOiJTRklRWjlZbjlYdzRuSVBHLUZvU1dsOFRVc1lUa3MzLUpvZExBbUYyIiwiaXNzIjoiSUFNIiwiYXVkIjpbIk1XRUIiXSwiZXhwIjoxNzQ1MzAyMjk1LCJpYXQiOjE3NDUzMDA0OTUsInN0YXRlIjoibG9naW5faWRfcmVxdWlyZWQiLCJvcHQiOiJsb2dpbiIsInNjb3BlcyI6WyJvcGVuaWQiXSwiY2xpZW50X2lkIjoiTklEIiwicHJpdmFjeV9wb2xpY3kiOnsidXJsIjoiaHR0cHM6Ly93d3cubmlra2VpLmNvbS9sb3VuZ2UvcHJpdmFjeS9wcml2YWN5LXZlcjEuaHRtbCIsInZlcnNpb24iOjEsInR5cGUiOiJOSUtLRUlfUFJJVkFDWV9QT0xJQ1kifSwidGVybXNfb2Zfc2VydmljZSI6eyJ1cmwiOiJodHRwczovL3d3dy5uaWtrZWkuY29tL2xvdW5nZS9oZWxwL3Rvcy5odG1sIiwidmVyc2lvbiI6MCwidHlwZSI6Ik5JS0tFSV9JRF9URVJNU19PRl9TRVJWSUNFIn19.UVX-Zyi7pQCCR76CrPrTQnDRk1gDtb22o9BghQk-hQTN4yfrJ6gbCL08H4P0jSntD_udLr4T_DGZchrZDhSnPw").unwrap();
        let response = self.request(auth_url.as_str(), &String::default()).await?;
        if response.status() != 200 {
            return Err(Error::msg("Failed to login"));
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
        let client = request::Client::builder().cookie_store(true).cookie_provider(cookies).build()?;

        let param = vec![("login-id-email", std::env::var("NIKKEI_ID_EMAIL").unwrap())];
        let response = client.post(login_url).query(&param).send().await?;
        if response.status() != 200 {
            return Err(Error::msg("Failed to login"));
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
        let client = request::Client::builder().cookie_store(true).cookie_provider(cookies).build()?;

        let param = vec![("login_password_password", std::env::var("NIKKEI_PASSWORD").unwrap())];
        let response = client.post(password_url).query(&param).send().await?;
        if response.status() != 200 {
            return Err(Error::msg("Failed to login"));
        }
        let cookies = response
            .cookies()
            .map(|c| format!("{}={}", c.name(), c.value()))
            .collect::<Vec<_>>()
            .join("; ");
        self.cookies = Some(cookies.clone());
        return Ok(cookies);
    }
    async fn get_articles(&mut self) -> Result<Vec<WebArticle>> {
        let cookies = self.login().await?;
        let response = self.request(self.url.as_str(), &cookies).await?;
        let feeds = match parsers::rss1::parse(response.text().await?.as_str()) {
            Ok(feeds) => feeds,
            Err(e) => {
                return Err(Error::msg(format!("Failed to parse RSS: {}", e)));
            }
        };
        let articles = feeds
            .iter()
            .map(|feed| {
                WebArticle::new(
                    self.name(),
                    feed.title.clone(),
                    feed.link.clone(),
                    feed.description.clone().unwrap_or("".to_string()),
                    DateTime::parse_from_rfc3339(&feed.date.clone().unwrap()).unwrap().into(),
                )
            })
            .collect::<Vec<WebArticle>>();
        return Ok(articles);
    }
    async fn parse_article(&mut self, url: &str) -> Result<(Html, Text)> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_nikkei_xtech_login() {
        let mut site = NikkeiXTech::default();
        let cookies = site.login().await;
        if let Ok(cookies) = cookies {
            println!("Cookies: {}", cookies);
            assert!(cookies.len() > 0);
        } else {
            assert!(false);
        }
    }

    #[tokio::test]
    async fn test_nikkei_xtech() {
        let mut site = NikkeiXTech::default();
        let articles = site.get_articles().await;
        if let Ok(articles) = articles {
            if articles.len() == 0 {
                println!("No articles found");
                assert!(true);
                return;
            }

            let article = articles.get(0).unwrap();
            println!("Article: {:?}", article);
            let html_and_text = site.parse_article(&article.url).await;
            match html_and_text {
                Ok(html_and_text) => {
                    let (html, text) = html_and_text;
                    println!("HTML: {}", html);
                    println!("Text: {}", text);
                    assert!(html.len() > 0);
                    assert!(text.len() > 0);
                }
                Err(e) => {
                    println!("Error: {}", e);
                    assert!(false);
                }
            }
        } else {
            assert!(false);
        }
    }
}
