use crate::models::academic_paper::{AcademicPaper, Author, Journal, Task};
use async_trait::async_trait;
use shared::error::AppResult;

#[async_trait]
pub trait AuthorRepository: Send + Sync {
    async fn get_author_by_id(&self, id: &str) -> AppResult<Author>;
    async fn get_all_authors(&self) -> AppResult<Vec<Author>>;
    async fn save_author(&self, author: Author) -> AppResult<()>;
    async fn delete_author(&self, id: &str) -> AppResult<()>;
}

#[async_trait]
pub trait JournalRepository: Send + Sync {
    async fn get_journal_by_id(&self, id: &str) -> AppResult<Journal>;
    async fn get_all_journals(&self) -> AppResult<Vec<Journal>>;
    async fn save_journal(&self, journal: Journal) -> AppResult<()>;
    async fn delete_journal(&self, id: &str) -> AppResult<()>;
}

#[async_trait]
pub trait TaskRepository: Send + Sync {
    async fn get_task_by_id(&self, id: &str) -> AppResult<Task>;
    async fn get_all_tasks(&self) -> AppResult<Vec<Task>>;
    async fn save_task(&self, task: Task) -> AppResult<()>;
    async fn delete_task(&self, id: &str) -> AppResult<()>;
}

#[async_trait]
pub trait AcademicPaperRepository: Send + Sync {
    async fn get_todays_articles(&self) -> AppResult<Vec<AcademicPaper>>;
    async fn get_academic_paper_by_arxiv_id(&self, arxiv_id: &str) -> AppResult<AcademicPaper>;
    async fn get_academic_paper_by_ss_id(&self, ss_id: &str) -> AppResult<AcademicPaper>;
    async fn get_academic_paper_by_id(&self, id: &str) -> AppResult<AcademicPaper>;
    async fn get_all_academic_papers(&self) -> AppResult<Vec<AcademicPaper>>;
    async fn get_academic_papers_by_keyword(&self, keyword: &str) -> AppResult<Vec<AcademicPaper>>;
    async fn save_academic_paper(&self, academic_paper: AcademicPaper) -> AppResult<()>;
    async fn delete_academic_paper(&self, id: &str) -> AppResult<()>;
}
