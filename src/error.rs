//! Custom error implementation.

use thiserror::Error;

/// Application error.
#[derive(Error, Debug)]
pub enum ReproStatusError {
    /// Error that may occur while I/O operations such as Read, Write and Seek.
    #[error("IO error: `{0}`")]
    IoError(#[from] std::io::Error),
    /// Error that may occur while ALPM operations.
    #[error("ALPM error: `{0}`")]
    AlpmError(#[from] alpm::Error),
    /// Error that may occur when processing a request.
    #[error("failed to send request: `{0}`")]
    RequestError(#[from] reqwest::Error),
    /// Error that may occur while handling Ctrl-C signals.
    #[error("Ctrl-C error: `{0}`")]
    SignalError(#[from] ctrlc::Error),
    /// Unknown error.
    #[error("unknown error")]
    Unknown,
}
