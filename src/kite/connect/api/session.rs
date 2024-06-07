//! Session API group: `/session/`
//!
//! The [Session] API group provides endpoints for managing user sessions,
//! including the two-step login flow required to authenticate with the
//! KiteConnect API.
//!
//! This module facilitates the following:
//!
//! 1. **Generating a request token**: Initiate the login flow by navigating to
//!    the Kite Connect login page with the `api_key`.
//! 2. **Exchanging the request token for an access token**: Use the `request_token`
//!    and a checksum to obtain an `access_token` for authenticated API requests.
//!
//! ![Login Flow Diagram](https://kite.trade/docs/connect/v3/images/kite-connect-flow.png)
//!
//! For detailed information, refer to the official KiteConnect API
//! [documentation](https://kite.trade/docs/connect/v3/user/#login-flow).
use std::collections::HashMap;

use secrecy::ExposeSecret;

// use crate::kite::connect::config::Config;
use crate::kite::connect::models::UserSession;
use crate::kite::connect::utils::create_checksum;
use crate::kite::connect::{client::HTTPClient, models::KiteApiResponse};
use crate::kite::error::Result;
use crate::kite::login::flow::login_flow;
use crate::kite::traits::{KiteConfig, KiteLoginFlow};

/// Represents the user session related API endpoints.
///
/// This struct handles operations related to user sessions, including login
/// and session management, by interfacing with the HTTP client.
///
/// For more details, refer to the official
/// [API documentation](https://kite.trade/docs/connect/v3/user/#login-flow).
pub struct Session<'c> {
    /// A mutable reference to the HTTP client used for making API requests
    /// and storing a `UserSession` object after a successful login flow.
    pub client: &'c mut HTTPClient,
}

impl<'c> KiteLoginFlow for Session<'c> {
    /// Generates a request token using the provided login function.
    ///
    /// This method uses the provided closure to execute the login process and
    /// obtain a request token.
    ///
    /// # Arguments
    ///
    /// * `login` - A closure that executes the login process and returns a `Result` containing
    ///   the request token as a `String`.
    ///
    /// # Returns
    ///
    /// A `Result` containing the request token as a `String` if successful, or an error otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// let mut client = HTTPClient::new();
    /// let session = Session::new(&mut client);
    /// let token_result = session.generate_request_token(|config| Ok("request_token".to_string()));
    /// assert!(token_result.is_ok());
    /// ```
    fn generate_request_token<F>(&self, login: F) -> Result<String>
    where
        F: Fn(&dyn KiteConfig) -> Result<String>,
    {
        login(self.client.http_config())
    }
}

impl<'c> Session<'c> {
    /// Creates a new `Session` instance.
    ///
    /// # Arguments
    ///
    /// * `client` - A mutable reference to an `HTTPClient` instance.
    ///
    /// # Returns
    ///
    /// Returns a new `Session` instance containing the provided HTTP client.
    ///
    /// # Example
    ///
    /// ```
    /// let mut client = HTTPClient::new();
    /// let session = Session::new(&mut client);
    /// ```
    pub fn new(client: &'c mut HTTPClient) -> Self {
        Self { client }
    }

    /// Generates a `request_token` which can be used to obtain an `access_token`.
    ///
    /// This method initiates the first step of the [login flow](https://kite.trade/docs/connect/v3/user/#login-flow).
    ///
    /// The login flow involves the following steps:
    ///
    /// 1. Navigate to the Kite Connect login page with the `api_key`.
    /// 2. A successful login returns a `request_token` as a URL query parameter to the registered redirect URL.
    /// 3. This `request_token`, along with a checksum (SHA-256 of `api_key` + `request_token` + `api_secret`), is POSTed to the token API to obtain an `access_token`.
    /// 4. The `access_token` is then used for signing all subsequent requests.
    ///
    /// # Arguments
    ///
    /// This method does not require any arguments as it uses the configuration
    /// from the client instance.
    ///
    /// # Returns
    ///
    /// * A `Result` containing the `request_token` as a `String` if successful,
    /// or an error if the request token generation fails.
    ///
    /// # Example
    ///
    /// ```
    /// let request_token = client.generate_request_token().await;
    /// match request_token {
    ///     Ok(token) => println!("Request token generated successfully: {}", token),
    ///     Err(e) => println!("Error generating request token: {}", e),
    /// }
    /// ```
    ///
    /// Note: Ensure that the required configuration parameters such as `api_login`,
    /// `api_redirect`, and `credentials` are correctly set in the client configuration.
    pub async fn generate_request_token(&self) -> Result<String> {
        // TODO: Fix the arguments for `login_flow` and put it behind a feature gate
        let config = self.client.http_config();
        login_flow(
            &url::Url::parse(config.api_login())?,
            &url::Url::parse(config.api_redirect())?,
            config.credentials(),
        )
        .await
    }

    /// Generates a session using a `request token`.
    ///
    /// This method completes the second step of the
    /// [login flow](https://kite.trade/docs/connect/v3/user/#login-flow),
    /// where a `request token` obtained from the initial login step is used
    /// to generate a valid session and obtain an `access_token`.
    ///
    /// # Arguments
    ///
    /// * `request_token` - The token received after the initial login step,
    /// which is required to generate the session.
    ///
    /// # Returns
    ///
    /// * A result containing the session details if successful, or an error
    /// if the session generation fails.
    ///
    /// # Example
    ///
    /// ```
    /// let session = kite_connect.generate_session(request_token);
    /// match session {
    ///     Ok(session) => println!("Session generated successfully!"),
    ///     Err(e) => println!("Error generating session: {}", e),
    /// }
    /// ```
    pub async fn generate_session(
        &mut self,
        request_token: &str,
    ) -> Result<KiteApiResponse<UserSession>> {
        // Yeah, this is quite a fetch.
        let api_key = self.client.http_config().credentials().api_key();
        let api_secret = self.client.http_config().credentials().api_secret();
        // Compute checksum needed for the API call
        let checksum = create_checksum(
            api_key.expose_secret().as_str(),
            request_token,
            api_secret.expose_secret().as_str(),
        );
        // Construct form parameters as per KiteConnect documentation
        //  ref: https://kite.trade/docs/connect/v3/user/#authentication-and-token-exchange
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("api_key", api_key.expose_secret().as_str());
        params.insert("request_token", request_token);
        params.insert("checksum", checksum.as_str());
        // info!("Params: {:?}", params);
        let kite_response: Result<KiteApiResponse<UserSession>> =
            self.client.post_form("/session/token", &params).await;
        match kite_response {
            Ok(kite_response) => {
                // Set the UserSession object on HTTPClient
                self.client.set_user_session(kite_response.data.clone());
                Ok(kite_response)
            }
            Err(err) => Err(err),
        }
    }

    /// Invalidates the `access_token` and destroys the current API session.
    ///
    /// After calling this method, the user will need to go through a new login
    /// flow to obtain a fresh `access_token` before any further interactions
    /// with the KiteConnect API can be made.
    ///
    /// This is useful for logging out a user or resetting their session for security reasons.
    pub async fn delete_session(&mut self) -> Result<KiteApiResponse<bool>> {
        match self.client.delete("/session/token", true).await {
            Ok(kite_response) => {
                // Remove the UserSession object from the HTTPClient
                self.client.set_user_session(None);
                Ok(kite_response)
            }
            Err(err) => Err(err),
        }
    }
}
