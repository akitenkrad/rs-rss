use adapter::database::ConnectionPool;
use adapter::repository::{
    academic_paper::{AcademicPaperRepositoryImpl, AuthorRepositoryImpl, JournalRepositoryImpl, TaskRepositoryImpl},
    health::HealthCheckRepositoryImpl,
    web_article::{WebArticleRepositoryImpl, WebSiteRepositoryImpl},
};
use kernel::repository::{
    academic_paper::{AcademicPaperRepository, AuthorRepository, JournalRepository, TaskRepository},
    health::HealthCheckRepository,
    web_article::{WebArticleRepository, WebSiteRepository},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppRegistryImpl {
    pub db: ConnectionPool,
    health_check_repository: Arc<dyn HealthCheckRepository>,
    web_article_repository: Arc<dyn WebArticleRepository>,
    web_site_repository: Arc<dyn WebSiteRepository>,
    academic_paper_repository: Arc<dyn AcademicPaperRepository>,
    author_repository: Arc<dyn AuthorRepository>,
    journal_repository: Arc<dyn JournalRepository>,
    task_repository: Arc<dyn TaskRepository>,
}

impl AppRegistryImpl {
    pub fn new(db: ConnectionPool) -> Self {
        let health_check_repository = Arc::new(HealthCheckRepositoryImpl::new(db.clone()));
        let web_article_repository = Arc::new(WebArticleRepositoryImpl::new(db.clone()));
        let web_site_repository = Arc::new(WebSiteRepositoryImpl::new(db.clone()));
        let academic_paper_repository = Arc::new(AcademicPaperRepositoryImpl::new(db.clone()));
        let author_repository = Arc::new(AuthorRepositoryImpl::new(db.clone()));
        let journal_repository = Arc::new(JournalRepositoryImpl::new(db.clone()));
        let task_repository = Arc::new(TaskRepositoryImpl::new(db.clone()));
        Self {
            db,
            health_check_repository,
            web_article_repository,
            web_site_repository,
            academic_paper_repository,
            author_repository,
            journal_repository,
            task_repository,
        }
    }

    pub fn web_article_repository(&self) -> Arc<dyn WebArticleRepository> {
        self.web_article_repository.clone()
    }
    pub fn web_site_repository(&self) -> Arc<dyn WebSiteRepository> {
        self.web_site_repository.clone()
    }
    pub fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository> {
        self.health_check_repository.clone()
    }
    pub fn academic_paper_repository(&self) -> Arc<dyn AcademicPaperRepository> {
        self.academic_paper_repository.clone()
    }
    pub fn author_repository(&self) -> Arc<dyn AuthorRepository> {
        self.author_repository.clone()
    }
    pub fn journal_repository(&self) -> Arc<dyn JournalRepository> {
        self.journal_repository.clone()
    }
    pub fn task_repository(&self) -> Arc<dyn TaskRepository> {
        self.task_repository.clone()
    }
}

#[mockall::automock]
pub trait AppRegistryExt {
    fn db(&self) -> &ConnectionPool;
    fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository>;
    fn web_article_repository(&self) -> Arc<dyn WebArticleRepository>;
    fn web_site_repository(&self) -> Arc<dyn WebSiteRepository>;
    fn academic_paper_repository(&self) -> Arc<dyn AcademicPaperRepository>;
    fn author_repository(&self) -> Arc<dyn AuthorRepository>;
    fn journal_repository(&self) -> Arc<dyn JournalRepository>;
    fn task_repository(&self) -> Arc<dyn TaskRepository>;
}

impl AppRegistryExt for AppRegistryImpl {
    fn db(&self) -> &ConnectionPool {
        &self.db
    }
    fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository> {
        self.health_check_repository.clone()
    }
    fn web_article_repository(&self) -> Arc<dyn WebArticleRepository> {
        self.web_article_repository.clone()
    }
    fn web_site_repository(&self) -> Arc<dyn WebSiteRepository> {
        self.web_site_repository.clone()
    }
    fn academic_paper_repository(&self) -> Arc<dyn AcademicPaperRepository> {
        self.academic_paper_repository.clone()
    }
    fn author_repository(&self) -> Arc<dyn AuthorRepository> {
        self.author_repository.clone()
    }
    fn journal_repository(&self) -> Arc<dyn JournalRepository> {
        self.journal_repository.clone()
    }
    fn task_repository(&self) -> Arc<dyn TaskRepository> {
        self.task_repository.clone()
    }
}

pub type AppRegistry = Arc<dyn AppRegistryExt + Send + Sync + 'static>;
