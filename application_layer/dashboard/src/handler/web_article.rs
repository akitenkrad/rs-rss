use crate::models::web_article::{PaginatedWebSiteResponse, WebSiteListQuery, WebSiteResponse};
use axum::{
    extract::{Json, Query, State},
    http::StatusCode,
};
use garde::Validate;
use registry::AppRegistry;
use shared::errors::{AppError, AppResult};

pub async fn get_all_sites(
    State(registry): State<AppRegistry>,
    Query(query): Query<WebSiteListQuery>,
) -> AppResult<Json<PaginatedWebSiteResponse>> {
    query.validate()?;

    registry
        .web_site_repository()
        .select_all_web_sites_paginated(query.into())
        .await
        .map(PaginatedWebSiteResponse::from)
        .map(Json)
}
