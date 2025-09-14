use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub message: String,
    pub status_code: usize,
}

impl From<StatusCode> for HealthResponse {
    fn from(status: StatusCode) -> Self {
        if status.is_success() {
            return HealthResponse {
                message: "OK".to_string(),
                status_code: status.as_u16() as usize,
            };
        } else {
            return HealthResponse {
                message: "Error".to_string(),
                status_code: status.as_u16() as usize,
            };
        }
    }
}
