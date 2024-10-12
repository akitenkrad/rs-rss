use crate::sites::{Site, WebArticle};
use chrono::DateTime;
use scraper::Selector;
pub struct StockmarkTechBlog {}

#[cfg(test)]
mod tests;

impl Site for StockmarkTechBlog {
    fn name(&self) -> String {
        return "Stockmark Tech Blog".to_string();
    }
    async fn get_articles(&self) -> Vec<WebArticle> {
        let url = "https://stockmark-tech.hatenablog.com/";
        let html = reqwest::get(url).await.unwrap().text().await.unwrap();
        let mut articles: Vec<WebArticle> = Vec::new();

        // parse html
        let doc = scraper::Html::parse_document(&html);
        let post_selector = Selector::parse("#main").unwrap();
        let posts = doc.select(&post_selector);
        for post in posts {
            let desc_selector =
                Selector::parse("div.archive-entry-body p.entry-description").unwrap();
            let title_selector = Selector::parse("div.archive-entry-header").unwrap();
            let url_selector = Selector::parse("div.archive-entry-header h1 a").unwrap();
            let date_selector =
                Selector::parse("div.archive-entry-header div.archive-date").unwrap();

            let article = WebArticle {
                title: post
                    .select(&title_selector)
                    .next()
                    .unwrap()
                    .text()
                    .collect(),
                url: post
                    .select(&url_selector)
                    .next()
                    .unwrap()
                    .value()
                    .attr("href")
                    .unwrap()
                    .to_string(),
                text: post.select(&desc_selector).next().unwrap().text().collect(),
                timestamp: DateTime::parse_from_str(
                    &format!(
                        "{} 00:00:00+0900",
                        post.select(&date_selector)
                            .next()
                            .unwrap()
                            .text()
                            .collect::<Vec<_>>()
                            .join("")
                    ),
                    "%Y-%m-%d %H:%M:%S%z",
                )
                .unwrap()
                .into(),
            };
            articles.push(article);
        }
        return articles;
    }
    async fn get_article_text(&self, url: &String) -> String {
        let html = reqwest::get(url).await.unwrap().text().await.unwrap();
        let doc = scraper::Html::parse_document(&html);
        let selector = Selector::parse("#main div.entry-inner").unwrap();
        let article = doc.select(&selector).next().unwrap();
        let text = article.text().collect::<Vec<_>>().join("\n");
        return self.trim_text(&text);
    }
}
