//! Asynchronous KiteConnect client

use reqwest::header::{HeaderMap, HeaderValue};
use std::collections::HashMap;

use crate::kite::connect::client_builder::KiteConnectClientBuilder;
use crate::kite::connect::config::KiteConfig;
use crate::kite::connect::credentials::KiteCredentials;
use crate::kite::connect::models::{user::UserSession, KiteApiResponse};
use crate::kite::connect::utils::create_checksum;
use crate::kite::error::{ManjaError, Result};
use crate::kite::login::flow::login_flow;

/// An asynchronous `KiteConnectClient` to make HTTP requests with.
///
/// `KiteConnectClient` is a wrapper over `reqwest::Client` which holds a connection
/// pool internally. It is advisable to create one and **reuse** it.
///
/// You do **not** have to wrap `KiteConnectClient` in an [`Rc`] or [`Arc`] to
/// **reuse** it because the `reqwest::Client` used internally already uses an
/// [`Arc`].
#[derive(Clone)]
pub struct KiteConnectClient {
    /// A reqwest client instance
    client: reqwest::Client,
    /// User credentials to access `KiteConnect` trading API
    kite_credentials: KiteCredentials,
    /// Kite config
    kite_config: KiteConfig,
    /// User session
    session: Option<UserSession>,
}

impl KiteConnectClient {
    /// Construct a `KiteConnectClient` using default configuration.
    ///
    /// Sample use:
    /// ```no_run
    /// use manja::kite::connect::credentials::KiteCredentials;
    /// use manja::kite::connect::client::KiteConnectClient;
    ///
    /// let credentials = KiteCredentials::load_from_env().expect("User credentials not found!");
    /// let client = KiteConnectClient::default_with_credentials(credentials);
    /// ```
    pub fn default_with_credentials(credentials: KiteCredentials) -> Result<Self> {
        KiteConnectClientBuilder::default()
            .with_credentials(credentials)
            .build()
    }

    /// Internal function to construct a `KiteConnectClient` from parts.
    ///
    pub fn from_parts(
        client: reqwest::Client,
        kite_credentials: KiteCredentials,
        kite_config: KiteConfig,
    ) -> Self {
        Self {
            client,
            kite_credentials,
            kite_config,
            session: None,
        }
    }

    /// HTTP header for accessing trading API version 3
    pub fn api_v3_headers(&self) -> HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("X-Kite-Version", HeaderValue::from_static("3"));
        headers
    }

    pub fn add_auth_token_header(&self, headers: &mut HeaderMap) -> Result<()> {
        match self.session {
            Some(ref session) => {
                headers.insert(
                    "Authorization",
                    HeaderValue::from_str(&format!(
                        "token {:?}:{}",
                        self.kite_credentials.api_key(),
                        session.access_token
                    ))?,
                );
                Ok(())
            }
            None => Err(format!("No valid user session.").into()),
        }
    }

    /// Generate a `request token` which can be used to generate an `access token`.
    ///
    /// This is the first half of the [login flow](https://kite.trade/docs/connect/v3/user/#login-flow).
    pub async fn generate_request_token(&self) -> Result<String> {
        login_flow(
            &self.kite_config.base_url_login,
            &self.kite_config.redirect_url,
            &self.kite_credentials,
        )
        .await
    }

    /// Generate a session using a `request token`.
    ///
    /// This is the second half of the [login flow](https://kite.trade/docs/connect/v3/user/#login-flow).
    ///
    pub async fn generate_session(
        mut self,
        request_token: &str,
    ) -> Result<KiteApiResponse<UserSession>> {
        // Extract credentials
        let api_key = self.kite_credentials.api_key();
        let api_secret = self.kite_credentials.api_secret();
        // Construct session token URL
        let mut session_token_url = self.kite_config.base_url_api.clone();
        session_token_url.set_path("/session/token");
        // Define the headers
        let headers = self.api_v3_headers();
        // Construct request payload
        let checksum = create_checksum(api_key.as_str(), request_token, api_secret.as_str());
        let mut params = HashMap::new();
        params.insert("api_key", api_key.as_str());
        params.insert("request_token", request_token);
        params.insert("checksum", checksum.as_str());

        // Send the POST request
        match self
            .client
            .post(session_token_url.as_str())
            .headers(headers)
            .form(&params)
            .send()
            .await
        {
            Ok(response) => {
                match response.text().await {
                    Ok(json_response) => {
                        // Attempt to parse into a Kite API response with a `UsesSession` object
                        let kite_response =
                            serde_json::from_str::<KiteApiResponse<UserSession>>(&json_response)?;
                        self.session = kite_response.data.clone();
                        Ok(kite_response)
                    }
                    Err(e) => Err(ManjaError::from(e)),
                }
            }
            Err(e) => Err(ManjaError::from(e)),
        }
    }
}
