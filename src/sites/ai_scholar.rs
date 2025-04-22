use crate::sites::{Category, Cookie, Html, Site, Text, WebArticle};
use anyhow::{Error, Result};
use chrono::{DateTime, Local};
use request::Url;
use scraper::Selector;

const URL: &str = "https://ai-scholar.tech/";

#[derive(Debug, Clone)]
pub struct AIScholar {
    url: Url,
}

impl AIScholar {
    pub fn new() -> Self {
        Self {
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
impl Site for AIScholar {
    fn name(&self) -> String {
        return "AI Scholar".to_string();
    }
    fn category(&self) -> super::Category {
        return Category::News;
    }
    fn domain(&self) -> String {
        return self.url.domain().unwrap().to_string();
    }
    async fn login(&mut self) -> Result<Cookie> {
        // No login required
        return Ok(String::new());
    }
    async fn get_articles(&mut self) -> Result<Vec<WebArticle>> {
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
                let title_text = article.select(&a_sel).next().unwrap().text().collect::<Vec<_>>().join("");
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
                WebArticle::new(self.name(), title_text, url.to_string(), desc_text, date)
            })
            .collect::<Vec<WebArticle>>();
        return Ok(articles);
    }

    async fn parse_article(&mut self, url: &str) -> Result<(Html, Text)> {
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
                return Err(Error::msg("Failed to parse article text"));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ai_scholar() {
        let mut site = AIScholar::default();
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
