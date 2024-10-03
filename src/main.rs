use chrono::DateTime;
use scraper::Selector;
// use std::{fs::File, io::Write};
// use tokio::time;

#[tokio::main]
async fn main() {
    let url = "https://codezine.jp/news";
    let html = reqwest::get(url).await.unwrap().text().await.unwrap();

    let doc = scraper::Html::parse_document(&html);
    let sel = Selector::parse("ul.c-articleindex_list").unwrap();
    for (_i, ul) in doc.select(&sel).enumerate() {
        let sel = Selector::parse("li.c-articleindex_listitem").unwrap();
        for (_j, item) in ul.select(&sel).enumerate() {
            // title, url
            let title_sel = Selector::parse("p.c-articleindex_item_heading").unwrap();
            let title = item.select(&title_sel).next().unwrap();
            let a_sel = Selector::parse("a").unwrap();
            let a = title.select(&a_sel).next().unwrap();
            let tilte_text = a.text().collect::<Vec<_>>().join("");
            let url = a.value().attr("href").unwrap().to_string();

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
                    println!("{}: {}", e, date_text);
                    continue;
                }
            };
            println!("{}: {} {}", tilte_text, url, date);
        }
    }
}
