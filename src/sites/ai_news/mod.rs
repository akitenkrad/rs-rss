use crate::sites::{Category, Html, Site, Text, WebArticle};
use chrono::Local;
use scraper::Selector;
pub struct AINews {}

#[cfg(test)]
mod tests;

#[async_trait::async_trait]
impl Site for AINews {
    fn name(&self) -> String {
        return "AI News".to_string();
    }
    fn category(&self) -> super::Category {
        return Category::News;
    }
    async fn get_articles(&self) -> Result<Vec<WebArticle>, String> {
        let url = "https://ai-news.dev/".parse().unwrap();
        let body = self.request(&url).await;

        let mut articles: Vec<WebArticle> = Vec::new();
        // parse html
        let doc = scraper::Html::parse_document(&body);
        let sel = Selector::parse("div.chakra-container a.chakra-link").unwrap();
        for (_i, article) in doc.select(&sel).enumerate() {
            let title_text = article.text().collect::<Vec<_>>().join("");
            let url = article.value().attr("href").unwrap();

            let date_sel = Selector::parse("div.chakra-card__footer").unwrap();
            let mut date_text = match article.select(&date_sel).next() {
                Some(x) => x.text().collect::<Vec<_>>().join(""),
                None => continue,
            };

            if !(date_text.contains("時間前") || date_text.contains("最近")) {
                continue;
            }
            let date = Local::now();

            let desc_sel = Selector::parse("div.chakra-card__body").unwrap();
            let desc_text = match article.select(&desc_sel).next() {
                Some(x) => x.text().collect::<Vec<_>>().join(""),
                None => "".to_string(),
            };

            date_text.push_str("+09:00");
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
        let sel = Selector::parse("body").unwrap();
        let text = doc.select(&sel).next().unwrap().text().collect();
        let html = doc.select(&sel).next().unwrap().inner_html();
        return Ok((self.trim_text(&html), self.trim_text(&text)));
    }
}
