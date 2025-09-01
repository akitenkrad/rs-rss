use axum::http::StatusCode;
use chrono::NaiveDate;
use derive_new::new;
use kernel::models::paper_note::PaperNote;
use serde::{Deserialize, Serialize};
use shared::id::{AcademicPaperId, PaperNoteId};

#[derive(Debug, Clone, Deserialize, Serialize, new)]
pub struct PaperNoteResponse {
    pub paper_note_id: PaperNoteId,
    pub text: String,
    pub note_timestamp: NaiveDate,
}

#[derive(Debug, Clone, Deserialize, Serialize, new)]
pub struct PaperNoteSelectRequest {
    pub paper_id: AcademicPaperId,
}

#[derive(Debug, Clone, Deserialize, Serialize, new)]
pub struct PaperNoteSelectResponse {
    pub paper_notes: Vec<PaperNoteResponse>,
    pub status_code: usize,
}

impl From<Vec<PaperNote>> for PaperNoteSelectResponse {
    fn from(paper_notes: Vec<PaperNote>) -> Self {
        Self {
            paper_notes: paper_notes.into_iter().map(PaperNoteResponse::from).collect(),
            status_code: StatusCode::OK.as_u16() as usize,
        }
    }
}

impl From<PaperNote> for PaperNoteResponse {
    fn from(paper_note: PaperNote) -> Self {
        let PaperNote {
            paper_note_id,
            note: text,
            note_timestamp,
            ..
        } = paper_note;
        Self {
            paper_note_id,
            text,
            note_timestamp,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, new)]
pub struct PaperNoteCreateRequest {
    pub paper_id: String,
    pub text: String,
    pub note_timestamp: NaiveDate,
}

#[derive(Debug, Clone, Deserialize, Serialize, new)]
pub struct PaperNoteCreateResponse {
    pub paper_note: PaperNoteResponse,
    pub status_code: usize,
}

impl From<PaperNote> for PaperNoteCreateResponse {
    fn from(paper_note: PaperNote) -> Self {
        Self {
            paper_note: PaperNoteResponse::from(paper_note),
            status_code: StatusCode::CREATED.as_u16() as usize,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, new)]
pub struct PaperNoteUpdateRequest {
    pub paper_note_id: String,
    pub paper_id: String,
    pub text: String,
    pub note_timestamp: NaiveDate,
}

impl From<PaperNoteUpdateRequest> for PaperNote {
    fn from(req: PaperNoteUpdateRequest) -> Self {
        let PaperNoteUpdateRequest {
            paper_note_id,
            text,
            note_timestamp,
            ..
        } = req;
        Self {
            paper_note_id: PaperNoteId::from(paper_note_id),
            paper: Default::default(),
            note: text,
            note_timestamp,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, new)]
pub struct PaperNoteUpdateResponse {
    pub paper_note: PaperNoteResponse,
    pub status_code: usize,
}

impl From<PaperNote> for PaperNoteUpdateResponse {
    fn from(paper_note: PaperNote) -> Self {
        Self {
            paper_note: PaperNoteResponse::from(paper_note),
            status_code: StatusCode::CREATED.as_u16() as usize,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, new)]
pub struct PaperNoteDeleteRequest {
    pub paper_note_id: PaperNoteId,
}

#[derive(Debug, Clone, Deserialize, Serialize, new)]
pub struct PaperNoteDeleteResponse {
    pub status_code: usize,
}
