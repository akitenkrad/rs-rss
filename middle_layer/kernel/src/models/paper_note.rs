use crate::models::academic_paper::AcademicPaper;
use chrono::{DateTime, Datelike, NaiveDate, Utc};
use derive_new::new;
use openai_tools::{
    chat::request::ChatCompletion,
    common::{message::Message, role::Role, structured_output::Schema},
};
use serde::{Deserialize, Serialize};
use shared::id::PaperNoteId;

#[derive(Debug, Clone, Default, new)]
pub struct PaperNote {
    pub paper_note_id: PaperNoteId,
    pub paper: AcademicPaper,
    pub note: String,
    pub note_timestamp: NaiveDate,
}
