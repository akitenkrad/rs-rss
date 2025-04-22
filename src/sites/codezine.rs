use crate::sites::{Category, Cookie, Html, Site, Text, WebArticle};
use anyhow::{Error, Result};
use chrono::{DateTime, Local};
use request::Url;
use scraper::Selector;

const URL: &str = "https://codezine.jp/news";

#[derive(Debug, Clone)]
pub struct CodeZine {
    url: Url,
}

impl CodeZine {
    pub fn new() -> Self {
        let url = Url::parse(URL).unwrap();
        CodeZine { url }
    }
}

impl Default for CodeZine {
    fn default() -> Self {
        CodeZine::new()
    }
}

#[async_trait::async_trait]
impl Site for CodeZine {
    fn name(&self) -> String {
        return "CodeZine".to_string();
    }
    fn category(&self) -> Category {
        return Category::News;
    }
    fn domain(&self) -> String {
        return self.url.domain().unwrap().to_string();
    }
    async fn login(&mut self) -> Result<Cookie> {
        return Ok(Cookie::default());
    }
    async fn get_articles(&mut self) -> Result<Vec<WebArticle>> {
        let cookies = self.login().await?;
        let response = self.request(self.url.as_str(), &cookies).await?;

        // parse html
        let doc = scraper::Html::parse_document(response.text().await?.as_str());
        let sel = Selector::parse("ul.c-articleindex_list").unwrap();
        let mut articles = Vec::new();
        for ul in doc.select(&sel) {
            let sel = Selector::parse("li.c-articleindex_listitem").unwrap();
            for item in ul.select(&sel) {
                // title, url
                let title_sel = Selector::parse("p.c-articleindex_item_heading a").unwrap();
                let title = item.select(&title_sel).next().unwrap();
                let tilte_text = title.text().collect::<Vec<_>>().join("");
                let url = title.value().attr("href").unwrap().to_string();

                // date
                let date_sel = Selector::parse("p.c-featureindex_item_date").unwrap();
                let date_text = match item.select(&date_sel).next() {
                    Some(x) => x.text().collect::<Vec<_>>().join(""),
                    None => String::default(),
                };
                let date_text = date_text + " 00:00:00+09:00";
                let date = match DateTime::parse_from_str(&date_text, "%Y/%m/%d %H:%M:%S%z") {
                    Ok(x) => x.with_timezone(&Local),
                    Err(_) => DateTime::<Local>::default(),
                };

                articles.push(WebArticle::new(
                    self.name(),
                    tilte_text,
                    "https://codezine.jp".to_string() + &url,
                    "".to_string(),
                    date.into(),
                ));
            }
        }
        return Ok(articles);
    }
    async fn parse_article(&mut self, url: &str) -> Result<(Html, Text)> {
        let url = Url::parse(url).unwrap();
        let cookies = self.login().await?;
        let response = self.request(url.as_str(), &cookies).await?;
        let doc = scraper::Html::parse_document(response.text().await?.as_str());
        let sel = Selector::parse("main article div.detailBlock").unwrap();
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
    async fn test_codezine() {
        let mut site = CodeZine::default();
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
