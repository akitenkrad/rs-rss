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

    registry
        .academic_paper_repository()
        .select_paginated_academic_papers(query.into())
        .await
        .map(AcademicPaperListResponse::from)
        .map(Json)
}

pub async fn select_academic_papers_by_id(
    State(registry): State<AppRegistry>,
    Query(query): Query<AcademicPaperIdQuery>,
) -> AppResult<Json<AcademicPaperResponse>> {
    registry
        .academic_paper_repository()
        .select_academic_paper_by_id(query.paper_id.as_str())
        .await
        .map(AcademicPaperResponse::from)
        .map(Json)
}
