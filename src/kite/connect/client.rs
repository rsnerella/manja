//! Asynchronous KiteConnect client
//!
//! This module provides an asynchronous client for interacting with the KiteConnect API.
//! The `HTTPClient` struct encapsulates a `reqwest::Client` and includes methods for making
//! HTTP requests to various KiteConnect endpoints. The client supports retry mechanisms
//! with exponential backoff and handles user session management.
//!
//! # Features
//!
//! - **Session Management**: Manages user sessions, including storing and retrieving session tokens.
//! - **Request Handling**: Provides methods for making GET, POST, POST form, and DELETE requests.
//! - **Configurable**: Allows configuration via the `Config` struct, which can be loaded from
//!   environment variables or passed directly.
//!
//! # Examples
//!
//! ```rust
//! use crate::kite::connect::client::HTTPClient;
//! use crate::kite::connect::config::Config;
//!
//! // Create a new client with default settings
//! let client = HTTPClient::default();
//!
//! // Create a new client with a custom configuration
//! let config = Config::default();
//! let client = HTTPClient::with_config(config);
//! ```
//!
//! For more information on using the KiteConnect API, refer to the
//! [official documentation](https://kite.trade/docs/connect/v3/).
use core::future::Future;
use std::time::Duration;

use backoff::ExponentialBackoff;
// use reqwest::{Request, StatusCode};
use secrecy::{ExposeSecret, Secret};
use serde::{de::DeserializeOwned, Serialize};

use crate::kite::{
    connect::{
        api::{Market, Orders, Session, User},
        config::Config,
        models::{KiteApiResponse, UserSession},
    },
    error::{map_deserialization_error, KiteApiError, KiteApiException, ManjaError, Result},
    traits::KiteConfig,
};

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

    /// To call [Orders] related APIs using this client.
    pub fn orders(&mut self) -> Orders {
        Orders::new(self)
    }

    /// To call [Market] related APIs using this client.
    pub fn market(&mut self) -> Market {
        Market::new(self)
    }

    // --- [ HTTP verb functions ] ---

    /// Make a GET request to {path} and return the response body
    pub(crate) async fn get_raw(&self, path: &str, backoff: &ExponentialBackoff) -> Result<String> {
        let request_baker = || async {
            Ok(self
                .client
                .get(self.config.url(path))
                // Fetch access token for protected endpoints, if available
                .headers(self.config.headers(self.get_access_token()))
                .build()?)
        };

        self.execute_raw(backoff, request_baker).await
    }

    /// Make a GET request to {path} and deserialize the response body
    pub(crate) async fn get<Model>(
        &self,
        path: &str,
        backoff: &ExponentialBackoff,
    ) -> Result<KiteApiResponse<Model>>
    where
        Model: DeserializeOwned,
    {
        let request_baker = || async {
            Ok(self
                .client
                .get(self.config.url(path))
                // Fetch access token for protected endpoints, if available
                .headers(self.config.headers(self.get_access_token()))
                .build()?)
        };

        self.execute(backoff, request_baker).await
    }

    /// Make a GET request to {path} with given Query and deserialize the response body
    pub(crate) async fn get_with_query<Q, Model>(
        &self,
        path: &str,
        query: &Q,
        backoff: &ExponentialBackoff,
    ) -> Result<KiteApiResponse<Model>>
    where
        Q: Serialize + ?Sized,
        Model: DeserializeOwned,
    {
        let request_baker = || async {
            Ok(self
                .client
                .get(self.config.url(path))
                .query(query)
                // Fetch access token for protected endpoints, if available
                .headers(self.config.headers(self.get_access_token()))
                .build()?)
        };

        self.execute(backoff, request_baker).await
    }

    /// Make a POST request to {path} and deserialize the response body
    pub(crate) async fn post<Model, Payload>(
        &self,
        path: &str,
        data: Payload,
        backoff: &ExponentialBackoff,
    ) -> Result<KiteApiResponse<Model>>
    where
        Model: DeserializeOwned,
        Payload: Serialize,
    {
        let request_baker = || async {
            Ok(self
                .client
                .post(self.config.url(path))
                // Fetch access token for protected endpoints, if available
                .headers(self.config.headers(self.get_access_token()))
                .json(&data)
                .build()?)
        };

        self.execute(backoff, request_baker).await
    }

    /// POST a form at {path} and deserialize the response body into the generic `Model` type
    pub(crate) async fn post_form<Model, F>(
        &self,
        path: &str,
        form: &F,
        backoff: &ExponentialBackoff,
    ) -> Result<KiteApiResponse<Model>>
    where
        Model: DeserializeOwned,
        F: Serialize + ?Sized,
    {
        let request_baker = || async {
            Ok(self
                .client
                .post(self.config.url(path))
                // Fetch access token for protected endpoints, if available
                .headers(self.config.headers(self.get_access_token()))
                .form(form)
                .build()?)
        };

        self.execute(backoff, request_baker).await
    }

    /// Make a PUT request to {path} and deserialize the response body
    pub(crate) async fn put<Model, Payload>(
        &self,
        path: &str,
        data: Payload,
        backoff: &ExponentialBackoff,
    ) -> Result<KiteApiResponse<Model>>
    where
        Model: DeserializeOwned,
        Payload: Serialize,
    {
        let request_baker = || async {
            Ok(self
                .client
                .put(self.config.url(path))
                // Fetch access token for protected endpoints, if available
                .headers(self.config.headers(self.get_access_token()))
                .json(&data)
                .build()?)
        };

        self.execute(backoff, request_baker).await
    }

    /// Make a DELETE request to {path} and deserialize the response body
    pub(crate) async fn delete<Model>(
        &self,
        path: &str,
        with_auth: bool,
        backoff: &ExponentialBackoff,
    ) -> Result<KiteApiResponse<Model>>
    where
        Model: DeserializeOwned,
    {
        let request_baker = || async {
            let mut http_request_builder = self
                .client
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
            Ok(http_request_builder.build()?)
        };

        self.execute(backoff, request_baker).await
    }

    /// Execute a HTTP request asynchronously with backoff
    async fn execute<Model, RB, Fut>(
        &self,
        backoff: &ExponentialBackoff,
        request_baker: RB,
    ) -> Result<KiteApiResponse<Model>>
    where
        Model: DeserializeOwned,
        RB: Fn() -> Fut,
        Fut: Future<Output = Result<reqwest::Request>>,
    {
        let json_response = self.execute_raw::<RB, Fut>(backoff, request_baker).await?;

        let model: KiteApiResponse<Model> = serde_json::from_str(&json_response)
            .map_err(|e| map_deserialization_error(e, &json_response))?;

        Ok(model)
    }

    /// Execute a HTTP request asynchronously with backoff
    async fn execute_raw<RB, Fut>(
        &self,
        backoff: &ExponentialBackoff,
        request_baker: RB,
    ) -> Result<String>
    where
        RB: Fn() -> Fut,
        Fut: Future<Output = Result<reqwest::Request>>,
    {
        let client = self.http_client();
        // The magic sauce.
        backoff::future::retry(backoff.clone(), || async {
            // Bake a fresh request with rate limit
            let request = request_baker().await.map_err(backoff::Error::Permanent)?;
            let path = request.url().path().to_string();
            // Execute the HTTP request against some KiteConnect API endpoint
            let response = client
                .execute(request)
                .await
                .map_err(ManjaError::Reqwest)
                .map_err(backoff::Error::Permanent)?;
            let status = response.status();
            // Attempt to fetch the string (JSON) response
            let json_response = response
                .text()
                .await
                .map_err(ManjaError::Reqwest)
                .map_err(backoff::Error::Permanent)?;
            if !status.is_success() {
                // Attempt to JSON deserialize the KiteConnect API response
                let kite_response: KiteApiResponse<Option<String>> =
                    serde_json::from_str(&json_response)
                        .map_err(|e| map_deserialization_error(e, &json_response))
                        .map_err(backoff::Error::Permanent)?;
                let kite_error = KiteApiError {
                    endpoint: path.clone(),
                    status_code: status.as_u16(),
                    message: kite_response.message,
                    error_type: kite_response
                        .error_type
                        .and_then(|error_type| Some(KiteApiException::from(error_type.as_str())))
                        // This unwrap is safe since From<&str> is implemented for `KiteApiException`.
                        .unwrap(),
                };
                // Check if rate limit was exceeded on the endpoint
                if status.as_u16() == 429 {
                    tracing::warn!("Rate limited at endpoint: {}", path);
                    return Err(backoff::Error::transient(ManjaError::KiteApiError(
                        kite_error,
                    )));
                }
            }

            Ok(json_response)
        })
        .await
    }
}

#[cfg(test)]
pub mod test_utils {
    use std::collections::HashMap;

    use super::*;

    use mockito::{Mock, ServerGuard};

    pub fn read_to_object<M>(path: &str) -> Result<M>
    where
        M: DeserializeOwned,
    {
        let contents = std::fs::read_to_string(path).unwrap();
        let obj: KiteApiResponse<M> = serde_json::from_str(&contents)?;
        obj.data
            .ok_or(ManjaError::Internal(format!("obj not found")))
    }

    pub type HTTPMethod = &'static str;
    pub type APIEndpoint = &'static str;
    pub type TestResponse = &'static str;

    pub async fn add_mocks(
        mut server: ServerGuard,
        mock_map: HashMap<(HTTPMethod, APIEndpoint), TestResponse>,
    ) -> ServerGuard {
        let mut mocks = Vec::new();
        for ((method, api_endpoint), response_path) in mock_map {
            let response_json = std::fs::read_to_string(response_path).unwrap();
            let m = server
                .mock(method, api_endpoint)
                .with_status(200)
                .with_header("content-type", "application/json")
                .with_body(response_json)
                .create_async();
            mocks.push(m)
        }
        let _ms = futures::future::join_all(mocks).await;
        server
    }

    pub async fn get_manja_test_client() -> (ServerGuard, HTTPClient) {
        let server = mockito::Server::new_async().await;
        // Load env vars
        dotenv::dotenv().ok();
        // Patch the API base url on HTTPClient for testing
        std::env::set_var("KITECONNECT_API_BASE", &server.url());
        let session =
            read_to_object::<UserSession>("./kiteconnect-mocks/generate_session.json").unwrap();

        (server, HTTPClient::default().with_user_session(session))
    }
}
