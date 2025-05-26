use derive_new::new;
use garde::Validate;
use kernel::models::{
    list::PaginatedList,
    web_article::{WebArticle, WebSite, WebSiteListOptions},
};
use serde::{Deserialize, Serialize};
use shared::id::WebSiteId;

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
        }
    }
}
