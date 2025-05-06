use chrono::NaiveDate;
use derive_new::new;
use shared::id::{WebArticleId, WebSiteId};

#[derive(Debug, Clone, new)]
pub struct WebSite {
    pub site_id: WebSiteId,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, new)]
pub struct WebArticle {
    pub site: WebSite,
    pub article_id: WebArticleId,
    pub title: String,
    pub description: String,
    pub url: String,
    pub text: String,
    pub html: String,
    pub timestamp: NaiveDate,
    pub summary: String,
    pub is_new_technology_related: bool,
    pub is_new_product_related: bool,
    pub is_new_paper_related: bool,
    pub is_ai_related: bool,
}
