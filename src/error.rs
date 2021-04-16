//! Custom error implementation.

use thiserror::Error;

/// Application error.
#[derive(Error, Debug)]
pub enum ReproCheckError {
    /// Error that may occur when processing a request.
    #[error("failed to send request: `{0}`")]
    RequestError(#[from] reqwest::Error),
    /// Unknown error.
    #[error("unknown error")]
    Unknown,
}
