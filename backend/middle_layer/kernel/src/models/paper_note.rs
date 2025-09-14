use crate::models::academic_paper::AcademicPaper;
use chrono::{DateTime, Local};
use derive_new::new;
use shared::id::PaperNoteId;

#[derive(Debug, Clone, Default, new)]
pub struct PaperNote {
    pub paper_note_id: PaperNoteId,
    pub paper: AcademicPaper,
    pub note: String,
    pub note_timestamp: DateTime<Local>,
}
