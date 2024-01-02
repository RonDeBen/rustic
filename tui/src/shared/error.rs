use reqwest::Error as ReqwestError;
use thiserror::Error;

pub type ClientResult<T> = Result<T, ClientError>;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Network error")]
    NetworkError(#[from] ReqwestError),
    #[error("Failed to parse server response")]
    ParseError(#[from] serde_json::Error),
    #[error("Server returned an error: {0}")]
    ServerError(String),
}
