use chrono::NaiveDate;
use derive_new::new;
use shared::id::{AcademicPaperId, AuthorId, JournalId, TaskId};

#[derive(Debug, Clone, new)]
pub struct Author {
    pub author_id: AuthorId,
    pub ss_id: String,
    pub name: String,
    pub h_index: i32,
}

#[derive(Debug, Clone, new)]
pub struct Task {
    pub task_id: TaskId,
    pub name: String,
}

#[derive(Debug, Clone, new)]
pub struct Journal {
    pub journal_id: JournalId,
    pub name: String,
}

#[derive(Debug, Clone, new)]
pub struct AcademicPaper {
    pub paper_id: AcademicPaperId,
    pub ss_id: String,
    pub arxiv_id: String,
    pub journal: Journal,
    pub authors: Vec<Author>,
    pub tasks: Vec<Task>,
    pub title: String,
    pub abstract_text: String,
    pub abstract_ja: String,
    pub text: String,
    pub url: String,
    pub published_date: NaiveDate,
    pub primary_category: String,
    pub citation_count: i32,
    pub references_count: i32,
    pub influential_citation_count: i32,
    pub bibtex: String,
}
