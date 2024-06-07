//! KiteConnect client configurations
use reqwest::header::{HeaderMap, HeaderValue};
use secrecy::{ExposeSecret, Secret};

use crate::kite::connect::credentials::KiteCredentials;
use crate::kite::traits::{KiteAuth, KiteConfig};

/// Default v3 API base url
pub const KITECONNECT_API_BASE: &str = "https://api.kite.trade";

/// Default v3 API login url
pub const KITECONNECT_API_LOGIN: &str = "https://kite.trade/connect/login";

/// Default KiteConnect redirect url
pub const KITECONNECT_API_REDIRECT: &str = "https://127.0.0.1/kite-redirect?";

/// `manja::kite::connect::Client` uses this trait for every REST API call on
/// KiteConnect
// pub trait Config {
//     fn headers(&self, access_token: Option<Secret<String>>) -> HeaderMap;
//     fn url(&self, path: &str) -> String;
//     fn query(&self) -> Vec<(&str, &str)>;
// }

/// KiteConnect client configurations
#[derive(Clone, Debug)]
pub struct Config {
    api_base: String,
    api_login: String,
    api_redirect: String,
    credentials: KiteCredentials,
}

impl Default for Config {
    /// Default implementation of `KiteConfig` picks up values from environment
    /// variables
    fn default() -> Self {
        Self {
            api_base: std::env::var("KITECONNECT_API_BASE")
                .unwrap_or_else(|_| KITECONNECT_API_BASE.to_string())
                .into(),
            api_login: std::env::var("KITECONNECT_API_LOGIN")
                .unwrap_or_else(|_| KITECONNECT_API_LOGIN.to_string())
                .into(),
            api_redirect: std::env::var("KITECONNECT_API_REDIRECT")
                .unwrap_or_else(|_| KITECONNECT_API_REDIRECT.to_string())
                .into(),
            credentials: KiteCredentials::load_from_env(),
        }
    }
}

impl KiteConfig for Config {
    fn headers(&self, access_token: Option<Secret<String>>) -> HeaderMap {
        let mut headers = HeaderMap::new();
        // NOTE: `KiteConfig` currently points to v3 of KiteConnect API.
        // This could be made configurable if Zerodha announces any breaking changes
        // to the API.
        headers.insert("X-Kite-Version", HeaderValue::from_static("3"));
        if let Some(access_token) = access_token {
            headers.add_auth_header(
                self.credentials.api_key().expose_secret().clone(),
                access_token.expose_secret().clone(),
            )
        }
        headers
    }

    /// Construct a URL endpoint given a path.
    ///
    /// NOTE: `path` should have a leading backslash.
    fn url(&self, path: &str) -> String {
        format!("{}{}", self.api_base, path)
    }

    fn api_base(&self) -> &str {
        self.api_base.as_str()
    }

    fn api_login(&self) -> &str {
        self.api_login.as_str()
    }

    fn api_redirect(&self) -> &str {
        self.api_redirect.as_str()
    }

    fn credentials(&self) -> &KiteCredentials {
        &self.credentials
    }
}

impl Config {
    /// Constructor for `Config`.
    ///
    pub fn from_parts<InS>(
        api_base: InS,
        api_login: InS,
        api_redirect: InS,
        credentials: KiteCredentials,
    ) -> Self
    where
        InS: Into<String>,
    {
        Self {
            api_base: api_base.into(),
            api_login: api_login.into(),
            api_redirect: api_redirect.into(),
            credentials,
        }
    }
}
