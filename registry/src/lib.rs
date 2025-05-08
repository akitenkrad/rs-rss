use adapter::database::ConnectionPool;
use adapter::repository::web_article::{WebArticleRepositoryImpl, WebSiteRepositoryImpl};
use kernel::repository::web_article::{WebArticleRepository, WebSiteRepository};
use std::sync::Arc;

#[derive(Clone)]
pub struct Registry {
    web_article_repository: Arc<dyn WebArticleRepository>,
    web_site_repository: Arc<dyn WebSiteRepository>,
}

impl Registry {
    pub fn new(db: ConnectionPool) -> Self {
        let web_article_repository = Arc::new(WebArticleRepositoryImpl::new(db.clone()));
        let web_site_repository = Arc::new(WebSiteRepositoryImpl::new(db));
        Self {
            web_article_repository,
            web_site_repository,
        }
    }

    pub fn web_article_repository(&self) -> Arc<dyn WebArticleRepository> {
        self.web_article_repository.clone()
    }
    pub fn web_site_repository(&self) -> Arc<dyn WebSiteRepository> {
        self.web_site_repository.clone()
    }
}
