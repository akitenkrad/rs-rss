use crate::sites::{Category, Cookie, Html, Site, Text, WebArticle};
use anyhow::{Error, Result};
use chrono::DateTime;
use feed_parser::parsers;
use request::Url;

const URL: &str = "https://knowledge.sakura.ad.jp/feed";

#[derive(Debug, Clone)]
pub struct SakuraInternetTechBlog {
    url: Url,
}

impl SakuraInternetTechBlog {
    pub fn new() -> Self {
        Self {
            url: Url::parse(URL).unwrap(),
        }
    }
}

impl Default for SakuraInternetTechBlog {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl Site for SakuraInternetTechBlog {
    fn name(&self) -> String {
        return "CyberAgent Tech Blog".to_string();
    }
    fn category(&self) -> Category {
        return Category::Blog;
    }
    fn domain(&self) -> String {
        self.url.domain().unwrap().to_string()
    }
    async fn login(&mut self) -> Result<Cookie> {
        return Ok(Cookie::default());
    }
    async fn get_articles(&mut self) -> Result<Vec<WebArticle>> {
        let cookies = self.login().await?;
        let response = self.request(self.url.as_str(), &cookies).await?;
        let feeds = match parsers::rss2::parse(response.text().await?.as_str()) {
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
                    DateTime::parse_from_rfc2822(&feed.publish_date.clone().unwrap()).unwrap().into(),
                )
            })
            .collect::<Vec<WebArticle>>();
        return Ok(articles);
    }
    async fn parse_article(&mut self, url: &str) -> Result<(Html, Text)> {
        let url = Url::parse(url).unwrap();
        let cookies = self.login().await?;
        let response = self.request(url.as_str(), &cookies).await?;
        let document = scraper::Html::parse_document(response.text().await?.as_str());
        let selector = scraper::Selector::parse("main article div.entry-content").unwrap();
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
    async fn test_sakura_internet_techblog() {
        let mut site = SakuraInternetTechBlog::default();
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
