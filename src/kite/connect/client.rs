//! Asynchronous KiteConnect client
use std::time::Duration;

use reqwest::Request;
use secrecy::{ExposeSecret, Secret};
use serde::{de::DeserializeOwned, Serialize};
use tracing::info;

use crate::kite::connect::{
    api::{Session, User},
    config::Config,
    models::{KiteApiResponse, UserSession},
};
use crate::kite::error::Result;
use crate::kite::traits::KiteConfig;

use super::credentials;

/// An asynchronous Kite Connect client to make HTTP requests with.
///
/// `Client` is a wrapper over `reqwest::Client` which holds a connection
/// pool internally. It is advisable to create one and **reuse** it.
///
/// You do **not** have to wrap `KiteConnectClient` in an [`Rc`] or [`Arc`] to
/// **reuse** it because the `reqwest::Client` used internally already uses an
/// [`Arc`].
#[derive(Clone)]
pub struct HTTPClient {
    client: reqwest::Client,
    config: Config,
    backoff: backoff::ExponentialBackoff,
    session: Option<UserSession>,
}

impl Default for HTTPClient {
    fn default() -> Self {
        Self {
            // Default timeout for I/O operations: 10 seconds
            client: Self::default_reqwest_client(10),
            // Default config parameters are loaded from environment variables
            config: Config::default(),
            backoff: Default::default(),
            session: None,
        }
    }
}

impl HTTPClient {
    // Default `reqwest::Client` with timeout for I/O operations
    fn default_reqwest_client(timeout_seconds: u64) -> reqwest::Client {
        reqwest::ClientBuilder::new()
            .timeout(Duration::from_secs(timeout_seconds))
            .build()
            // This should not fail. Fallback to default `reqwest::Client`.
            .unwrap_or_else(|_| reqwest::Client::new())
    }

    fn get_access_token(&self) -> Option<Secret<String>> {
        // Clone and return the access token, if available
        match self.session {
            Some(ref user_session) => Some((user_session.access_token).clone()),
            None => None,
        }
    }

    /// Create a default HTTP client with config.
    pub fn with_config(config: Config) -> Self {
        Self {
            // Default timeout for I/O operations: 10 seconds
            client: Self::default_reqwest_client(10),
            config,
            backoff: Default::default(),
            session: None,
        }
    }

    /// Exponential backoff for retrying [rate limited](https://kite.trade/docs/connect/v3/exceptions/#api-rate-limit) requests.
    pub fn with_backoff(mut self, backoff: backoff::ExponentialBackoff) -> Self {
        self.backoff = backoff;
        self
    }

    /// Add `UserSession` to the `HTTPClient`
    pub fn with_user_session(mut self, user_session: UserSession) -> Self {
        self.session = Some(user_session);
        self
    }

    pub fn set_user_session(&mut self, user_session: Option<UserSession>) {
        self.session = user_session;
        ()
    }

    /// User session, if it exists.
    pub fn user_session(&self) -> Option<&UserSession> {
        self.session.as_ref()
    }

    /// HTTP configurations and Kite user credentials.
    pub fn http_config(&self) -> &Config {
        &self.config
    }

    /// Reqwest HTTP Client.
    pub fn http_client(&self) -> &reqwest::Client {
        &self.client
    }

    // --- [ API Groups ] ---

    /// To call [User] related APIs using this client.
    pub fn user(&self) -> User {
        User::new(self)
    }

    /// To call [Session] related APIs using this client.
    pub fn session(&mut self) -> Session {
        Session::new(self)
    }

    // --- [ HTTP verb functions ] ---

    /// Make a GET request to {path} and deserialize the response body
    pub(crate) async fn get<Model>(&self, path: &str) -> Result<KiteApiResponse<Model>>
    where
        Model: DeserializeOwned,
    {
        let http_request = self
            .http_client()
            .get(self.config.url(path))
            // .query(&self.config.query())
            // Fetch access token for protected endpoints, if available
            .headers(self.config.headers(self.get_access_token()))
            .build()?;

        self.execute(http_request).await
    }

    /// Make a POST request to {path} and deserialize the response body
    pub(crate) async fn post<Model, Payload>(
        &self,
        path: &str,
        data: Payload,
    ) -> Result<KiteApiResponse<Model>>
    where
        Model: DeserializeOwned,
        Payload: Serialize,
    {
        let http_request = self
            .http_client()
            .post(self.config.url(path))
            // .query(&self.config.query())
            // Fetch access token for protected endpoints, if available
            .headers(self.config.headers(self.get_access_token()))
            .json(&data)
            .build()?;

        self.execute(http_request).await
    }

    /// POST a form at {path} and deserialize the response body into the generic `Model` type
    pub(crate) async fn post_form<Model, F>(
        &self,
        path: &str,
        form: &F,
    ) -> Result<KiteApiResponse<Model>>
    where
        Model: DeserializeOwned,
        F: Serialize + ?Sized,
    {
        let http_request = self
            .http_client()
            .post(self.config.url(path))
            // .query(&self.config.query())
            // Fetch access token for protected endpoints, if available
            .headers(self.config.headers(self.get_access_token()))
            .form(form)
            .build()?;

        self.execute(http_request).await
    }

    /// Make a DELETE request to {path} and deserialize the response body
    pub(crate) async fn delete<Model>(
        &self,
        path: &str,
        with_auth: bool,
    ) -> Result<KiteApiResponse<Model>>
    where
        Model: DeserializeOwned,
    {
        let mut http_request_builder = self
            .http_client()
            .delete(self.config.url(path))
            // Fetch access token for protected endpoints, if available
            .headers(self.config.headers(self.get_access_token()));

        if with_auth {
            let api_key = self.http_config().credentials().api_key();
            // Construct Vec<&str, &str> for query construction
            let query_vec = vec![
                ("api_key", api_key.expose_secret().as_str()),
                (
                    "access_token",
                    self.user_session()
                        .and_then(|session| Some(session.access_token.expose_secret().as_str()))
                        .unwrap_or_else(|| &"(ﾉﾟ0ﾟ)ﾉ~"),
                ),
            ];
            http_request_builder = http_request_builder.query(&query_vec);
        }
        let http_request = http_request_builder.build()?;

        self.execute(http_request).await
    }

    // Execute an HTTP request asynchronously
    async fn execute<Model>(&self, request: Request) -> Result<KiteApiResponse<Model>>
    where
        Model: DeserializeOwned,
    {
        match self.http_client().execute(request).await {
            Ok(response) => {
                match response.text().await {
                    Ok(json_response) => {
                        // Attempt to parse into a Kite API response on a generic type (Model)
                        Ok(serde_json::from_str::<KiteApiResponse<Model>>(
                            &json_response,
                        )?)
                    }
                    Err(err) => Err(err.into()),
                }
            }
            Err(err) => Err(err.into()),
        }
    }
}
