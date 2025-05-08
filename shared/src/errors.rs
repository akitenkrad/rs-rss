use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    // from anyhow
    #[error("Error: {0}")]
    AnyhowError(#[from] anyhow::Error),
    // from database errors
    #[error("Database Error: {0}")]
    DatabaseError(#[source] sqlx::Error),
    #[error("Database Error - record not found: {0}")]
    RecordNotFound(#[source] sqlx::Error),
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

pub type AppResult<T> = Result<T, AppError>;
