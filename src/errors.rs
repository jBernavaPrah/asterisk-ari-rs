use reqwest::header::InvalidHeaderValue;
use reqwest::Error as ReqwError;
use reqwest::StatusCode;
use std::result;
use tokio_tungstenite::tungstenite::Error as WSError;
use url::ParseError;

/// Represents various errors that can occur in the ARI client.
#[derive(Debug)]
pub enum AriError {
    /// Error during JSON serialization/deserialization.
    Serde(serde_json::Error),
    /// Error converting a byte array to a UTF-8 string.
    Utf8(std::string::FromUtf8Error),
    /// API-specific error.
    Api(ApiError),
    /// Invalid HTTP header value.
    HttpInvalidHeader(InvalidHeaderValue),
    /// General HTTP error.
    Http(ReqwError),
    /// URL parsing error.
    UrlParse(ParseError),
    /// WebSocket error.
    Websocket(WSError),
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

impl From<serde_json::Error> for AriError {
    /// Converts a `serde_json::Error` into an `AriError`.
    ///
    /// # Arguments
    ///
    /// * `e` - The `serde_json::Error` to convert.
    ///
    /// # Returns
    ///
    /// An `AriError` representing the JSON error.
    fn from(e: serde_json::Error) -> Self {
        AriError::Serde(e)
    }
}

impl From<std::string::FromUtf8Error> for AriError {
    /// Converts a `std::string::FromUtf8Error` into an `AriError`.
    ///
    /// # Arguments
    ///
    /// * `e` - The `std::string::FromUtf8Error` to convert.
    ///
    /// # Returns
    ///
    /// An `AriError` representing the UTF-8 conversion error.
    fn from(e: std::string::FromUtf8Error) -> Self {
        AriError::Utf8(e)
    }
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

impl From<InvalidHeaderValue> for AriError {
    /// Converts an `InvalidHeaderValue` into an `AriError`.
    ///
    /// # Arguments
    ///
    /// * `e` - The `InvalidHeaderValue` to convert.
    ///
    /// # Returns
    ///
    /// An `AriError` representing the invalid HTTP header value error.
    fn from(e: InvalidHeaderValue) -> Self {
        AriError::HttpInvalidHeader(e)
    }
}

impl From<ReqwError> for AriError {
    /// Converts a `ReqwError` into an `AriError`.
    ///
    /// # Arguments
    ///
    /// * `e` - The `ReqwError` to convert.
    ///
    /// # Returns
    ///
    /// An `AriError` representing the HTTP error.
    fn from(e: ReqwError) -> Self {
        AriError::Http(e)
    }
}