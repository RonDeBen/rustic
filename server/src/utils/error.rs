use sqlx::Error as SqlxError;
use thiserror::Error;

pub type Result<T> = anyhow::Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    // TODO: don't expose these errors to the user
    #[error("Database error: {0}")]
    DatabaseError(#[from] SqlxError),
    // Add more error types as needed
    #[error("An internal error occurred. Please try again later.")]
    InternalError,
    #[error("Weekends are currently not supported")]
    WeekendError,
}

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::DatabaseError(e) => {
                // TODO: don't expose these errors to the user
                let error_message = format!("Database error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
            }
            // Handle other errors as needed
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response(),
        }
    }
}
