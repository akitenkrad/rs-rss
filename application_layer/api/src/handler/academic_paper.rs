use crate::models::academic_paper::{
    AcademicPaperIdQuery, AcademicPaperListQuery, AcademicPaperListResponse, AcademicPaperResponse,
};
use axum::extract::{Json, Query, State};
use garde::Validate;
use registry::AppRegistry;
use shared::errors::AppResult;

pub async fn select_paginated_academic_papers(
    State(registry): State<AppRegistry>,
    Query(query): Query<AcademicPaperListQuery>,
) -> AppResult<Json<AcademicPaperListResponse>> {
    query.validate()?;

    let mut tx = registry.db().inner_ref().begin().await?;
    let result = registry
        .academic_paper_repository()
        .select_paginated_academic_papers(&mut tx, query.into())
        .await
        .map(AcademicPaperListResponse::from)
        .map(Json);
    tx.commit().await?;
    result
}

pub async fn select_academic_papers_by_id(
    State(registry): State<AppRegistry>,
    Query(query): Query<AcademicPaperIdQuery>,
) -> AppResult<Json<AcademicPaperResponse>> {
    let mut tx = registry.db().inner_ref().begin().await?;
    let result = registry
        .academic_paper_repository()
        .select_academic_paper_by_id(&mut tx, query.paper_id.as_str())
        .await
        .map(AcademicPaperResponse::from)
        .map(Json);
    tx.commit().await?;
    result
}
