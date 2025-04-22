use crate::sites::{Category, Cookie, Html, Site, Text, WebArticle};
use anyhow::{Error, Result};
use chrono::{DateTime, Local};
use request::Url;
use scraper::Selector;

const URL: &str = "https://www.businessinsider.jp/science/";

#[derive(Debug, Clone)]
pub struct BusinessInsiderScience {
    url: Url,
}

impl BusinessInsiderScience {
    pub fn new() -> Self {
        let url = Url::parse(URL).unwrap();
        BusinessInsiderScience { url }
    }
}

impl Default for BusinessInsiderScience {
    fn default() -> Self {
        BusinessInsiderScience::new()
    }
}

#[async_trait::async_trait]
impl Site for BusinessInsiderScience {
    fn name(&self) -> String {
        return "Business Insider Science".to_string();
    }
    fn category(&self) -> super::Category {
        return Category::News;
    }
    fn domain(&self) -> String {
        self.url.domain().unwrap().to_string()
    }
    async fn login(&mut self) -> Result<Cookie> {
        return Ok(String::default());
    }
    async fn get_articles(&mut self) -> Result<Vec<WebArticle>> {
        let cookies = self.login().await?;
        let response = self.request(self.url.as_str(), &cookies).await?;

        // parse html
        let doc = scraper::Html::parse_document(response.text().await.unwrap().as_str());
        let sel = Selector::parse("#mainContent div.p-cardList-content div.p-cardList-card").unwrap();
        let articles = doc
            .select(&sel)
            .map(|card| {
                let a_sel = Selector::parse("h1 a").unwrap();
                let title_text = card.select(&a_sel).next().unwrap().text().collect::<Vec<_>>().join("");
                let url = card.select(&a_sel).next().unwrap().value().attr("href").unwrap();

                let date_sel = Selector::parse("ul li.p-cardList-cardDate").unwrap();
                let mut date_text = match card.select(&date_sel).next() {
                    Some(x) => x.text().collect::<Vec<_>>().join(""),
                    None => String::default(),
                };
                let reg = regex::Regex::new(r"[\d]{1,2}h ago").unwrap();
                if reg.is_match(&date_text) {
                    date_text = Local::now().format("%b. %d, %Y 00:00:00+09:00").to_string();
                } else {
                    date_text = date_text + " 00:00:00+09:00";
                }
                let date = match DateTime::parse_from_str(&date_text, "%b. %d, %Y %H:%M:%S%z") {
                    Ok(x) => x.with_timezone(&Local),
                    Err(_) => DateTime::<Local>::default(),
                };
                WebArticle::new(
                    self.name(),
                    title_text,
                    "https://www.businessinsider.jp".to_string() + &url,
                    "".to_string(),
                    date.into(),
                )
            })
            .collect::<Vec<WebArticle>>();
        return Ok(articles);
    }

    async fn parse_article(&mut self, url: &str) -> Result<(Html, Text)> {
        let url = Url::parse(url).unwrap();
        let cookies = self.login().await?;
        let response = self.request(url.as_str(), &cookies).await?;
        let doc = scraper::Html::parse_document(response.text().await.unwrap().as_str());
        let sel = Selector::parse("article div.p-post-content").unwrap();
        match doc.select(&sel).next() {
            Some(elem) => {
                let text = elem.text().collect::<Vec<_>>().join("\n");
                let html = elem.html().to_string();
                return Ok((self.trim_text(&html), self.trim_text(&text)));
            }
            None => {
                return Err(Error::msg("Failed to parse article text"));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_business_insider_science() {
        let mut site = BusinessInsiderScience::default();
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
