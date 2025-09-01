use crate::models::paper_note::PaperNote;
use async_trait::async_trait;
use shared::{
    errors::AppResult,
    id::{AcademicPaperId, PaperNoteId},
};
use sqlx::{Postgres as Pg, Transaction as T};

#[async_trait]
pub trait PaperNoteRepository: Send + Sync {
    async fn select_paper_note(&self, tx: &mut T<'_, Pg>, paper_id: AcademicPaperId) -> AppResult<Vec<PaperNote>>;
    async fn create_paper_note(&self, tx: &mut T<'_, Pg>, paper_note: PaperNote) -> AppResult<PaperNote>;
    async fn update_paper_note(&self, tx: &mut T<'_, Pg>, paper_note: PaperNote) -> AppResult<PaperNote>;
    async fn delete_paper_note(&self, tx: &mut T<'_, Pg>, paper_note_id: PaperNoteId) -> AppResult<()>;
}
