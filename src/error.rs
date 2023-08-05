use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Post not found")]
    NotFound,
    #[error("Failed to connect to database")]
    ConnectionError,
    #[error("Database error: {0}")]
    DatabaseError(String),
}

impl From<r2d2::Error> for ApiError {
    fn from(_: r2d2::Error) -> Self {
        ApiError::ConnectionError
    }
}

impl From<rusqlite::Error> for ApiError {
    fn from(err: rusqlite::Error) -> Self {
        match err {
            rusqlite::Error::QueryReturnedNoRows => ApiError::NotFound,
            _ => ApiError::DatabaseError(err.to_string()),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::NotFound => HttpResponse::NotFound().finish(),
            ApiError::ConnectionError | ApiError::DatabaseError(_) => {
                HttpResponse::ServiceUnavailable().finish()
            }
        }
    }
}
