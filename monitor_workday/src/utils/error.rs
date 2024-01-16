use thiserror::Error;

pub type Result<T> = anyhow::Result<T, MonitorError>;

#[derive(Error, Debug)]
pub enum MonitorError {
    // Add more error types as needed
    #[error("An internal error occurred. Please try again later.")]
    InternalError,
    #[error("An error occured while sending the notification: {0}")]
    NotificationError(#[from] notify_rust::error::Error),
}
