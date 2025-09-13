use axum::http::StatusCode;
use chrono::{DateTime, Local, Utc};
use derive_new::new;
use garde::Validate;
use kernel::models::{
    academic_paper::{AcademicPaper, AcademicPaperListOptions, Author, Journal, Task},
    list::PaginatedList,
};
use serde::{Deserialize, Serialize};
use shared::id::{AcademicPaperId, AuthorId, JournalId, TaskId};

#[derive(Debug, Clone, Deserialize, Serialize, new)]
pub struct AuthorResponse {
    pub author_id: AuthorId,
    pub ss_id: Option<String>,
    pub name: String,
    pub h_index: Option<i32>,
}

impl From<Author> for AuthorResponse {
    fn from(author: Author) -> Self {
        let Author {
            author_id,
            ss_id,
            name,
            h_index,
        } = author;
        Self {
            author_id,
            ss_id: Some(ss_id),
            name,
            h_index: Some(h_index),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, new)]
pub struct TaskResponse {
    pub task_id: TaskId,
    pub name: String,
}

impl From<Task> for TaskResponse {
    fn from(task: Task) -> Self {
        let Task { task_id, name } = task;
        Self { task_id, name }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, new)]
pub struct JournalResponse {
    pub journal_id: JournalId,
    pub name: String,
}

impl From<Journal> for JournalResponse {
    fn from(journal: Journal) -> Self {
        let Journal { journal_id, name } = journal;
        Self { journal_id, name }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, new)]
pub struct AcademicPaperResponse {
    pub paper_id: AcademicPaperId,
    pub ss_id: String,
    pub arxiv_id: String,
    pub doi: String,
    pub title: String,
    pub abstract_text: String,
    pub authors: Vec<AuthorResponse>,
    pub tasks: Vec<TaskResponse>,
    pub primary_category: String,
    pub published_date: DateTime<Local>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub journal: JournalResponse,
    pub text: String,
    pub url: String,
    pub citation_count: i32,
    pub reference_count: i32,
    pub influential_citation_count: i32,
    pub bibtex: String,
    pub summary: String,
    pub background_and_purpose: String,
    pub methodology: String,
    pub dataset: String,
    pub results: String,
    pub advantages_limitations_and_future_work: String,
}

impl From<AcademicPaper> for AcademicPaperResponse {
    fn from(paper: AcademicPaper) -> Self {
        let AcademicPaper {
            paper_id,
            ss_id,
            arxiv_id,
            title,
            abstract_text,
            abstract_text_ja: _,
            primary_category,
            published_date,
            created_at,
            updated_at,
            authors,
            tasks,
            journal,
            text,
            url,
            doi,
            citations_count: citation_count,
            references_count: reference_count,
            influential_citation_count,
            bibtex,
            summary,
            background_and_purpose,
            methodology,
            dataset,
            results,
            advantages_limitations_and_future_work,
        } = paper;
        Self {
            paper_id,
            ss_id,
            arxiv_id,
            doi,
            title,
            abstract_text,
            authors: authors.into_iter().map(AuthorResponse::from).collect(),
            tasks: tasks.into_iter().map(TaskResponse::from).collect(),
            primary_category,
            published_date,
            created_at,
            updated_at,
            journal: JournalResponse::from(journal),
            text,
            url,
            citation_count,
            reference_count,
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

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct AcademicPaperListResponse {
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub items: Vec<AcademicPaperResponse>,
    pub status_code: usize,
}

impl From<PaginatedList<AcademicPaper>> for AcademicPaperListResponse {
    fn from(paginated_list: PaginatedList<AcademicPaper>) -> Self {
        let PaginatedList {
            total,
            limit,
            offset,
            items,
        } = paginated_list;
        Self {
            total,
            limit,
            offset,
            items: items.into_iter().map(AcademicPaperResponse::from).collect(),
            status_code: StatusCode::OK.as_u16() as usize,
        }
    }
}

pub fn default_limit() -> Option<i64> {
    Some(20)
}
pub fn default_offset() -> Option<i64> {
    Some(0)
}
#[derive(Debug, Clone, Default, Deserialize, Validate, new)]
pub struct AcademicPaperListQuery {
    #[garde(range(min = 0))]
    #[serde(default = "default_limit")]
    pub limit: Option<i64>,
    #[garde(range(min = 0))]
    #[serde(default = "default_offset")]
    pub offset: Option<i64>,
}

impl From<AcademicPaperListQuery> for AcademicPaperListOptions {
    fn from(query: AcademicPaperListQuery) -> Self {
        let AcademicPaperListQuery { limit, offset } = query;
        Self {
            limit: limit.expect("Limit must be provided"),
            offset: offset.expect("Offset must be provided"),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, new)]
pub struct AcademicPaperIdQuery {
    pub paper_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate, new)]
pub struct AcademicPaperCreateRequest {
    #[garde(length(min = 1))]
    pub title: String,
    #[garde(url)]
    pub pdf_url: String,
}

impl From<AcademicPaperCreateRequest> for AcademicPaper {
    fn from(request: AcademicPaperCreateRequest) -> Self {
        let AcademicPaperCreateRequest { title, pdf_url } = request;
        Self {
            paper_id: AcademicPaperId::default(),
            ss_id: String::new(),
            arxiv_id: String::new(),
            title,
            abstract_text: String::new(),
            abstract_text_ja: String::new(),
            authors: vec![],
            tasks: vec![],
            primary_category: String::new(),
            published_date: Local::now(),
            created_at: Local::now(),
            updated_at: Local::now(),
            journal: Journal {
                journal_id: JournalId::default(),
                name: String::new(),
            },
            text: String::new(),
            url: pdf_url,
            doi: String::new(),
            citations_count: 0,
            references_count: 0,
            influential_citation_count: 0,
            bibtex: String::new(),
            summary: String::new(),
            background_and_purpose: String::new(),
            methodology: String::new(),
            dataset: String::new(),
            results: String::new(),
            advantages_limitations_and_future_work: String::new(),
        }
    }
}
