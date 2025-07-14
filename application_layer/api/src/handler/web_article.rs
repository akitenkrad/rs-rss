use crate::models::web_article::{
    PaginatedWebArticleResponse, PaginatedWebSiteResponse, WebArticleListQuery, WebSiteListQuery,
};
use axum::extract::{Json, Query, State};
use garde::Validate;
use registry::AppRegistry;
use shared::errors::AppResult;

pub async fn select_paginated_web_sites(
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

pub async fn select_paginated_web_articles(
    State(registry): State<AppRegistry>,
    Query(query): Query<WebArticleListQuery>,
) -> AppResult<Json<PaginatedWebArticleResponse>> {
    query.validate()?;

    registry
        .web_article_repository()
        .select_paginated_web_articles(query.into())
        .await
        .map(PaginatedWebArticleResponse::from)
        .map(Json)
}
