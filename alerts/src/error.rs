use thiserror::Error;

pub type Result<T> = anyhow::Result<T, AlertError>;

#[derive(Error, Debug)]
pub enum AlertError {
    // Add more error types as needed
    #[error("An internal error occurred. Please try again later.")]
    InternalError,
    #[error("An error occured while sending the notification: {0}")]
    NotificationError(#[from] notify_rust::error::Error),
    #[error("An error occured while making a request to the server: {0}")]
    ReqwestError(#[from] reqwest::Error),
}
