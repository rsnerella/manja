use std::env::VarError;
use std::fmt;

use fantoccini::error::CmdError;
use fantoccini::error::NewSessionError;
use reqwest::header::InvalidHeaderValue;
use serde::Deserialize;

/// A `Result` alias where the `Err` case is `manja::kite::Error`.
pub type Result<T> = std::result::Result<T, ManjaError>;

/// Custom enum that contains all the possible errors that may occur when using
/// [`manja`].
// TODO: Rationalize.
#[derive(Debug, thiserror::Error)]
pub enum ManjaError {
    #[error("KiteConnect API error: {0}")]
    KiteApiError(KiteApiError),

    #[error("Environment variable error: {0}")]
    EnvVarError(#[from] VarError),

    #[error("Invalid header value: {0}")]
    InvalidHeaderValueError(#[from] InvalidHeaderValue),

    // TODO: Refactor away to `manja-webdriver` crate
    #[error("WebDriver new session error: {0}")]
    WebDriverNewSessionError(#[from] NewSessionError),

    // TODO: Refactor away to `manja-webdriver` crate
    #[error("WebDriver error: {0}")]
    WebDriverError(#[from] CmdError),

    #[error("JSON deserialization error: {0}")]
    JSONDeserialize(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    // The request couldn't be completed because there was an error when trying
    // to do so.
    #[error("HTTP error: {0}")]
    Reqwest(#[from] reqwest::Error),

    // TODO: Refactor away to `manja-webdriver` crate
    // TOTP error.
    #[error("TOTP error: {0}")]
    TotpError(String),

    // Internal manja errors
    #[error("Internal `manja` error: {0}")]
    Internal(String),
}

impl From<&str> for ManjaError {
    fn from(value: &str) -> Self {
        ManjaError::Internal(value.to_string())
    }
}


#[derive(Debug, Deserialize)]
pub(crate) struct KiteApiError {
    pub endpoint: String,
    pub status_code: u16,
    pub message: Option<String>,
    pub error_type: KiteApiException,
}

// TODO: Fix this.
impl fmt::Display for KiteApiError { 
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.message)
    }
}



/// Enum representing various types of errors that can occur while interacting
///  with the Kite API.
#[derive(Debug, Deserialize)]
pub enum KiteApiException {
    /// Preceded by a 403 header, this indicates the expiry or invalidation of
    /// an authenticated session. This can be caused by the user logging out,
    /// a natural expiry, or the user logging into another Kite instance.
    /// When you encounter this error, you should clear the user's session and
    /// re-initiate a login.
    TokenException,

    /// Represents user account related errors.
    UserException,

    /// Represents order related errors such as placement failures or a corrupt
    /// fetch.
    OrderException,

    /// Represents missing required fields or bad values for parameters.
    InputException,

    /// Represents insufficient funds required for order placement.
    MarginException,

    /// Represents insufficient holdings available to place a sell order for a
    /// specified instrument.
    HoldingException,

    /// Represents a network error where the API was unable to communicate
    /// with the Order Management System (OMS).
    NetworkException,

    /// Represents an internal system error where the API was unable to
    /// understand the response from the OMS to respond to a request.
    DataException,

    /// Represents an unclassified error. This should only happen rarely.
    GeneralException,

    /// Represents a deserialization error from a KiteConnect API response.
    /// This error indicates that the KiteConnect API has been updated with
    /// a new `error_type`.
    DeserializationException(String),
}


impl From<&str> for KiteApiException {
    fn from(s: &str) -> Self {
        match s {
            "TokenException" => KiteApiException::TokenException,
            "UserException" => KiteApiException::UserException,
            "OrderException" => KiteApiException::OrderException,
            "InputException" => KiteApiException::InputException,
            "MarginException" => KiteApiException::MarginException,
            "HoldingException" => KiteApiException::HoldingException,
            "NetworkException" => KiteApiException::NetworkException,
            "DataException" => KiteApiException::DataException,
            "GeneralException" => KiteApiException::GeneralException,
            _ => KiteApiException::DeserializationException(s.to_string()), // Handle unknown input
        }
    }
}

// TODO: Deprecated. Delete.
// impl Default for KiteApiException {
//     fn default() -> Self {
//         KiteApiException::GeneralException
//     }
// }

impl fmt::Display for KiteApiException {
    #[allow(deprecated)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            KiteApiException::TokenException => write!(
                f,
                "TokenException: indicates the expiry or invalidation of an authenticated session"
            ),
            KiteApiException::UserException => write!(
                f, 
                "UserException: represents user account related errors"
            ),
            KiteApiException::OrderException => write!(
                f, 
                "OrderException: represents order related errors such as placement failures or a corrupt fetch"
            ),
            KiteApiException::InputException => write!(
                f, 
                "InputException: represents missing required fields or bad values for parameters"
            ),
            KiteApiException::MarginException => write!(
                f, 
                "MarginException: represents insufficient funds required for order placement"
            ),
            KiteApiException::HoldingException => write!(
                f, 
                "HoldingException: represents insufficient holdings available to place a sell order for a specified instrument"
            ),
            KiteApiException::NetworkException => write!(
                f, 
                "NetworkException: represents a network error where the API was unable to communicate with the Order Management System (OMS)"
            ),
            KiteApiException::DataException => write!(
                f, 
                "DataException: represents an internal system error where the API was unable to understand the response from the OMS to respond to a request"
            ),
            KiteApiException::GeneralException => write!(
                f, 
                "GeneralException: represents an unclassified error"
            ),
            KiteApiException::DeserializationException(path) => write!(
                f,
                "DeserializationException: represents a JSON deserialization error of a response from a KiteConnect API endpoint (`{}`).",
                path
            )
        }
    }
}


pub(crate) fn map_deserialization_error(e: serde_json::Error, json_str: &str) -> ManjaError {
    tracing::error!("failed deserialization of: {}", json_str);
    ManjaError::JSONDeserialize(e)
}
