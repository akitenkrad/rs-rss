use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    Unprocessable(String),
    #[error("{0}")]
    InternalServerError(String),

    // from anyhow
    #[error("Error: {0}")]
    AnyhowError(#[from] anyhow::Error),

    // from database errors
    #[error("{0}")]
    EntityNotFound(String),
    #[error("{0}")]
    ValidationError(#[from] garde::Report),
    #[error("Database Error: {0}")]
    DatabaseError(String),
    #[error("Database Error: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("Database Error - record not found: {0}")]
    RecordNotFound(#[source] sqlx::Error),
    #[error("Database Error - sqlx core error: {0}")]
    SqlxCoreError(#[source] sqlx_core::error::Error),

    // from redis errors
    #[error("Redis Error: {0}")]
    RedisError(#[from] redis::RedisError),

    // from uuid errors
    #[error("Uuid Error: {0}")]
    ConvertToUuidError(#[from] uuid::Error),

    // from crawler errors
    #[error("RssParser Error: {0}")]
    RssParseError(#[source] feed_parser::parsers::errors::ParseError),

    // from request errors
    #[error("Request Error: {0}")]
    RequestError(#[from] request::Error),
    #[error("Request Error - parse error: {0}")]
    ParseError(#[from] url::ParseError),

    // from serde errors
    #[error("Json Parse Error: {0}")]
    JsonParseError(#[from] serde_json::Error),

    // from scrape errors
    #[error("Scrape Error: {0}")]
    ScrapeError(String),
}

fn app_error_to_status_code(error: &AppError) -> StatusCode {
    match error {
        AppError::Unprocessable(_) => StatusCode::UNPROCESSABLE_ENTITY,
        AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        AppError::AnyhowError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        AppError::EntityNotFound(_) => StatusCode::NOT_FOUND,
        AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
        AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        AppError::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        AppError::RecordNotFound(_) => StatusCode::NOT_FOUND,
        AppError::SqlxCoreError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        AppError::RedisError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        AppError::ConvertToUuidError(_) => StatusCode::BAD_REQUEST,
        AppError::RssParseError(_) => StatusCode::BAD_REQUEST,
        AppError::RequestError(_) => StatusCode::BAD_REQUEST,
        AppError::ParseError(_) => StatusCode::BAD_REQUEST,
        AppError::JsonParseError(_) => StatusCode::BAD_REQUEST,
        AppError::ScrapeError(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status_code = app_error_to_status_code(&self);
        status_code.into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
