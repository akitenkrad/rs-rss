use crate::models::{
    academic_paper::{AcademicPaper, AcademicPaperListOptions, Author, AuthorListOptions, Journal, Task},
    list::PaginatedList,
};
use async_trait::async_trait;
use shared::errors::AppResult;
use sqlx::{Postgres as Pg, Transaction as T};

#[async_trait]
pub trait AuthorRepository: Send + Sync {
    async fn select_author_by_id(&self, tx: &mut T<'_, Pg>, id: &str) -> AppResult<Author>;
    async fn select_author_by_ssid(&self, tx: &mut T<'_, Pg>, ss_id: &str) -> AppResult<Author>;
    async fn select_all_authors(&self, tx: &mut T<'_, Pg>) -> AppResult<Vec<Author>>;
    async fn select_all_authors_paginated(
        &self,
        tx: &mut T<'_, Pg>,
        options: AuthorListOptions,
    ) -> AppResult<PaginatedList<Author>>;
    async fn create_author(&self, tx: &mut T<'_, Pg>, author: Author) -> AppResult<Author>;
    async fn delete_author(&self, tx: &mut T<'_, Pg>, id: &str) -> AppResult<()>;
}

#[async_trait]
pub trait JournalRepository: Send + Sync {
    async fn select_journal_by_id(&self, tx: &mut T<'_, Pg>, id: &str) -> AppResult<Journal>;
    async fn select_journal_by_name(&self, tx: &mut T<'_, Pg>, name: &str) -> AppResult<Journal>;
    async fn select_all_journals(&self, tx: &mut T<'_, Pg>) -> AppResult<Vec<Journal>>;
    async fn create_journal(&self, tx: &mut T<'_, Pg>, journal: Journal) -> AppResult<Journal>;
    async fn delete_journal(&self, tx: &mut T<'_, Pg>, id: &str) -> AppResult<()>;
}

#[async_trait]
pub trait TaskRepository: Send + Sync {
    async fn select_task_by_id(&self, tx: &mut T<'_, Pg>, id: &str) -> AppResult<Task>;
    async fn select_task_by_name(&self, tx: &mut T<'_, Pg>, name: &str) -> AppResult<Task>;
    async fn select_all_tasks(&self, tx: &mut T<'_, Pg>) -> AppResult<Vec<Task>>;
    async fn create_task(&self, tx: &mut T<'_, Pg>, task: Task) -> AppResult<Task>;
    async fn delete_task(&self, tx: &mut T<'_, Pg>, id: &str) -> AppResult<()>;
}

#[async_trait]
pub trait AcademicPaperRepository: Send + Sync {
    async fn select_todays_articles(&self, tx: &mut T<'_, Pg>) -> AppResult<Vec<AcademicPaper>>;
    async fn select_academic_paper_by_arxiv_id(&self, tx: &mut T<'_, Pg>, arxiv_id: &str) -> AppResult<AcademicPaper>;
    async fn select_academic_paper_by_ss_id(&self, tx: &mut T<'_, Pg>, ss_id: &str) -> AppResult<AcademicPaper>;
    async fn select_academic_paper_by_id(&self, tx: &mut T<'_, Pg>, id: &str) -> AppResult<AcademicPaper>;
    async fn select_academic_paper_by_title(&self, tx: &mut T<'_, Pg>, title: &str) -> AppResult<Vec<AcademicPaper>>;
    async fn select_all_academic_papers(&self, tx: &mut T<'_, Pg>) -> AppResult<Vec<AcademicPaper>>;
    async fn select_academic_papers_by_keyword(
        &self,
        tx: &mut T<'_, Pg>,
        keyword: &str,
    ) -> AppResult<Vec<AcademicPaper>>;
    async fn select_paginated_academic_papers(
        &self,
        tx: &mut T<'_, Pg>,
        options: AcademicPaperListOptions,
    ) -> AppResult<PaginatedList<AcademicPaper>>;
    async fn fill_fields(&self, tx: &mut T<'_, Pg>, academic_paper: &mut AcademicPaper) -> AppResult<()>;
    async fn create_academic_paper(
        &self,
        tx: &mut T<'_, Pg>,
        academic_paper: AcademicPaper,
    ) -> AppResult<AcademicPaper>;
    async fn update_academic_paper(
        &self,
        tx: &mut T<'_, Pg>,
        academic_paper: AcademicPaper,
    ) -> AppResult<AcademicPaper>;
    async fn delete_academic_paper(&self, tx: &mut T<'_, Pg>, id: &str) -> AppResult<()>;
}
