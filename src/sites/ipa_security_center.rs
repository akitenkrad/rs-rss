use crate::sites::{Category, Cookie, Html, Site, Text, WebArticle};
use anyhow::Result;
use chrono::DateTime;
use feed_parser::parsers;
use request::Url;

const URL: &str = "https://www.ipa.go.jp/security/rss/alert.rdf";

#[derive(Debug, Clone)]
pub struct IPASecurityCenter {
    url: Url,
}

impl IPASecurityCenter {
    pub fn new() -> Self {
        Self {
            url: Url::parse(URL).unwrap(),
        }
    }
}

impl Default for IPASecurityCenter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl Site for IPASecurityCenter {
    fn name(&self) -> String {
        return "IPA Security Center".to_string();
    }
    fn category(&self) -> Category {
        return Category::Security;
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
        let feeds = match parsers::rss1::parse(response.text().await?.as_str()) {
            Ok(feeds) => feeds,
            Err(e) => {
                return Err(anyhow::anyhow!("Failed to parse RSS feed: {}", e));
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
        let cookies = self.login().await?;
        let response = self.request(url.as_str(), &cookies).await?;
        let document = scraper::Html::parse_document(response.text().await?.as_str());
        let selector = scraper::Selector::parse("div.news-detail main h1.ttl,h2.ttl,p.article-txt,span.list__item__txt").unwrap();
        let mut text = String::new();
        for p in document.select(&selector) {
            text.push_str(&p.text().collect::<Vec<_>>().join("\n"));
            text.push_str("\n");
        }
        let html = document.select(&selector).next().unwrap().html().to_string();
        return Ok((self.trim_text(&html), self.trim_text(&text)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ipa_security_center() {
        let mut site = IPASecurityCenter::default();
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
