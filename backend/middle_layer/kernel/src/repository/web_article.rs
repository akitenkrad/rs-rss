use crate::models::{
    list::PaginatedList,
    web_article::{Status, WebArticle, WebArticleFilteredOptions, WebArticleListOptions, WebSite, WebSiteListOptions},
};
use async_trait::async_trait;
use shared::id::WebArticleId;
use shared::{errors::AppResult, id::WebSiteId};

#[async_trait]
pub trait WebSiteRepository: Send + Sync {
    async fn create_web_site(&self, web_site: WebSite) -> AppResult<WebSite>;
    async fn update_web_site(&self, web_site: WebSite) -> AppResult<()>;
    async fn delete_web_site(&self, id: WebSiteId) -> AppResult<()>;
    async fn select_web_site_by_id(&self, id: WebSiteId) -> AppResult<WebSite>;
    async fn select_web_site_by_name(&self, name: &str) -> AppResult<WebSite>;
    async fn select_or_create_web_site(&self, name: &str, url: &str) -> AppResult<WebSite>;
    async fn select_all_web_sites_paginated(&self, options: WebSiteListOptions) -> AppResult<PaginatedList<WebSite>>;
}

#[async_trait]
pub trait WebArticleRepository: Send + Sync {
    async fn create_web_article(&self, web_article: &mut WebArticle) -> AppResult<WebArticle>;
    async fn update_web_article(&self, web_article: WebArticle) -> AppResult<()>;
    async fn update_web_article_state(&self, article_id: WebArticleId, new_status: Status) -> AppResult<WebArticle>;
    async fn delete_web_article(&self, id: WebArticleId) -> AppResult<()>;
    async fn select_todays_web_articles(&self) -> AppResult<Vec<WebArticle>>;
    async fn select_web_article_by_id(&self, id: WebArticleId) -> AppResult<WebArticle>;
    async fn select_web_articles_by_keyword(&self, keyword: &str) -> AppResult<Vec<WebArticle>>;
    async fn select_web_article_by_url(&self, url: &str) -> AppResult<WebArticle>;
    async fn select_or_create_web_article(&self, web_article: WebArticle) -> AppResult<WebArticle>;
    async fn select_all_web_articles(&self) -> AppResult<Vec<WebArticle>>;
    async fn select_paginated_web_articles(
        &self,
        options: WebArticleListOptions,
    ) -> AppResult<PaginatedList<WebArticle>>;
    async fn select_filtered_web_articles(
        &self,
        options: WebArticleFilteredOptions,
    ) -> AppResult<PaginatedList<WebArticle>>;
}
