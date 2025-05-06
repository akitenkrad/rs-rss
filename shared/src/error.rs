use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    // Database errors
    #[error("Error has occurred while processing the request: {0}")]
    DatabaseError(#[source] sqlx::Error),
    #[error("Record not found: {0}")]
    RecordNotFound(#[source] sqlx::Error),
    // Uuid errors
    #[error("{0}")]
    ConvertToUuidError(#[from] uuid::Error),
}

pub type AppResult<T> = Result<T, AppError>;
