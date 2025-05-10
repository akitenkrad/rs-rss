use chrono::NaiveDate;
use derive_new::new;
use kernel::models::web_article::{WebArticle, WebSite};
use shared::id::{WebArticleId, WebSiteId};
use sqlx::FromRow;

#[derive(Debug, Clone, new, FromRow)]
pub struct WebSiteRecord {
    pub site_id: WebSiteId,
    pub name: String,
    pub url: String,
}

impl From<WebSite> for WebSiteRecord {
    fn from(web_site: WebSite) -> Self {
        let WebSite { site_id, name, url } = web_site;
        Self { site_id, name, url }
    }
}

impl From<WebSiteRecord> for WebSite {
    fn from(web_site_record: WebSiteRecord) -> Self {
        let WebSiteRecord { site_id, name, url } = web_site_record;
        Self { site_id, name, url }
    }
}

#[derive(Debug, Clone, new, FromRow)]
pub struct WebArticleRecord {
    pub site_id: WebSiteId,
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
    pub is_new_academic_paper_related: bool,
    pub is_ai_related: bool,
    pub is_security_related: bool,
    pub is_it_related: bool,
}

impl From<WebArticle> for WebArticleRecord {
    fn from(web_article: WebArticle) -> Self {
        let WebArticle {
            site,
            article_id,
            title,
            description,
            url,
            text,
            html,
            timestamp,
            summary,
            is_new_technology_related,
            is_new_product_related,
            is_new_academic_paper_related,
            is_ai_related,
            is_security_related,
            is_it_related,
        } = web_article;
        Self {
            site_id: site.site_id,
            article_id,
            title,
            description,
            url,
            text,
            html,
            timestamp,
            summary,
            is_new_technology_related,
            is_new_product_related,
            is_new_academic_paper_related,
            is_ai_related,
            is_security_related,
            is_it_related,
        }
    }
}

impl From<WebArticleRecord> for WebArticle {
    fn from(web_article_record: WebArticleRecord) -> Self {
        let WebArticleRecord {
            site_id,
            article_id,
            title,
            description,
            url,
            text,
            html,
            timestamp,
            summary,
            is_new_technology_related,
            is_new_product_related,
            is_new_academic_paper_related: is_new_paper_related,
            is_ai_related,
            is_security_related,
            is_it_related,
        } = web_article_record;
        Self {
            site: WebSite {
                site_id,
                name: String::new(),
                url: String::new(),
            },
            article_id,
            title,
            description,
            url,
            text,
            html,
            timestamp,
            summary,
            is_new_technology_related,
            is_new_product_related,
            is_new_academic_paper_related: is_new_paper_related,
            is_ai_related,
            is_security_related,
            is_it_related,
        }
    }
}
