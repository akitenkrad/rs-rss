use crate::sites::{Category, Cookie, Html, Site, Text, WebArticle};
use anyhow::Result;
use request::Url;
use scraper::Selector;

const URL: &str = "https://medium.com/tag/{}/archive";

#[derive(Debug, Clone)]
pub struct Medium {
    target: String,
    tag: String,
    url: Url,
}

impl Medium {
    pub fn new(target: &str, tag: &str) -> Self {
        return Medium {
            target: target.to_string(),
            tag: tag.to_string(),
            url: Url::parse(URL.replace("{}", tag).as_str()).unwrap(),
        };
    }
    pub fn get_url(&self) -> String {
        return URL.replace("{}", &self.tag);
    }
}

impl Default for Medium {
    fn default() -> Self {
        Medium::new("AI", "ai")
    }
}

#[async_trait::async_trait]
impl Site for Medium {
    fn name(&self) -> String {
        return format!("Medium {}", self.target).to_string();
    }
    fn category(&self) -> super::Category {
        return Category::News;
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
        let mut articles: Vec<WebArticle> = Vec::new();
        // parse html
        let doc = scraper::Html::parse_document(response.text().await?.as_str());
        let sel = Selector::parse("article").unwrap();
        for article in doc.select(&sel) {
            let title_sel = Selector::parse("a h2").unwrap();
            let title_text = article.select(&title_sel).next().unwrap().text().collect::<Vec<_>>().join("");
            let mut url = Url::parse("https://medium.com").unwrap();
            let a_sel = Selector::parse("div a").unwrap();
            let href = article.select(&a_sel).next().unwrap().value().attr("href").unwrap();
            if href.contains("https://") {
                url = Url::parse(href).unwrap();
            } else {
                url.set_path(href);
            }
            let date_sel = Selector::parse("span").unwrap();

            match article.select(&date_sel).next() {
                Some(x) => {
                    let _text = x.text().collect::<Vec<_>>().join("").trim().to_string().to_lowercase();
                    if !(_text.contains("just now") || _text.contains("h ago") || _text.contains("m ago")) {
                        println!("{} is not recent", _text);
                        continue;
                    }
                }
                None => {
                    println!("No date found");
                    continue;
                }
            };
            let date = chrono::Local::now();
            let desc_sel = Selector::parse("a h3").unwrap();
            let desc_text = match article.select(&desc_sel).next() {
                Some(x) => x.text().collect::<Vec<_>>().join(""),
                None => "".to_string(),
            };
            let article = WebArticle::new(self.name(), title_text, url.to_string(), desc_text, date.into());
            articles.push(article);
        }
        return Ok(articles);
    }

    async fn parse_article(&mut self, url: &str) -> Result<(Html, Text)> {
        let url = Url::parse(url)?;
        let cookies = self.login().await?;
        let response = self.request(url.as_str(), &cookies).await?;
        let doc = scraper::Html::parse_document(response.text().await?.as_str());
        let sel = match Selector::parse("article") {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!("Failed to parse selector: {}", e));
            }
        };
        let text = doc.select(&sel).next().unwrap().text().collect::<Vec<_>>().join("\n");
        let html = doc.select(&sel).next().unwrap().inner_html();
        return Ok((self.trim_text(&html), self.trim_text(&text)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_medium_artificial_intelligence() {
        let mut site = Medium::new("Artificial Intelligence", "artificial-intelligence");
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

    #[tokio::test]
    async fn test_medium_ai() {
        let mut site = Medium {
            target: "AI".to_string(),
            tag: "ai".to_string(),
            url: Url::parse(URL.replace("{}", "ai").as_str()).unwrap(),
        };
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
