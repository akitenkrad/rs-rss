use crate::sites::{Site, WebArticle};
use chrono::DateTime;
use feed_parser::Rss2;
use scraper::Selector;
pub struct Gigazin {}

#[cfg(test)]
mod tests;

impl Site for Gigazin {
    async fn get_articles(&self) -> Vec<WebArticle> {}
}
