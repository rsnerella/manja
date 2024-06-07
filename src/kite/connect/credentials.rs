//! User credentials for accessing KiteConnect trading APIs
//!
//! The following environment variables are required to be set to use the
//! KiteConnect trading APIs.
//!
//! - `KITECONNECT_API_KEY`: API key available from the KiteConnect dev portal
//! - `KITECONNECT_API_SECRET`: API secret available from the KiteConnect dev portal
//! - `KITECONNECT_USER_ID`: User login ID
//! - `KITECONNECT_PASSWORD`: User password
//! - `KITECONNECT_TOTP_KEY`: Time-based One-Time Password for 2FA, available
//! from the Kite web portal

use secrecy::Secret;

/// KiteConnect Credentials
///
/// When `KiteCredentials` is dropped, its contents are zeroed in memory.
#[derive(Clone, Debug)]
pub struct KiteCredentials {
    api_key: Secret<String>,
    api_secret: Secret<String>,
    user_id: Secret<String>,
    user_pwd: Secret<String>,
    totp_key: Secret<String>,
}

impl Default for KiteCredentials {
    fn default() -> Self {
        // Default to loading credentials from environment variables
        Self::load_from_env()
    }
}

impl KiteCredentials {
    /// Creates `KiteCredentials`.
    ///
    /// Intended to be used from a customs credentials provider implementation.
    /// It is __NOT__ safe to hardcode credentials in your application.
    pub fn new<InS>(
        api_key: InS,
        api_secret: InS,
        user_id: InS,
        user_pwd: InS,
        totp_key: InS,
    ) -> Self
    where
        InS: Into<String>,
    {
        KiteCredentials {
            api_key: Secret::new(api_key.into()),
            api_secret: Secret::new(api_secret.into()),
            user_id: Secret::new(user_id.into()),
            user_pwd: Secret::new(user_pwd.into()),
            totp_key: Secret::new(totp_key.into()),
        }
    }

    /// Load credentials from environment variables
    pub fn load_from_env() -> Self {
        Self {
            api_key: std::env::var("KITECONNECT_API_KEY")
                .unwrap_or_else(|_| "".to_string())
                .into(),
            api_secret: std::env::var("KITECONNECT_API_SECRET")
                .unwrap_or_else(|_| "".to_string())
                .into(),
            user_id: std::env::var("KITECONNECT_USER_ID")
                .unwrap_or_else(|_| "".to_string())
                .into(),
            user_pwd: std::env::var("KITECONNECT_PASSWORD")
                .unwrap_or_else(|_| "".to_string())
                .into(),
            totp_key: std::env::var("KITECONNECT_TOTP_KEY")
                .unwrap_or_else(|_| "".to_string())
                .into(),
        }
    }

    /// Returns the API key
    pub fn api_key(&self) -> Secret<String> {
        self.api_key.clone()
    }

    /// Returns the API secret
    pub fn api_secret(&self) -> Secret<String> {
        self.api_secret.clone()
    }

    /// Returns the user ID
    pub fn user_id(&self) -> Secret<String> {
        self.user_id.clone()
    }

    /// Returns the user password
    pub fn user_pwd(&self) -> Secret<String> {
        self.user_pwd.clone()
    }

    /// Returns the TOTP key
    pub fn totp_key(&self) -> Secret<String> {
        self.totp_key.clone()
    }
}

// TODO: Move to `kite::ticker`
#[derive(Clone)]
pub struct KiteTickerCredentials {
    request_token: Secret<String>,
    access_token: Secret<String>,
}

#[cfg(test)]
mod test {
    use secrecy::ExposeSecret;

    use super::*;
    use std::env;

    #[test]
    fn test_kite_credentials_default() {
        // Setup env vars
        env::set_var("KITECONNECT_API_KEY", "notanapikey42");
        env::set_var("KITECONNECT_API_SECRET", "thatreallylongsupersecret42");
        env::set_var("KITECONNECT_USER_ID", "XY12345");
        env::set_var("KITECONNECT_PASSWORD", "ohsosecret");
        env::set_var("KITECONNECT_TOTP_KEY", "JBSWY3DPEHPK3PXPZVZSWIDGNJQXGZLE");

        let kc = KiteCredentials::default();

        assert_eq!(kc.api_key().expose_secret(), &String::from("notanapikey42"));
        assert_eq!(
            kc.api_secret().expose_secret(),
            &String::from("thatreallylongsupersecret42")
        );
        assert_eq!(kc.user_id().expose_secret(), &String::from("XY12345"));
        assert_eq!(kc.user_pwd().expose_secret(), &String::from("ohsosecret"));
        assert_eq!(
            kc.totp_key().expose_secret(),
            &String::from("JBSWY3DPEHPK3PXPZVZSWIDGNJQXGZLE")
        );
    }
}
