use reqwest::header::{HeaderMap, HeaderValue};
use secrecy::Secret;

use crate::kite::connect::credentials::KiteCredentials;
use crate::kite::error::Result;

/// `manja::kite::connect::Client` uses this trait for every REST API call on
/// KiteConnect
pub trait KiteConfig {
    fn headers(&self, access_token: Option<Secret<String>>) -> HeaderMap;
    fn url(&self, path: &str) -> String;
    // fn query(&self) -> Vec<(&str, &str)>;
    fn api_base(&self) -> &str;
    fn api_login(&self) -> &str;
    fn api_redirect(&self) -> &str;
    fn credentials(&self) -> &KiteCredentials;
}

pub trait KiteLoginFlow {
    /// Generates a request token using the provided login function.
    ///
    /// # Arguments
    ///
    /// * `login` - A closure that takes a reference to a type implementing the `Config` trait
    ///   and returns a `Result` containing the request token as a `String`.
    ///
    /// # Returns
    ///
    /// A `Result` containing the request token as a `String` if successful, or an error otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use manja::kite::error::Result;
    /// use manja::kite::connect::config::Config;
    ///
    /// struct MySession;
    /// struct MyConfig;
    ///
    /// impl Config for MyConfig {}
    ///
    /// impl LoginFlow for MySession {
    ///     fn generate_request_token<F, C>(&self, login: F) -> Result<String>
    ///     where
    ///         F: Fn(&C) -> Result<String>,
    ///         C: Config,
    ///     {
    ///         let config = MyConfig;
    ///         login(&config)
    ///     }
    /// }
    ///
    /// let session = MySession;
    /// let token_result = session.generate_request_token(|_config| Ok("request_token".to_string()));
    /// assert!(token_result.is_ok());
    /// ```
    fn generate_request_token<F>(&self, login: F) -> Result<String>
    where
        F: Fn(&dyn KiteConfig) -> Result<String>;
}

pub trait KiteAuth {
    /// Adds the `Authorization` header as per Kite's official documentation
    /// on [signing HTTP requests](https://kite.trade/docs/connect/v3/user/#signing-requests).
    fn add_auth_header(&mut self, api_key: String, access_token: String) {}
}

impl KiteAuth for HeaderMap {
    fn add_auth_header(&mut self, api_key: String, access_token: String) {
        if let Ok(value) =
            HeaderValue::from_str(format!("token {}:{}", api_key, access_token).as_ref())
        {
            // Once the authentication is complete, all requests should be
            // signed with the HTTP `Authorization` header with `token` as
            // the authorization scheme, followed by a space, and then the
            // `api_key:access_token` combination.
            self.insert("Authorization", value);
        }
        ()
    }
}
