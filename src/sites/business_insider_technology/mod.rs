use crate::sites::{Category, Site, WebArticle};
use chrono::{DateTime, Local};
use scraper::Selector;
pub struct BusinessInsiderTechnology {}

#[cfg(test)]
mod tests;

impl Site for BusinessInsiderTechnology {
    fn name(&self) -> String {
        return "Business Insider Technology".to_string();
    }
    fn category(&self) -> Category {
        return Category::News;
    }
    async fn get_articles(&self) -> Result<Vec<WebArticle>, String> {
        let url = "https://www.businessinsider.jp/tech-article/";
        let body = self.request(&url.to_string()).await;
        let mut articles: Vec<WebArticle> = Vec::new();

        // parse html
        let doc = scraper::Html::parse_document(&body);
        let sel =
            Selector::parse("#mainContent div.p-cardList-content div.p-cardList-card").unwrap();
        for (_i, card) in doc.select(&sel).enumerate() {
            let a_sel = Selector::parse("h1 a").unwrap();
            let title_text = card
                .select(&a_sel)
                .next()
                .unwrap()
                .text()
                .collect::<Vec<_>>()
                .join("");
            let url = card
                .select(&a_sel)
                .next()
                .unwrap()
                .value()
                .attr("href")
                .unwrap();

            let date_sel = Selector::parse("ul li.p-cardList-cardDate").unwrap();
            let mut date_text = match card.select(&date_sel).next() {
                Some(x) => x.text().collect::<Vec<_>>().join(""),
                None => continue,
            };
            let reg = regex::Regex::new(r"[\d]{1,2}h ago").unwrap();
            if reg.is_match(&date_text) {
                date_text = Local::now().format("%b. %d, %Y 00:00:00+09:00").to_string();
            } else {
                date_text = date_text + " 00:00:00+09:00";
            }
            let date = match DateTime::parse_from_str(&date_text, "%b. %d, %Y %H:%M:%S%z") {
                Ok(x) => x,
                Err(e) => {
                    println!("Got ERROR {}: {}", e, date_text);
                    continue;
                }
            };

            let article = WebArticle {
                title: title_text,
                url: "https://www.businessinsider.jp".to_string() + &url,
                text: "".to_string(),
                timestamp: date.into(),
            };
            articles.push(article);
        }
        return Ok(articles);
    }

    async fn get_article_text(&self, url: &String) -> Result<String, String> {
        let body = self.request(url).await;
        let doc = scraper::Html::parse_document(&body);
        let sel = Selector::parse("article div.p-post-content").unwrap();
        let text = doc.select(&sel).next().unwrap().text().collect();
        return Ok(self.trim_text(&text));
    }
}
