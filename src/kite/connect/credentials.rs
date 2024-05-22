//! User credentials for accessing KiteConnect trading APIs

use std::error::Error;
use zeroize::Zeroizing;

/// KiteConnect Credentials
///
/// When `KiteCredentials` is dropped, its contents are zeroed in memory.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct KiteCredentials {
    api_key: Zeroizing<String>,
    api_secret: Zeroizing<String>,
    user_id: Zeroizing<String>,
    user_pwd: Zeroizing<String>,
    totp_key: Zeroizing<String>,
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
            api_key: Zeroizing::new(api_key.into()),
            api_secret: Zeroizing::new(api_secret.into()),
            user_id: Zeroizing::new(user_id.into()),
            user_pwd: Zeroizing::new(user_pwd.into()),
            totp_key: Zeroizing::new(totp_key.into()),
        }
    }

    /// Load credentials from environment
    pub fn load_from_env() -> Result<KiteCredentials, Box<dyn Error>> {
        Ok(KiteCredentials::new(
            std::env::var("KITE_API_KEY")?.as_str(),
            std::env::var("KITE_API_SECRET")?.as_str(),
            std::env::var("KITE_USER_ID")?.as_str(),
            std::env::var("KITE_USER_PWD")?.as_str(),
            std::env::var("KITE_TOTP_KEY")?.as_str(),
        ))
    }

    /// Returns the API key
    pub fn api_key(&self) -> Zeroizing<String> {
        self.api_key.clone()
    }

    /// Returns the API secret
    pub fn api_secret(&self) -> Zeroizing<String> {
        self.api_secret.clone()
    }

    /// Returns the user ID
    pub fn user_id(&self) -> Zeroizing<String> {
        self.user_id.clone()
    }

    /// Returns the user password
    pub fn user_pwd(&self) -> Zeroizing<String> {
        self.user_pwd.clone()
    }

    /// Returns the TOTP key
    pub fn totp_key(&self) -> Zeroizing<String> {
        self.totp_key.clone()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct KiteTickerCredentials {
    request_token: Zeroizing<String>,
    access_token: Zeroizing<String>,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::env;

    #[test]
    fn test_kite_credentials_from_env() {
        // Setup env vars
        env::set_var("KITE_API_KEY", "notanapikey42");
        env::set_var("KITE_API_SECRET", "thatreallylongsupersecret42");
        env::set_var("KITE_USER_ID", "XY12345");
        env::set_var("KITE_USER_PWD", "ohsosecret");
        env::set_var("KITE_TOTP_KEY", "JBSWY3DPEHPK3PXPZVZSWIDGNJQXGZLE");

        match KiteCredentials::load_from_env() {
            Ok(kc) => {
                assert_eq!(kc.api_key(), Zeroizing::new(String::from("notanapikey42")));
                assert_eq!(
                    kc.api_secret(),
                    Zeroizing::new(String::from("thatreallylongsupersecret42"))
                );
                assert_eq!(kc.user_id(), Zeroizing::new(String::from("XY12345")));
                assert_eq!(kc.user_pwd(), Zeroizing::new(String::from("ohsosecret")));
                assert_eq!(
                    kc.totp_key(),
                    Zeroizing::new(String::from("JBSWY3DPEHPK3PXPZVZSWIDGNJQXGZLE"))
                );
            }
            Err(e) => assert_eq!(String::new(), format!("Error: {}", e)),
        }
    }
}
