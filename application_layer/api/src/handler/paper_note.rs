use crate::models::paper_note::{
    PaperNoteCreateRequest, PaperNoteCreateResponse, PaperNoteDeleteRequest, PaperNoteDeleteResponse,
    PaperNoteResponse, PaperNoteSelectRequest, PaperNoteSelectResponse, PaperNoteUpdateRequest,
    PaperNoteUpdateResponse,
};
use axum::{
    extract::{Json, Query, State},
    http::StatusCode,
};
use kernel::models::paper_note::PaperNote;
use registry::AppRegistry;
use shared::errors::AppResult;

pub async fn select_paper_note(
    State(registry): State<AppRegistry>,
    Query(query): Query<PaperNoteSelectRequest>,
) -> AppResult<Json<PaperNoteSelectResponse>> {
    let mut tx = registry.db().inner_ref().begin().await?;
    let result = registry
        .paper_note_repository()
        .select_paper_note(&mut tx, query.paper_id)
        .await?;
    tx.commit().await?;

    let result = PaperNoteSelectResponse::from(result);
    Ok(Json(result))
}

pub async fn create_paper_note(
    State(registry): State<AppRegistry>,
    Json(body): Json<PaperNoteCreateRequest>,
) -> AppResult<Json<PaperNoteCreateResponse>> {
    let mut tx = registry.db().inner_ref().begin().await?;
    let new_paper_note = PaperNote {
        paper_note_id: Default::default(),
        note: body.text,
        note_timestamp: body.note_timestamp,
        paper: registry
            .academic_paper_repository()
            .select_academic_paper_by_id(&mut tx, &body.paper_id)
            .await?,
    };
    let created_paper_note = registry
        .paper_note_repository()
        .create_paper_note(&mut tx, new_paper_note)
        .await?;
    tx.commit().await?;

    let result = PaperNoteCreateResponse::new(
        PaperNoteResponse::from(created_paper_note),
        StatusCode::CREATED.as_u16() as usize,
    );
    Ok(Json(result))
}

pub async fn update_paper_note(
    State(registry): State<AppRegistry>,
    Json(body): Json<PaperNoteUpdateRequest>,
) -> AppResult<Json<PaperNoteUpdateResponse>> {
    let mut tx = registry.db().inner_ref().begin().await?;

    let updated_paper_note = registry
        .paper_note_repository()
        .update_paper_note(&mut tx, PaperNote::from(body))
        .await?;
    tx.commit().await?;

    let result = PaperNoteUpdateResponse::new(
        PaperNoteResponse::from(updated_paper_note),
        StatusCode::OK.as_u16() as usize,
    );
    Ok(Json(result))
}

pub async fn delete_paper_note(
    State(registry): State<AppRegistry>,
    Json(body): Json<PaperNoteDeleteRequest>,
) -> AppResult<Json<PaperNoteDeleteResponse>> {
    let mut tx = registry.db().inner_ref().begin().await?;

    registry
        .paper_note_repository()
        .delete_paper_note(&mut tx, body.paper_note_id)
        .await?;
    tx.commit().await?;

    let result = PaperNoteDeleteResponse::new(StatusCode::OK.as_u16() as usize);
    Ok(Json(result))
}
