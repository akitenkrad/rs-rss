use crate::sites::{Category, Site, WebArticle};
use chrono::DateTime;
use scraper::Selector;
pub struct CodeZine {}

#[cfg(test)]
mod tests;

impl Site for CodeZine {
    fn name(&self) -> String {
        return "CodeZine".to_string();
    }
    fn category(&self) -> Category {
        return Category::News;
    }
    async fn get_articles(&self) -> Result<Vec<WebArticle>, String> {
        let url = "https://codezine.jp/news";
        let body = self.request(&url.to_string()).await;
        let mut articles: Vec<WebArticle> = Vec::new();

        // parse html
        let doc = scraper::Html::parse_document(&body);
        let sel = Selector::parse("ul.c-articleindex_list").unwrap();
        for (_i, ul) in doc.select(&sel).enumerate() {
            let sel = Selector::parse("li.c-articleindex_listitem").unwrap();
            for (_j, item) in ul.select(&sel).enumerate() {
                // title, url
                let title_sel = Selector::parse("p.c-articleindex_item_heading a").unwrap();
                let title = item.select(&title_sel).next().unwrap();
                let tilte_text = title.text().collect::<Vec<_>>().join("");
                let url = title.value().attr("href").unwrap().to_string();

                // date
                let date_sel = Selector::parse("p.c-featureindex_item_date").unwrap();
                let date_text = match item.select(&date_sel).next() {
                    Some(x) => x.text().collect::<Vec<_>>().join(""),
                    None => continue,
                };
                let date_text = date_text + " 00:00:00+09:00";
                let date = match DateTime::parse_from_str(&date_text, "%Y/%m/%d %H:%M:%S%z") {
                    Ok(x) => x,
                    Err(e) => {
                        println!("Got ERROR {}: {}", e, date_text);
                        continue;
                    }
                };

                let article = WebArticle {
                    title: tilte_text,
                    url: "https://codezine.jp".to_string() + &url,
                    text: "".to_string(),
                    timestamp: date.into(),
                };
                articles.push(article);
            }
        }
        return Ok(articles);
    }
    async fn get_article_text(&self, url: &String) -> Result<String, String> {
        let body = self.request(url).await;
        let doc = scraper::Html::parse_document(&body);
        let sel = Selector::parse("main article div.detailBlock").unwrap();
        let text = doc.select(&sel).next().unwrap().text().collect();
        return Ok(self.trim_text(&text));
    }
}
