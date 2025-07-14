use crate::database::{
    models::academic_paper::{AcademicPaperRecord, AuthorRecord, JournalRecord, TaskRecord},
    ConnectionPool,
};
use async_trait::async_trait;
use derive_new::new;
use kernel::{
    models::{
        academic_paper::{AcademicPaper, AcademicPaperListOptions, Author, AuthorListOptions, Journal, Task},
        list::PaginatedList,
    },
    repository::academic_paper::{AcademicPaperRepository, AuthorRepository, JournalRepository, TaskRepository},
};
use shared::{
    errors::{AppError, AppResult},
    id::{AcademicPaperId, AuthorId, JournalId, TaskId},
    utils::levenshtein_dist,
};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone, new)]
pub struct AuthorRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl AuthorRepository for AuthorRepositoryImpl {
    async fn create_author(&self, author: Author) -> AppResult<Author> {
        // Check if the author already exists by ss_id
        if let Ok(existing_author) = self.select_author_by_ssid(&author.ss_id).await {
            return Ok(existing_author);
        }

        // If the author does not exist, insert a new record
        let res = sqlx::query!(
            r#"INSERT INTO author (
                ss_id,
                name,
                h_index
            ) VALUES ($1, $2, $3)
            RETURNING author_id"#,
            author.ss_id,
            author.name,
            author.h_index
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(|e| shared::errors::AppError::SqlxError(e))?;

        Ok(Author {
            author_id: AuthorId::from(res.author_id),
            ss_id: author.ss_id,
            name: author.name,
            h_index: author.h_index,
        })
    }
    async fn select_author_by_id(&self, id: &str) -> AppResult<Author> {
        let author = sqlx::query_as!(
            Author,
            r#"SELECT author_id, ss_id, name, h_index FROM author WHERE author_id = $1"#,
            Uuid::from_str(id)?
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(|e| shared::errors::AppError::SqlxError(e))?;

        Ok(author)
    }
    async fn select_author_by_ssid(&self, ss_id: &str) -> AppResult<Author> {
        let author = sqlx::query_as!(
            Author,
            r#"SELECT author_id, ss_id, name, h_index FROM author WHERE ss_id = $1"#,
            ss_id
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(|e| shared::errors::AppError::SqlxError(e))?;

        Ok(author)
    }
    async fn select_all_authors(&self) -> AppResult<Vec<Author>> {
        let authors = sqlx::query_as!(Author, r#"SELECT author_id, ss_id, name, h_index FROM author"#)
            .fetch_all(self.db.inner_ref())
            .await
            .map_err(|e| shared::errors::AppError::SqlxError(e))?;

        Ok(authors)
    }
    async fn select_all_authors_paginated(&self, options: AuthorListOptions) -> AppResult<PaginatedList<Author>> {
        let total_count = sqlx::query_scalar!(r#"SELECT COUNT(*) FROM author"#)
            .fetch_one(self.db.inner_ref())
            .await
            .map_err(|e| shared::errors::AppError::SqlxError(e))?
            .expect("Total count should not be None");

        let authors = sqlx::query_as!(
            Author,
            r#"SELECT author_id, ss_id, name, h_index FROM author
            ORDER BY name
            LIMIT $1 OFFSET $2"#,
            options.limit as i64,
            options.offset as i64
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(|e| shared::errors::AppError::SqlxError(e))?;

        Ok(PaginatedList::<Author>::new(
            total_count,
            options.limit,
            options.offset,
            authors,
        ))
    }

    async fn delete_author(&self, id: &str) -> AppResult<()> {
        sqlx::query!(r#"DELETE FROM author WHERE author_id = $1"#, Uuid::from_str(id)?)
            .execute(self.db.inner_ref())
            .await
            .map_err(|e| shared::errors::AppError::SqlxError(e))?;

        Ok(())
    }
}

#[derive(Debug, Clone, new)]
pub struct TaskRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl TaskRepository for TaskRepositoryImpl {
    async fn create_task(&self, task: Task) -> AppResult<Task> {
        // Check if the task already exists by name
        if let Ok(existing_task) = self.select_task_by_name(&task.name).await {
            return Ok(existing_task);
        }

        // If the task does not exist, insert a new record
        let res = sqlx::query!(
            r#"INSERT INTO task (
                name
            ) VALUES ($1)
            RETURNING task_id"#,
            task.name
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(|e| shared::errors::AppError::SqlxError(e))?;

        Ok(Task {
            task_id: TaskId::from(res.task_id),
            name: task.name,
        })
    }
    async fn select_task_by_id(&self, id: &str) -> AppResult<Task> {
        let task = sqlx::query_as!(
            Task,
            r#"SELECT task_id, name FROM task WHERE task_id = $1"#,
            Uuid::from_str(id)?
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(|e| shared::errors::AppError::SqlxError(e))?;

        Ok(task)
    }
    async fn select_task_by_name(&self, name: &str) -> AppResult<Task> {
        let task = sqlx::query_as!(Task, r#"SELECT task_id, name FROM task WHERE name = $1"#, name)
            .fetch_one(self.db.inner_ref())
            .await
            .map_err(|e| shared::errors::AppError::SqlxError(e))?;

        Ok(task)
    }
    async fn select_all_tasks(&self) -> AppResult<Vec<Task>> {
        let tasks = sqlx::query_as!(Task, r#"SELECT task_id, name FROM task"#)
            .fetch_all(self.db.inner_ref())
            .await
            .map_err(|e| shared::errors::AppError::SqlxError(e))?;

        Ok(tasks)
    }
    async fn delete_task(&self, id: &str) -> AppResult<()> {
        sqlx::query!(r#"DELETE FROM task WHERE task_id = $1"#, Uuid::from_str(id)?)
            .execute(self.db.inner_ref())
            .await
            .map_err(|e| shared::errors::AppError::SqlxError(e))?;

        Ok(())
    }
}

#[derive(Debug, Clone, new)]
pub struct JournalRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl JournalRepository for JournalRepositoryImpl {
    async fn create_journal(&self, journal: Journal) -> AppResult<Journal> {
        // Check if the journal already exists by name
        if let Ok(existing_journal) = self.select_journal_by_name(&journal.name).await {
            return Ok(existing_journal);
        }

        // If the journal does not exist, insert a new record
        let res = sqlx::query!(
            r#"INSERT INTO journal (
                name
            ) VALUES ($1)
            RETURNING journal_id"#,
            journal.name
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(|e| shared::errors::AppError::SqlxError(e))?;

        Ok(Journal {
            journal_id: JournalId::from(res.journal_id),
            name: journal.name,
        })
    }
    async fn select_journal_by_id(&self, id: &str) -> AppResult<Journal> {
        let journal = sqlx::query_as!(
            Journal,
            r#"SELECT journal_id, name FROM journal WHERE journal_id = $1"#,
            Uuid::from_str(id)?
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(|e| shared::errors::AppError::SqlxError(e))?;

        Ok(journal)
    }
    async fn select_journal_by_name(&self, name: &str) -> AppResult<Journal> {
        let journal = sqlx::query_as!(Journal, r#"SELECT journal_id, name FROM journal WHERE name = $1"#, name)
            .fetch_one(self.db.inner_ref())
            .await
            .map_err(|e| shared::errors::AppError::SqlxError(e))?;

        Ok(journal)
    }
    async fn select_all_journals(&self) -> AppResult<Vec<Journal>> {
        let journals = sqlx::query_as!(Journal, r#"SELECT journal_id, name FROM journal"#)
            .fetch_all(self.db.inner_ref())
            .await
            .map_err(|e| shared::errors::AppError::SqlxError(e))?;

        Ok(journals)
    }
    async fn delete_journal(&self, id: &str) -> AppResult<()> {
        sqlx::query!(r#"DELETE FROM journal WHERE journal_id = $1"#, Uuid::from_str(id)?)
            .execute(self.db.inner_ref())
            .await
            .map_err(|e| shared::errors::AppError::SqlxError(e))?;

        Ok(())
    }
}

#[derive(Debug, Clone, new)]
pub struct AcademicPaperRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl AcademicPaperRepository for AcademicPaperRepositoryImpl {
    async fn create_academic_paper(&self, academic_paper: AcademicPaper) -> AppResult<AcademicPaper> {
        // Check if the academic paper already exists by ss_id
        if let Ok(existing_paper) = self.select_academic_paper_by_title(&academic_paper.title).await {
            for paper in existing_paper {
                let lev_dist = levenshtein_dist(&paper.title, &academic_paper.title);
                if lev_dist < 2 {
                    // If the paper already exists with a similar title, return it
                    return Ok(paper);
                }
            }
        }

        // If the academic paper does not exist, insert a new record
        // Check if the authors exist, if not, create them
        let mut authors: Vec<Author> = vec![];
        for author in &academic_paper.authors {
            let author = AuthorRepositoryImpl::new(self.db.clone())
                .create_author(author.clone())
                .await?;
            authors.push(author);
        }

        // Check if the journal exists, if not, create it
        let journal = JournalRepositoryImpl::new(self.db.clone())
            .create_journal(academic_paper.journal.clone())
            .await?;

        // Check if the tasks exist, if not, create them
        let mut tasks: Vec<Task> = vec![];
        for task in &academic_paper.tasks {
            let task = TaskRepositoryImpl::new(self.db.clone())
                .create_task(task.clone())
                .await?;
            tasks.push(task);
        }

        // get status of the academic paper
        let status_id = sqlx::query!(r#"SELECT status_id FROM status WHERE name = 'todo'"#,)
            .fetch_one(self.db.inner_ref())
            .await
            .map_err(|e| shared::errors::AppError::SqlxError(e))?
            .status_id;

        let res = sqlx::query!(
            r#"INSERT INTO academic_paper (
                arxiv_id,
                ss_id,
                title,
                abstract,
                abstract_ja,
                journal_id,
                primary_category,
                citation_count,
                influential_citation_count,
                references_count,
                published_date,
                url,
                text,
                bibtex,
                summary,
                background_and_purpose,
                methodology,
                dataset,
                results,
                advantages_limitations_and_future_work,
                status_id
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21)
            RETURNING paper_id"#,
            academic_paper.arxiv_id,
            academic_paper.ss_id,
            academic_paper.title,
            academic_paper.abstract_text,
            academic_paper.abstract_text_ja,
            Uuid::from(journal.journal_id),
            academic_paper.primary_category,
            academic_paper.citation_count,
            academic_paper.influential_citation_count,
            academic_paper.reference_count,
            academic_paper.published_date,
            academic_paper.url,
            academic_paper.text,
            academic_paper.bibtex,
            academic_paper.summary,
            academic_paper.background_and_purpose,
            academic_paper.methodology,
            academic_paper.dataset,
            academic_paper.results,
            academic_paper.advantages_limitations_and_future_work,
            status_id
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(|e| shared::errors::AppError::SqlxError(e))?;

        // Insert authors and tasks into the join tables
        for author in authors.iter() {
            // Check if the author already exists in the relation
            let exists = sqlx::query!(
                r#"SELECT paper_id FROM author_paper_relation WHERE paper_id = $1 AND author_id = $2"#,
                res.paper_id.clone(),
                Uuid::from(author.author_id)
            )
            .fetch_optional(self.db.inner_ref())
            .await
            .map_err(|e| shared::errors::AppError::SqlxError(e))?;
            if exists.is_none() {
                // If not, insert the relation
                sqlx::query!(
                    r#"INSERT INTO author_paper_relation (paper_id, author_id) VALUES ($1, $2)"#,
                    res.paper_id,
                    Uuid::from(author.author_id)
                )
                .execute(self.db.inner_ref())
                .await
                .map_err(|e| shared::errors::AppError::SqlxError(e))?;
            }
        }

        for task in tasks.iter() {
            // Check if the task already exists in the relation
            let exists = sqlx::query!(
                r#"SELECT paper_id FROM task_paper_relation WHERE paper_id = $1 AND task_id = $2"#,
                res.paper_id.clone(),
                Uuid::from(task.task_id)
            )
            .fetch_optional(self.db.inner_ref())
            .await
            .map_err(|e| shared::errors::AppError::SqlxError(e))?;
            if exists.is_none() {
                // If not, insert the relation
                sqlx::query!(
                    r#"INSERT INTO task_paper_relation (paper_id, task_id) VALUES ($1, $2)"#,
                    res.paper_id,
                    Uuid::from(task.task_id)
                )
                .execute(self.db.inner_ref())
                .await
                .map_err(|e| shared::errors::AppError::SqlxError(e))?;
            }
        }

        Ok(AcademicPaper {
            paper_id: AcademicPaperId::from(res.paper_id),
            ss_id: academic_paper.ss_id,
            arxiv_id: academic_paper.arxiv_id,
            title: academic_paper.title,
            abstract_text: academic_paper.abstract_text,
            abstract_text_ja: academic_paper.abstract_text_ja,
            authors: authors,
            journal: journal,
            tasks: tasks,
            text: academic_paper.text,
            url: academic_paper.url,
            doi: academic_paper.doi,
            published_date: academic_paper.published_date,
            primary_category: academic_paper.primary_category,
            citation_count: academic_paper.citation_count,
            reference_count: academic_paper.reference_count,
            influential_citation_count: academic_paper.influential_citation_count,
            bibtex: academic_paper.bibtex,
            summary: academic_paper.summary,
            background_and_purpose: academic_paper.background_and_purpose,
            methodology: academic_paper.methodology,
            dataset: academic_paper.dataset,
            results: academic_paper.results,
            advantages_limitations_and_future_work: academic_paper.advantages_limitations_and_future_work,
        })
    }
    async fn fill_fields(&self, academic_paper: &mut AcademicPaper) -> AppResult<()> {
        // Fill authors
        let authors = sqlx::query_as!(
            AuthorRecord,
            r#"SELECT author.author_id, ss_id, name, h_index FROM author
            JOIN author_paper_relation ON author.author_id = author_paper_relation.author_id
            WHERE author_paper_relation.paper_id = $1"#,
            Uuid::from(academic_paper.paper_id)
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(|e| AppError::EntityNotFound(e.to_string()))?;
        academic_paper.authors = authors.into_iter().map(Author::from).collect();

        // Fill journal
        let journal = sqlx::query_as!(
            JournalRecord,
            r#"SELECT journal_id, name FROM journal WHERE journal_id = $1"#,
            Uuid::from(academic_paper.journal.journal_id)
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(|e| AppError::EntityNotFound(e.to_string()))?;
        academic_paper.journal = Journal::from(journal);

        // Fill tasks
        let tasks = sqlx::query_as!(
            TaskRecord,
            r#"SELECT task.task_id, name FROM task
            JOIN task_paper_relation ON task.task_id = task_paper_relation.task_id
            WHERE task_paper_relation.paper_id = $1"#,
            Uuid::from(academic_paper.paper_id)
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(|e| AppError::EntityNotFound(e.to_string()))?;
        academic_paper.tasks = tasks.into_iter().map(Task::from).collect();

        Ok(())
    }
    async fn select_todays_articles(&self) -> AppResult<Vec<AcademicPaper>> {
        let current_date = chrono::Local::now().date_naive();
        let papers = sqlx::query_as!(
            AcademicPaperRecord,
            r#"SELECT
                paper_id,
                ss_id,
                arxiv_id,
                journal_id,
                title,
                abstract,
                abstract_ja,
                text,
                url,
                doi,
                published_date,
                primary_category,
                citation_count,
                references_count,
                influential_citation_count,
                status_id,
                bibtex,
                summary,
                background_and_purpose,
                methodology,
                dataset,
                results,
                advantages_limitations_and_future_work
            FROM academic_paper WHERE published_date = $1"#,
            &current_date
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(|e| AppError::EntityNotFound(e.to_string()))?;

        let mut academic_papers: Vec<AcademicPaper> = vec![];
        // fill fields
        for paper in papers.iter() {
            let mut academic_paper = AcademicPaper::from(paper.clone());
            self.fill_fields(&mut academic_paper).await?;
            academic_papers.push(academic_paper);
        }

        Ok(academic_papers)
    }
    async fn select_academic_paper_by_arxiv_id(&self, arxiv_id: &str) -> AppResult<AcademicPaper> {
        let paper = sqlx::query_as!(
            AcademicPaperRecord,
            r#"SELECT
                paper_id,
                ss_id,
                arxiv_id,
                journal_id,
                title,
                abstract,
                abstract_ja,
                text,
                url,
                doi,
                published_date,
                primary_category,
                citation_count,
                references_count,
                influential_citation_count,
                status_id,
                bibtex,
                summary,
                background_and_purpose,
                methodology,
                dataset,
                results,
                advantages_limitations_and_future_work
            FROM academic_paper WHERE arxiv_id = $1"#,
            arxiv_id
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(|e| AppError::EntityNotFound(e.to_string()))?;

        let mut academic_paper = AcademicPaper::from(paper);
        self.fill_fields(&mut academic_paper).await?;
        Ok(academic_paper)
    }
    async fn select_academic_paper_by_ss_id(&self, ss_id: &str) -> AppResult<AcademicPaper> {
        let paper = sqlx::query_as!(
            AcademicPaperRecord,
            r#"SELECT
                paper_id,
                ss_id,
                arxiv_id,
                journal_id,
                title,
                abstract,
                abstract_ja,
                text,
                url,
                doi,
                published_date,
                primary_category,
                citation_count,
                references_count,
                influential_citation_count,
                status_id,
                bibtex,
                summary,
                background_and_purpose,
                methodology,
                dataset,
                results,
                advantages_limitations_and_future_work
            FROM academic_paper WHERE ss_id = $1"#,
            ss_id
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(|e| AppError::EntityNotFound(e.to_string()))?;

        let mut academic_paper = AcademicPaper::from(paper);
        self.fill_fields(&mut academic_paper).await?;
        Ok(academic_paper)
    }
    async fn select_academic_paper_by_id(&self, id: &str) -> AppResult<AcademicPaper> {
        let paper = sqlx::query_as!(
            AcademicPaperRecord,
            r#"SELECT
                paper_id,
                ss_id,
                arxiv_id,
                journal_id,
                title,
                abstract,
                abstract_ja,
                text,
                url,
                doi,
                published_date,
                primary_category,
                citation_count,
                references_count,
                influential_citation_count,
                status_id,
                bibtex,
                summary,
                background_and_purpose,
                methodology,
                dataset,
                results,
                advantages_limitations_and_future_work
            FROM academic_paper WHERE paper_id = $1"#,
            Uuid::from_str(id)?
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(|e| AppError::EntityNotFound(e.to_string()))?;

        let mut academic_paper = AcademicPaper::from(paper);
        self.fill_fields(&mut academic_paper).await?;
        Ok(academic_paper)
    }
    async fn select_academic_paper_by_title(&self, title: &str) -> AppResult<Vec<AcademicPaper>> {
        let papers = sqlx::query_as!(
            AcademicPaperRecord,
            r#"SELECT
                paper_id,
                ss_id,
                arxiv_id,
                journal_id,
                title,
                abstract,
                abstract_ja,
                text,
                url,
                doi,
                published_date,
                primary_category,
                citation_count,
                references_count,
                influential_citation_count,
                status_id,
                bibtex,
                summary,
                background_and_purpose,
                methodology,
                dataset,
                results,
                advantages_limitations_and_future_work
            FROM academic_paper WHERE title ILIKE $1"#,
            format!("%{}%", title)
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(|e| AppError::EntityNotFound(e.to_string()))?;

        let mut academic_papers: Vec<AcademicPaper> = vec![];
        // fill fields
        for paper in papers.iter() {
            let mut academic_paper = AcademicPaper::from(paper.clone());
            self.fill_fields(&mut academic_paper).await?;
            academic_papers.push(academic_paper);
        }

        Ok(academic_papers)
    }
    async fn select_all_academic_papers(&self) -> AppResult<Vec<AcademicPaper>> {
        let papers = sqlx::query_as!(
            AcademicPaperRecord,
            r#"SELECT
                paper_id,
                ss_id,
                arxiv_id,
                journal_id,
                title,
                abstract,
                abstract_ja,
                text,
                url,
                doi,
                published_date,
                primary_category,
                citation_count,
                references_count,
                influential_citation_count,
                status_id,
                bibtex,
                summary,
                background_and_purpose,
                methodology,
                dataset,
                results,
                advantages_limitations_and_future_work
            FROM academic_paper"#
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(|e| AppError::EntityNotFound(e.to_string()))?;

        let mut academic_papers: Vec<AcademicPaper> = vec![];
        // fill fields
        for paper in papers.iter() {
            let mut academic_paper = AcademicPaper::from(paper.clone());
            self.fill_fields(&mut academic_paper).await?;
            academic_papers.push(academic_paper);
        }

        Ok(academic_papers)
    }
    async fn select_paginated_academic_papers(
        &self,
        options: AcademicPaperListOptions,
    ) -> AppResult<PaginatedList<AcademicPaper>> {
        let total_count = sqlx::query_scalar!(r#"SELECT COUNT(*) FROM academic_paper"#)
            .fetch_one(self.db.inner_ref())
            .await
            .map_err(|e| shared::errors::AppError::SqlxError(e))?
            .expect("Total count should not be None");

        let papers = sqlx::query_as!(
            AcademicPaperRecord,
            r#"SELECT
                paper_id,
                ss_id,
                arxiv_id,
                journal_id,
                title,
                abstract,
                abstract_ja,
                text,
                url,
                doi,
                published_date,
                primary_category,
                citation_count,
                references_count,
                influential_citation_count,
                status_id,
                bibtex,
                summary,
                background_and_purpose,
                methodology,
                dataset,
                results,
                advantages_limitations_and_future_work
            FROM academic_paper
            ORDER BY published_date DESC
            LIMIT $1 OFFSET $2"#,
            options.limit as i64,
            options.offset as i64
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(|e| AppError::EntityNotFound(e.to_string()))?;

        let mut academic_papers: Vec<AcademicPaper> = vec![];
        // fill fields
        for paper in papers.iter() {
            let mut academic_paper = AcademicPaper::from(paper.clone());
            self.fill_fields(&mut academic_paper).await?;
            academic_papers.push(academic_paper);
        }

        Ok(PaginatedList::<AcademicPaper>::new(
            total_count,
            options.limit,
            options.offset,
            academic_papers,
        ))
    }
    async fn select_academic_papers_by_keyword(&self, keyword: &str) -> AppResult<Vec<AcademicPaper>> {
        let papers = sqlx::query_as!(
            AcademicPaperRecord,
            r#"SELECT
                paper_id,
                ss_id,
                arxiv_id,
                journal_id,
                title,
                abstract,
                abstract_ja,
                text,
                url,
                doi,
                published_date,
                primary_category,
                citation_count,
                references_count,
                influential_citation_count,
                status_id,
                bibtex,
                summary,
                background_and_purpose,
                methodology,
                dataset,
                results,
                advantages_limitations_and_future_work
            FROM academic_paper
            WHERE 
                title ILIKE $1
                OR abstract ILIKE $1
                OR abstract_ja ILIKE $1
            "#,
            format!("%{}%", keyword)
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(|e| AppError::EntityNotFound(e.to_string()))?;

        let mut academic_papers: Vec<AcademicPaper> = vec![];
        // fill fields
        for paper in papers.iter() {
            let mut academic_paper = AcademicPaper::from(paper.clone());
            self.fill_fields(&mut academic_paper).await?;
            academic_papers.push(academic_paper);
        }

        Ok(academic_papers)
    }
    async fn delete_academic_paper(&self, id: &str) -> AppResult<()> {
        sqlx::query!(r#"DELETE FROM academic_paper WHERE paper_id = $1"#, Uuid::from_str(id)?)
            .execute(self.db.inner_ref())
            .await
            .map_err(|e| shared::errors::AppError::SqlxError(e))?;

        Ok(())
    }
}
