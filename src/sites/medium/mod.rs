use crate::sites::{Category, Html, Site, Text, WebArticle};
use scraper::Selector;
pub struct Medium {
    target: String,
    tag: String,
}

#[cfg(test)]
mod tests;

impl Medium {
    pub fn new(target: &str, tag: &str) -> Self {
        return Medium {
            target: target.to_string(),
            tag: tag.to_string(),
        };
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
    async fn get_articles(&self) -> Result<Vec<WebArticle>, String> {
        let url = format!("https://medium.com/tag/{}/archive", self.tag);
        let body = self.request(&url.to_string()).await;

        let mut articles: Vec<WebArticle> = Vec::new();
        // parse html
        let doc = scraper::Html::parse_document(&body);
        let sel = Selector::parse("article").unwrap();
        for (_i, article) in doc.select(&sel).enumerate() {
            let title_sel = Selector::parse("a h2").unwrap();
            let title_text = article
                .select(&title_sel)
                .next()
                .unwrap()
                .text()
                .collect::<Vec<_>>()
                .join("");
            let mut url = "https://medium.com".to_string();
            let a_sel = Selector::parse("div a").unwrap();
            url.push_str(
                article
                    .select(&a_sel)
                    .next()
                    .unwrap()
                    .value()
                    .attr("href")
                    .unwrap(),
            );
            let date_sel = Selector::parse("span").unwrap();

            match article.select(&date_sel).next() {
                Some(x) => {
                    let _text = x
                        .text()
                        .collect::<Vec<_>>()
                        .join("")
                        .trim()
                        .to_string()
                        .to_lowercase();
                    if !(_text.contains("just now")
                        || _text.contains("h ago")
                        || _text.contains("m ago"))
                    {
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
            let article = WebArticle {
                site: self.name(),
                title: title_text,
                url: url.to_string(),
                description: desc_text,
                timestamp: date.into(),
            };
            articles.push(article);
        }
        return Ok(articles);
    }

    async fn get_article_text(&self, url: &String) -> Result<(Html, Text), String> {
        let body = self.request(url).await;
        let doc = scraper::Html::parse_document(&body);
        let sel = Selector::parse("article").unwrap();
        let text = doc.select(&sel).next().unwrap().text().collect();
        let html = doc.select(&sel).next().unwrap().inner_html();
        return Ok((self.trim_text(&html), self.trim_text(&text)));
    }
}
