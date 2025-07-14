use axum::http::StatusCode;
use chrono::NaiveDate;
use derive_new::new;
use garde::Validate;
use kernel::models::{
    list::PaginatedList,
    web_article::{WebArticle, WebArticleListOptions, WebSite, WebSiteListOptions},
};
use serde::{Deserialize, Serialize};
use shared::id::{WebArticleId, WebSiteId};

const DEFAULT_LIMIT: i64 = 20;
const fn default_limit() -> i64 {
    DEFAULT_LIMIT
}

#[derive(Debug, Clone, Deserialize, Serialize, new)]
#[serde(rename_all = "camelCase")]
pub struct WebSiteResponse {
    pub site_id: WebSiteId,
    pub name: String,
    pub url: String,
}
impl From<WebSite> for WebSiteResponse {
    fn from(site: WebSite) -> Self {
        let WebSite { site_id, name, url } = site;
        Self { site_id, name, url }
    }
}

#[derive(Debug, Clone, Deserialize, new, Validate)]
pub struct WebSiteListQuery {
    #[garde(range(min = 0))]
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[garde(range(min = 0))]
    #[serde(default)]
    pub offset: i64,
}

impl From<WebSiteListQuery> for WebSiteListOptions {
    fn from(query: WebSiteListQuery) -> Self {
        let WebSiteListQuery { limit, offset } = query;
        Self { limit, offset }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PaginatedWebSiteResponse {
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub items: Vec<WebSiteResponse>,
    pub status_code: usize,
}
impl From<PaginatedList<WebSite>> for PaginatedWebSiteResponse {
    fn from(paginated_list: PaginatedList<WebSite>) -> Self {
        let PaginatedList {
            total,
            limit,
            offset,
            items,
        } = paginated_list;
        Self {
            total,
            limit,
            offset,
            items: items.into_iter().map(|site| site.into()).collect(),
            status_code: StatusCode::OK.as_u16() as usize,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebArticleResponse {
    pub site_id: WebSiteId,
    pub site_name: String,
    pub site_url: String,
    pub article_id: WebArticleId,
    pub title: String,
    pub description: String,
    pub url: String,
    pub text: String,
    pub html: String,
    pub timestamp: NaiveDate,
    pub summary: String,
    pub is_new_technology_related: bool,
    pub is_new_academic_paper_related: bool,
    pub is_ai_related: bool,
    pub is_it_related: bool,
    pub is_new_product_related: bool,
    pub is_security_related: bool,
}

impl From<WebArticle> for WebArticleResponse {
    fn from(article: WebArticle) -> Self {
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
            is_new_academic_paper_related,
            is_ai_related,
            is_it_related,
            is_new_product_related,
            is_security_related,
        } = article;
        Self {
            site_id: site.site_id,
            site_name: site.name,
            site_url: site.url,
            article_id,
            title,
            description,
            url,
            text,
            html,
            timestamp,
            summary,
            is_new_technology_related,
            is_new_academic_paper_related,
            is_ai_related,
            is_it_related,
            is_new_product_related,
            is_security_related,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct WebArticleListQuery {
    #[garde(range(min = 0))]
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[garde(range(min = 0))]
    #[serde(default)]
    pub offset: i64,
}

impl From<WebArticleListQuery> for WebArticleListOptions {
    fn from(query: WebArticleListQuery) -> Self {
        let WebArticleListQuery { limit, offset } = query;
        Self { limit, offset }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PaginatedWebArticleResponse {
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub items: Vec<WebArticleResponse>,
    pub status_code: usize,
}

impl From<PaginatedList<WebArticle>> for PaginatedWebArticleResponse {
    fn from(paginated_list: PaginatedList<WebArticle>) -> Self {
        let PaginatedList {
            total,
            limit,
            offset,
            items,
        } = paginated_list;
        Self {
            total,
            limit,
            offset,
            items: items.into_iter().map(|article| article.into()).collect(),
            status_code: StatusCode::OK.as_u16() as usize,
        }
    }
}
