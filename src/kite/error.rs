use fantoccini::error::CmdError;
use fantoccini::error::NewSessionError;
use reqwest::header::InvalidHeaderValue;
use std::env::VarError;
use thiserror::Error;

/// A `Result` alias where the `Err` case is `manja::kite::Error`.
pub type Result<T> = std::result::Result<T, ManjaError>;

/// Custom enum that contains all the possible errors that may occur when using
/// [`manja`].
// TODO: Rationalize.
#[derive(Error, Debug)]
pub enum ManjaError {
    #[error("Environment variable error: {0}")]
    EnvVarError(#[from] VarError),

    #[error("Invalid header value: {0}")]
    InvalidHeaderValueError(#[from] InvalidHeaderValue),

    #[error("WebDriver new session error: {0}")]
    WebDriverNewSessionError(#[from] NewSessionError),

    #[error("WebDriver error: {0}")]
    WebDriverError(#[from] CmdError),

    #[error("KiteConnect API JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    // TODO: Refactor?
    #[error("IO timeout error: {0}")]
    IoTimeoutError(String),

    // The request couldn't be completed because there was an error when trying
    // to do so.
    #[error("Request: {0}")]
    Client(#[from] reqwest::Error),

    // TOTP error.
    #[error("TOTP error: {0}")]
    TotpError(String),

    /// The request was made, but the server returned an unsuccessful status
    /// code, such as 404 or 503. In some cases, the response may contain a
    /// custom message from Zerodha API with more information, which can be
    /// serialized into `ApiError`.
    #[error("Status code: {}", reqwest::Response::status(.0))]
    StatusCode(reqwest::Response),
}

impl From<String> for ManjaError {
    fn from(error: String) -> Self {
        ManjaError::IoTimeoutError(error)
    }
}

impl From<&str> for ManjaError {
    fn from(error: &str) -> Self {
        ManjaError::IoTimeoutError(error.to_string())
    }
}
