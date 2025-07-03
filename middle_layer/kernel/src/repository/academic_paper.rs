use crate::models::{
    academic_paper::{AcademicPaper, AcademicPaperListOptions, Author, AuthorListOptions, Journal, Task},
    list::PaginatedList,
};
use async_trait::async_trait;
use shared::errors::AppResult;

#[async_trait]
pub trait AuthorRepository: Send + Sync {
    async fn select_author_by_id(&self, id: &str) -> AppResult<Author>;
    async fn select_author_by_ssid(&self, ss_id: &str) -> AppResult<Author>;
    async fn select_all_authors(&self) -> AppResult<Vec<Author>>;
    async fn select_all_authors_paginated(&self, options: AuthorListOptions) -> AppResult<PaginatedList<Author>>;
    async fn create_author(&self, author: Author) -> AppResult<Author>;
    async fn delete_author(&self, id: &str) -> AppResult<()>;
}

#[async_trait]
pub trait JournalRepository: Send + Sync {
    async fn select_journal_by_id(&self, id: &str) -> AppResult<Journal>;
    async fn select_journal_by_name(&self, name: &str) -> AppResult<Journal>;
    async fn select_all_journals(&self) -> AppResult<Vec<Journal>>;
    async fn create_journal(&self, journal: Journal) -> AppResult<Journal>;
    async fn delete_journal(&self, id: &str) -> AppResult<()>;
}

#[async_trait]
pub trait TaskRepository: Send + Sync {
    async fn select_task_by_id(&self, id: &str) -> AppResult<Task>;
    async fn select_task_by_name(&self, name: &str) -> AppResult<Task>;
    async fn select_all_tasks(&self) -> AppResult<Vec<Task>>;
    async fn create_task(&self, task: Task) -> AppResult<Task>;
    async fn delete_task(&self, id: &str) -> AppResult<()>;
}

#[async_trait]
pub trait AcademicPaperRepository: Send + Sync {
    async fn select_todays_articles(&self) -> AppResult<Vec<AcademicPaper>>;
    async fn select_academic_paper_by_arxiv_id(&self, arxiv_id: &str) -> AppResult<AcademicPaper>;
    async fn select_academic_paper_by_ss_id(&self, ss_id: &str) -> AppResult<AcademicPaper>;
    async fn select_academic_paper_by_id(&self, id: &str) -> AppResult<AcademicPaper>;
    async fn select_academic_paper_by_title(&self, title: &str) -> AppResult<Vec<AcademicPaper>>;
    async fn select_all_academic_papers(&self) -> AppResult<Vec<AcademicPaper>>;
    async fn select_academic_papers_by_keyword(&self, keyword: &str) -> AppResult<Vec<AcademicPaper>>;
    async fn select_all_academic_papers_paginated(
        &self,
        options: AcademicPaperListOptions,
    ) -> AppResult<PaginatedList<AcademicPaper>>;
    async fn fill_fields(&self, academic_paper: &mut AcademicPaper) -> AppResult<()>;
    async fn create_academic_paper(&self, academic_paper: AcademicPaper) -> AppResult<AcademicPaper>;
    async fn delete_academic_paper(&self, id: &str) -> AppResult<()>;
}
