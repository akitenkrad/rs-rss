use chrono::NaiveDate;
use derive_new::new;
use kernel::models::academic_paper::{AcademicPaper, Author, Journal, Task};
use shared::id::{AcademicPaperId, AuthorId, JournalId, StatusId, TaskId};
use sqlx::FromRow;

#[derive(Debug, Clone, new, FromRow)]
pub struct AuthorRecord {
    pub author_id: AuthorId,
    pub ss_id: String,
    pub name: String,
    pub h_index: i32,
}

impl From<Author> for AuthorRecord {
    fn from(author: Author) -> Self {
        let Author {
            author_id,
            ss_id,
            name,
            h_index,
        } = author;
        Self {
            author_id,
            ss_id,
            name,
            h_index,
        }
    }
}

impl From<AuthorRecord> for Author {
    fn from(author_record: AuthorRecord) -> Self {
        let AuthorRecord {
            author_id,
            ss_id,
            name,
            h_index,
        } = author_record;
        Self {
            author_id,
            ss_id,
            name,
            h_index,
        }
    }
}

#[derive(Debug, Clone, new, FromRow)]
pub struct TaskRecord {
    pub task_id: TaskId,
    pub name: String,
}

impl From<Task> for TaskRecord {
    fn from(task: Task) -> Self {
        let Task { task_id, name } = task;
        Self { task_id, name }
    }
}

impl From<TaskRecord> for Task {
    fn from(task_record: TaskRecord) -> Self {
        let TaskRecord { task_id, name } = task_record;
        Self { task_id, name }
    }
}

#[derive(Debug, Clone, new, FromRow)]
pub struct JournalRecord {
    pub journal_id: JournalId,
    pub name: String,
}

impl From<Journal> for JournalRecord {
    fn from(journal: Journal) -> Self {
        let Journal { journal_id, name } = journal;
        Self { journal_id, name }
    }
}

impl From<JournalRecord> for Journal {
    fn from(journal_record: JournalRecord) -> Self {
        let JournalRecord { journal_id, name } = journal_record;
        Self { journal_id, name }
    }
}

#[derive(Debug, Clone, new, FromRow)]
pub struct AcademicPaperRecord {
    pub paper_id: AcademicPaperId,
    pub ss_id: String,
    pub arxiv_id: String,
    pub journal_id: JournalId,
    pub status_id: StatusId,
    pub title: String,
    pub r#abstract: String,
    pub abstract_ja: String,
    pub text: String,
    pub url: String,
    pub doi: String,
    pub published_date: NaiveDate,
    pub primary_category: String,
    pub citation_count: i32,
    pub references_count: i32,
    pub influential_citation_count: i32,
    pub bibtex: String,
    pub summary: String,
    pub background_and_purpose: String,
    pub methodology: String,
    pub dataset: String,
    pub results: String,
    pub advantages_limitations_and_future_work: String,
}

impl From<AcademicPaper> for AcademicPaperRecord {
    fn from(paper: AcademicPaper) -> Self {
        let AcademicPaper {
            paper_id,
            ss_id,
            arxiv_id,
            title,
            abstract_text,
            abstract_text_ja: abstract_ja,
            text,
            url,
            doi,
            journal,
            published_date,
            primary_category,
            citation_count,
            reference_count: references_count,
            influential_citation_count,
            bibtex,
            summary,
            background_and_purpose,
            methodology,
            dataset,
            results,
            advantages_limitations_and_future_work,
            ..
        } = paper;
        Self {
            paper_id,
            ss_id,
            arxiv_id,
            journal_id: journal.journal_id,
            status_id: StatusId::default(),
            title,
            r#abstract: abstract_text,
            abstract_ja,
            text,
            url,
            doi,
            published_date,
            primary_category,
            citation_count,
            references_count,
            influential_citation_count,
            bibtex,
            summary,
            background_and_purpose,
            methodology,
            dataset,
            results,
            advantages_limitations_and_future_work,
        }
    }
}

impl From<AcademicPaperRecord> for AcademicPaper {
    fn from(paper_record: AcademicPaperRecord) -> Self {
        let AcademicPaperRecord {
            paper_id,
            ss_id,
            arxiv_id,
            journal_id,
            status_id: _,
            title,
            r#abstract: abstract_text,
            abstract_ja,
            text,
            url,
            doi,
            published_date,
            primary_category,
            citation_count,
            references_count,
            influential_citation_count,
            bibtex,
            summary,
            background_and_purpose,
            methodology,
            dataset,
            results,
            advantages_limitations_and_future_work,
        } = paper_record;
        Self {
            paper_id,
            ss_id,
            arxiv_id,
            journal: Journal::new(journal_id, String::new()),
            authors: vec![],
            tasks: vec![],
            title,
            abstract_text,
            abstract_text_ja: abstract_ja,
            text,
            url,
            doi,
            published_date,
            primary_category,
            citation_count,
            reference_count: references_count,
            influential_citation_count,
            bibtex,
            summary,
            background_and_purpose,
            methodology,
            dataset,
            results,
            advantages_limitations_and_future_work,
        }
    }
}
