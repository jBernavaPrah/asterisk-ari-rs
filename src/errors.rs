use reqwest::Error as ReqwError;
use reqwest::StatusCode;
use std::error::Error;
use std::fmt::Display;
use std::fmt::{Formatter, Result as FmtResult};
use std::result;
use std::sync::{MutexGuard, PoisonError};
use thiserror::Error;
use tokio_tungstenite::tungstenite::Error as WSError;
use url::ParseError;

/// Represents various errors that can occur in the ARI client.
#[derive(Debug, Error)]
pub enum AriError {
    /// Error during JSON serialization/deserialization.
    #[error("serde error: {0}")]
    Serde(#[from] serde_json::Error),
    /// Error converting a byte array to a UTF-8 string.
    #[error("Conversion error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    /// API-specific error.
    #[error("Api Error: {0}")]
    Api(ApiError),
    /// General HTTP error.
    #[error("HTTP error: {raw} - body: {body}")]
    Http {
        /// The HTTP error.
        raw: ReqwError,
        /// The response body.
        body: String,
    },
    /// URL parsing error.
    #[error("URL parse error: {0}")]
    UrlParse(ParseError),
    /// WebSocket error.
    #[error("WebSocket error: {0}")]
    Websocket(WSError),
    /// Internal error.
    #[error("Internal error: {0}")]
    Internal(String),
}

impl AriError {
    /// Creates a new `AriError` with the given status code and optional content.
    ///
    /// # Arguments
    ///
    /// * `code` - The HTTP status code.
    /// * `content` - Optional content associated with the error.
    ///
    /// # Returns
    ///
    /// A new instance of `AriError`.
    pub fn new(code: StatusCode, content: Option<String>) -> Self {
        AriError::Api(ApiError { code, content })
    }
}

/// Result type alias for operations that can return an `AriError`.
pub type Result<T> = result::Result<T, AriError>;

/// Represents an API-specific error.
#[derive(Debug)]
pub struct ApiError {
    /// The HTTP status code associated with the error.
    pub code: StatusCode,
    /// Optional content associated with the error.
    pub content: Option<String>,
}

impl From<ParseError> for AriError {
    /// Converts a `ParseError` into an `AriError`.
    ///
    /// # Arguments
    ///
    /// * `e` - The `ParseError` to convert.
    ///
    /// # Returns
    ///
    /// An `AriError` representing the URL parsing error.
    fn from(e: ParseError) -> Self {
        AriError::UrlParse(e)
    }
}

impl From<WSError> for AriError {
    /// Converts a `WSError` into an `AriError`.
    ///
    /// # Arguments
    ///
    /// * `e` - The `WSError` to convert.
    ///
    /// # Returns
    ///
    /// An `AriError` representing the WebSocket error.
    fn from(e: WSError) -> Self {
        AriError::Websocket(e)
    }
}

impl<T: std::fmt::Display> From<tokio::sync::MutexGuard<'_, T>> for AriError {
    /// Converts a `tokio::sync::MutexGuard<'_, T>` into an `AriError`.
    ///
    /// # Arguments
    ///
    /// * `e` - The `tokio::sync::MutexGuard<'_, T>` to convert.
    ///
    /// # Returns
    ///
    /// An `AriError` representing the poisoned mutex error.
    fn from(e: tokio::sync::MutexGuard<'_, T>) -> Self {
        AriError::Internal(format!("{}", e))
    }
}

impl<T> From<PoisonError<MutexGuard<'_, T>>> for AriError {
    /// Converts a `PoisonError<MutexGuard<'_, T>>` into an `AriError`.
    ///
    /// # Arguments
    ///
    /// * `e` - The `PoisonError<MutexGuard<'_, T>>` to convert.
    ///
    /// # Returns
    ///
    /// An `AriError` representing the poisoned mutex error.
    fn from(e: PoisonError<MutexGuard<'_, T>>) -> Self {
        AriError::Internal(format!("{}", e))
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.content {
            Some(content) => write!(f, "API error (status {}): {}", self.code, content),
            None => write!(f, "API error (status {})", self.code),
        }
    }
}

impl Error for ApiError {}
