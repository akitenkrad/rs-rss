use crate::sites::{Site, WebArticle};
use chrono::DateTime;
use scraper::Selector;
pub struct Supership {}

#[cfg(test)]
mod tests;

impl Site for Supership {
    fn name(&self) -> String {
        return "Supership".to_string();
    }
    async fn get_articles(&self) -> Vec<WebArticle> {
        let url = "https://supership.jp/news/";
        let html = reqwest::get(url).await.unwrap().text().await.unwrap();
        let mut articles: Vec<WebArticle> = Vec::new();

        // parse html
        let doc = scraper::Html::parse_document(&html);
        let sel =
            Selector::parse("main article ul.p-magazine__archive li.p-magazine__card").unwrap();
        for (_i, li) in doc.select(&sel).enumerate() {
            let title_sel = Selector::parse("p.p-magazine__card_title").unwrap();
            let title_text = li
                .select(&title_sel)
                .next()
                .unwrap()
                .text()
                .collect::<Vec<_>>()
                .join("");
            let url_sel = Selector::parse("a").unwrap();
            let url = li
                .select(&url_sel)
                .next()
                .unwrap()
                .value()
                .attr("href")
                .unwrap()
                .to_string();
            let pubdate_sel = Selector::parse("time.p-magazine__card_time").unwrap();
            let publish_date_text = li
                .select(&pubdate_sel)
                .next()
                .unwrap()
                .text()
                .collect::<Vec<_>>()
                .join("")
                + " 00:00:00+09:00";
            let publish_date =
                match DateTime::parse_from_str(&publish_date_text, "%Y.%m.%d %H:%M:%S%z") {
                    Ok(x) => x,
                    Err(e) => {
                        println!("Got ERROR {}: {}", e, publish_date_text);
                        continue;
                    }
                };
            let article = WebArticle {
                title: title_text,
                url: url,
                text: "".to_string(),
                timestamp: publish_date.into(),
            };
            articles.push(article);
        }
        return articles;
    }

    async fn get_article_text(&self, url: &String) -> String {
        let html = reqwest::get(url).await.unwrap().text().await.unwrap();
        let doc = scraper::Html::parse_document(&html);
        let sel = Selector::parse("main article div.c-grid__block--content").unwrap();
        let text = doc.select(&sel).next().unwrap().text().collect();
        return self.trim_text(&text);
    }
}
