use sqlx::Error as SqlxError;
use thiserror::Error;

pub type Result<T> = anyhow::Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error")]
    DatabaseError(#[from] SqlxError),

    // Add more error types as needed
    #[error("An internal error occurred. Please try again later.")]
    InternalError,
}

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::DatabaseError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
            }
            // Handle other errors as needed
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response(),
        }
    }
}
