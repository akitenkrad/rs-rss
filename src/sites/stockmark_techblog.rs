use crate::sites::{Category, Cookie, Html, Site, Text, WebArticle};
use anyhow::Result;
use chrono::DateTime;
use request::Url;
use scraper::Selector;

const URL: &str = "https://stockmark-tech.hatenablog.com/";

#[derive(Debug, Clone)]
pub struct StockmarkTechBlog {
    url: Url,
}

impl StockmarkTechBlog {
    pub fn new() -> Self {
        Self {
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
impl Site for StockmarkTechBlog {
    fn name(&self) -> String {
        return "Stockmark Tech Blog".to_string();
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
        let doc = scraper::Html::parse_document(response.text().await?.as_str());

        // parse html
        let mut articles: Vec<WebArticle> = Vec::new();
        let post_selector = Selector::parse("#main").unwrap();
        let posts = doc.select(&post_selector);
        for post in posts {
            let desc_selector = Selector::parse("div.archive-entry-body p.entry-description").unwrap();
            let title_selector = Selector::parse("div.archive-entry-header").unwrap();
            let url_selector = Selector::parse("div.archive-entry-header h1 a").unwrap();
            let date_selector = Selector::parse("div.archive-entry-header div.archive-date").unwrap();

            let article = WebArticle::new(
                self.name(),
                post.select(&title_selector).next().unwrap().text().collect(),
                post.select(&url_selector).next().unwrap().value().attr("href").unwrap().to_string(),
                post.select(&desc_selector).next().unwrap().text().collect(),
                DateTime::parse_from_str(
                    &format!(
                        "{} 00:00:00+0900",
                        post.select(&date_selector).next().unwrap().text().collect::<Vec<_>>().join("")
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
    async fn parse_article(&mut self, url: &str) -> Result<(Html, Text)> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_stockmarktechblog() {
        let mut site = StockmarkTechBlog::default();
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
