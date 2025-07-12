use crate::models::health::HealthResponse;
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use registry::AppRegistry;
use shared::errors::AppResult;

pub async fn health_check() -> AppResult<Json<HealthResponse>> {
    Ok(Json(HealthResponse::from(StatusCode::OK)))
}

pub async fn health_check_db(State(registry): State<AppRegistry>) -> AppResult<Json<HealthResponse>> {
    if registry.health_check_repository().check_db().await {
        Ok(Json(HealthResponse::from(StatusCode::OK)))
    } else {
        Ok(Json(HealthResponse::from(StatusCode::INTERNAL_SERVER_ERROR)))
    }
}
