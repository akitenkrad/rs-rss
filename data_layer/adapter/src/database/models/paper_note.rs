use chrono::{DateTime, Utc};
use derive_new::new;
use kernel::models::{academic_paper::AcademicPaper, paper_note::PaperNote};
use shared::id::{AcademicPaperId, PaperNoteId};
use sqlx::FromRow;

#[derive(Debug, Clone, new, FromRow)]
pub struct PaperNoteRecord {
    pub paper_note_id: PaperNoteId,
    pub paper_id: AcademicPaperId,
    pub note: String,
    pub note_timestamp: Option<DateTime<Utc>>,
}

impl From<PaperNote> for PaperNoteRecord {
    fn from(paper_note: PaperNote) -> Self {
        let PaperNote {
            paper_note_id,
            paper,
            note,
            note_timestamp,
        } = paper_note;
        Self {
            paper_note_id,
            paper_id: paper.paper_id,
            note,
            note_timestamp: Some(note_timestamp.and_hms_opt(0, 0, 0).unwrap().and_utc()),
        }
    }
}

impl From<PaperNoteRecord> for PaperNote {
    fn from(paper_note_record: PaperNoteRecord) -> Self {
        let PaperNoteRecord {
            paper_note_id,
            paper_id,
            note,
            note_timestamp,
        } = paper_note_record;
        Self {
            paper_note_id,
            paper: AcademicPaper {
                paper_id,
                ..Default::default()
            },
            note,
            note_timestamp: note_timestamp.unwrap().date_naive(),
        }
    }
}
