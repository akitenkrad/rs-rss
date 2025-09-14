use crate::models::web_article::{
    PaginatedWebArticleResponse, PaginatedWebSiteResponse, WebArticleFilteredRequest, WebArticleListRequest,
    WebArticleResponse, WebArticleUpdateRequest, WebSiteListRequest,
};
use axum::extract::{Json, Query, State};
use garde::Validate;
use kernel::models::web_article::Status;
use registry::AppRegistry;
use shared::errors::{AppError, AppResult};
use std::str::FromStr;

pub async fn select_paginated_web_sites(
    State(registry): State<AppRegistry>,
    Query(query): Query<WebSiteListRequest>,
) -> AppResult<Json<PaginatedWebSiteResponse>> {
    registry
        .web_site_repository()
        .select_all_web_sites_paginated(query.into())
        .await
        .map(PaginatedWebSiteResponse::from)
        .map(Json)
}

pub async fn select_paginated_web_articles(
    State(registry): State<AppRegistry>,
    Query(query): Query<WebArticleListRequest>,
) -> AppResult<Json<PaginatedWebArticleResponse>> {
    query.validate()?;

    registry
        .web_article_repository()
        .select_paginated_web_articles(query.into())
        .await
        .map(PaginatedWebArticleResponse::from)
        .map(Json)
}

pub async fn select_filtered_web_articles(
    State(registry): State<AppRegistry>,
    Query(query): Query<WebArticleFilteredRequest>,
) -> AppResult<Json<PaginatedWebArticleResponse>> {
    registry
        .web_article_repository()
        .select_filtered_web_articles(query.into())
        .await
        .map(PaginatedWebArticleResponse::from)
        .map(Json)
}

pub async fn update_web_article_status(
    State(registry): State<AppRegistry>,
    Json(payload): Json<WebArticleUpdateRequest>,
) -> AppResult<Json<WebArticleResponse>> {
    registry
        .web_article_repository()
        .update_web_article_state(
            payload.article_id,
            Status::from_str(&payload.new_status)
                .map_err(|e| AppError::EnumParseError(format!("Invalid status: {}", e)))?,
        )
        .await
        .map(WebArticleResponse::from)
        .map(Json)
}
