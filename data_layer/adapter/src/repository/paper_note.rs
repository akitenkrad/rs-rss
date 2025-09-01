use crate::{
    database::{models::paper_note::PaperNoteRecord, ConnectionPool},
    repository::academic_paper::AcademicPaperRepositoryImpl,
};
use async_trait::async_trait;
use derive_new::new;
use kernel::{
    models::paper_note::PaperNote,
    repository::{academic_paper::AcademicPaperRepository, paper_note::PaperNoteRepository},
};
use shared::{
    errors::AppResult,
    id::{AcademicPaperId, PaperNoteId},
};
use sqlx::{Postgres as Pg, Transaction as T};
use uuid::Uuid;

#[derive(new)]
pub struct PaperNoteRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl PaperNoteRepository for PaperNoteRepositoryImpl {
    async fn select_paper_note(&self, tx: &mut T<'_, Pg>, paper_id: AcademicPaperId) -> AppResult<Vec<PaperNote>> {
        let records = sqlx::query_as!(
            PaperNoteRecord,
            r#"
            SELECT
                paper_note.paper_note_id,
                paper_note.note,
                paper_note.note_timestamp,
                paper_note_relation.paper_id
            FROM paper_note
            INNER JOIN paper_note_relation ON paper_note.paper_note_id = paper_note_relation.paper_note_id
            WHERE paper_note_relation.paper_id = $1
            "#,
            Uuid::from(paper_id)
        )
        .fetch_all(&mut **tx)
        .await?;

        let mut paper_notes: Vec<PaperNote> = records.into_iter().map(|record| PaperNote::from(record)).collect();

        for note in paper_notes.iter_mut() {
            let academic_paper = AcademicPaperRepositoryImpl::new(self.db.clone())
                .select_academic_paper_by_id(tx, &note.paper.paper_id.to_string())
                .await?;
            note.paper = academic_paper;
        }

        Ok(paper_notes)
    }

    async fn create_paper_note(&self, tx: &mut T<'_, Pg>, paper_note: PaperNote) -> AppResult<PaperNote> {
        let created_paper_note = sqlx::query_as!(
            PaperNoteRecord,
            r#"
            WITH inserted_note AS (
                INSERT INTO paper_note (paper_note_id, note, note_timestamp)
                VALUES ($1, $2, $3)
                RETURNING paper_note_id, note, note_timestamp
            ),
            inserted_relation AS (
                INSERT INTO paper_note_relation (paper_note_id, paper_id)
                SELECT paper_note_id, $4 FROM inserted_note
                RETURNING paper_note_id, paper_id
            )
            SELECT 
                n.paper_note_id,
                n.note,
                n.note_timestamp,
                r.paper_id
            FROM inserted_note n
            INNER JOIN inserted_relation r ON n.paper_note_id = r.paper_note_id
            "#,
            Uuid::from(paper_note.paper_note_id),
            paper_note.note,
            paper_note.note_timestamp.and_hms_opt(0, 0, 0).unwrap().and_utc(),
            Uuid::from(paper_note.paper.paper_id)
        )
        .fetch_one(&mut **tx)
        .await?;

        let mut result_note = PaperNote::from(created_paper_note);
        result_note.paper = paper_note.paper;

        Ok(result_note)
    }

    async fn update_paper_note(&self, tx: &mut T<'_, Pg>, paper_note: PaperNote) -> AppResult<PaperNote> {
        let updated_paper_note = sqlx::query_as!(
            PaperNoteRecord,
            r#"
            WITH updated_note AS (
                UPDATE paper_note
                SET note = $2, note_timestamp = $3
                WHERE paper_note_id = $1
                RETURNING paper_note_id, note, note_timestamp
            )
            SELECT 
                n.paper_note_id,
                n.note,
                n.note_timestamp,
                r.paper_id
            FROM updated_note n
            INNER JOIN paper_note_relation r ON n.paper_note_id = r.paper_note_id
            "#,
            Uuid::from(paper_note.paper_note_id),
            paper_note.note,
            paper_note.note_timestamp.and_hms_opt(0, 0, 0).unwrap().and_utc()
        )
        .fetch_one(&mut **tx)
        .await?;

        let mut result_note = PaperNote::from(updated_paper_note);
        result_note.paper = paper_note.paper;

        Ok(result_note)
    }

    async fn delete_paper_note(&self, tx: &mut T<'_, Pg>, paper_note_id: PaperNoteId) -> AppResult<()> {
        sqlx::query!(
            r#"
            DELETE FROM paper_note_relation
            WHERE paper_note_id = $1
            "#,
            Uuid::from(paper_note_id)
        )
        .execute(&mut **tx)
        .await?;

        sqlx::query!(
            r#"
            DELETE FROM paper_note
            WHERE paper_note_id = $1
            "#,
            Uuid::from(paper_note_id)
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }
}
