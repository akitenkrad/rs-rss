use adapter::database::ConnectionPool;
use adapter::repository::{
    health::HealthCheckRepositoryImpl,
    web_article::{WebArticleRepositoryImpl, WebSiteRepositoryImpl},
};
use kernel::repository::{
    health::HealthCheckRepository,
    web_article::{WebArticleRepository, WebSiteRepository},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppRegistryImpl {
    health_check_repository: Arc<dyn HealthCheckRepository>,
    web_article_repository: Arc<dyn WebArticleRepository>,
    web_site_repository: Arc<dyn WebSiteRepository>,
}

impl AppRegistryImpl {
    pub fn new(db: ConnectionPool) -> Self {
        let health_check_repository = Arc::new(HealthCheckRepositoryImpl::new(db.clone()));
        let web_article_repository = Arc::new(WebArticleRepositoryImpl::new(db.clone()));
        let web_site_repository = Arc::new(WebSiteRepositoryImpl::new(db.clone()));
        Self {
            health_check_repository,
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

#[mockall::automock]
pub trait AppRegistryExt {
    fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository>;
    fn web_article_repository(&self) -> Arc<dyn WebArticleRepository>;
    fn web_site_repository(&self) -> Arc<dyn WebSiteRepository>;
}

impl AppRegistryExt for AppRegistryImpl {
    fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository> {
        self.health_check_repository.clone()
    }
    fn web_article_repository(&self) -> Arc<dyn WebArticleRepository> {
        self.web_article_repository.clone()
    }
    fn web_site_repository(&self) -> Arc<dyn WebSiteRepository> {
        self.web_site_repository.clone()
    }
}

pub type AppRegistry = Arc<dyn AppRegistryExt + Send + Sync + 'static>;
