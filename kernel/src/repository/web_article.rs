use crate::models::web_article::{WebArticle, WebSite};
use async_trait::async_trait;
use shared::errors::AppResult;

#[async_trait]
pub trait WebSiteRepository: Send + Sync {
    async fn create_web_site(&self, web_site: WebSite) -> AppResult<WebSite>;
    async fn read_web_site_by_id(&self, id: &str) -> AppResult<WebSite>;
    async fn read_web_site_by_name(&self, name: &str) -> AppResult<WebSite>;
    async fn read_or_create_web_site(&self, name: &str, url: &str) -> AppResult<WebSite>;
    async fn read_all_web_sites(&self) -> AppResult<Vec<WebSite>>;
    async fn update_web_site(&self, web_site: WebSite) -> AppResult<()>;
    async fn delete_web_site(&self, id: &str) -> AppResult<()>;
}

#[async_trait]
pub trait WebArticleRepository: Send + Sync {
    async fn create_web_article(&self, web_article: &mut WebArticle) -> AppResult<WebArticle>;
    async fn read_todays_articles(&self) -> AppResult<Vec<WebArticle>>;
    async fn read_web_article_by_id(&self, id: &str) -> AppResult<WebArticle>;
    async fn read_web_articles_by_keyword(&self, keyword: &str) -> AppResult<Vec<WebArticle>>;
    async fn read_web_article_by_url(&self, url: &str) -> AppResult<WebArticle>;
    async fn read_or_create_web_article(&self, web_article: WebArticle) -> AppResult<WebArticle>;
    async fn read_all_articles(&self) -> AppResult<Vec<WebArticle>>;
    async fn update_web_article(&self, web_article: WebArticle) -> AppResult<()>;
    async fn delete_web_article(&self, id: &str) -> AppResult<()>;
}
