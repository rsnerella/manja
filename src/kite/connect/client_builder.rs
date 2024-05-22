use std::time::Duration;
use url::Url;

use crate::kite::connect::client::KiteConnectClient;
use crate::kite::connect::config::KiteConfig;
use crate::kite::connect::credentials::KiteCredentials;
use crate::kite::error::{ManjaError, Result};

/// Builder struct for `KiteConnectClient`.
///
pub struct KiteConnectClientBuilder {
    client_builder: Option<reqwest::ClientBuilder>,
    kite_credentials: Option<KiteCredentials>,
    kite_config: Option<KiteConfig>,
}

impl Default for KiteConnectClientBuilder {
    fn default() -> Self {
        Self {
            // Default timeout for I/O operations: 10 seconds
            client_builder: Some(reqwest::ClientBuilder::new().timeout(Duration::from_secs(10))),
            kite_credentials: None,
            // Default configurations
            kite_config: Some(KiteConfig::default()),
        }
    }
}

impl KiteConnectClientBuilder {
    /// Add `KiteCredentials` to the client builder
    ///
    pub fn with_credentials(mut self, credentials: KiteCredentials) -> Self {
        self.kite_credentials = Some(credentials);
        self
    }

    /// A successful login comes back with a `request_token` as a URL query
    /// parameter to the redirect URL registered on the developer console
    /// for that `api_key`.
    ///
    /// See [login flow](https://kite.trade/docs/connect/v3/user/#login-flow).
    pub fn with_redirect_url(mut self, redirect_url: Url) -> Self {
        match self.kite_config {
            Some(ref mut config) => {
                // Side-effect.
                config.redirect_url = redirect_url;
                ()
            }
            None => {
                // Use a default config and update `redirect_url`.
                let mut config = KiteConfig::default();
                config.redirect_url = redirect_url;
                self.kite_config = Some(config);
                ()
            }
        };
        self
    }

    /// Try constructing a `KiteConnectClient`.
    ///
    pub fn build(&mut self) -> Result<KiteConnectClient> {
        let client = self
            .client_builder
            .take()
            .ok_or_else(|| ManjaError::from("`reqwest::ClientBuilder` not set."))?
            .build()?;
        let kite_credentials = self
            .kite_credentials
            .take()
            .ok_or_else(|| ManjaError::from("`KiteCredential` not set."))?;
        let kite_config = self
            .kite_config
            .take()
            .ok_or_else(|| ManjaError::from("`KiteConfig` not set."))?;
        Ok(KiteConnectClient::from_parts(
            client,
            kite_credentials,
            kite_config,
        ))
    }
}
