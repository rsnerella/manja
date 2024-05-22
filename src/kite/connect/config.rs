use url::Url;

#[derive(Clone)]
pub struct KiteConfig {
    pub base_url_api: Url,
    pub base_url_login: Url,
    pub redirect_url: Url,
}

impl Default for KiteConfig {
    fn default() -> Self {
        let base_url_api = match Url::parse("https://api.kite.trade") {
            Ok(url) => url,
            Err(_) => {
                // This should ideally never happen.
                panic!("Error parsing `base_url_api` in `KiteConnectClientBuilder::default()`")
            }
        };
        let base_url_login = match Url::parse("https://kite.trade/connect/login") {
            Ok(url) => url,
            Err(_) => {
                // This should ideally never happen.
                panic!("Error parsing `base_url_login` in `KiteConnectClientBuilder::default()`")
            }
        };
        let redirect_url = match Url::parse("https://127.0.0.1/kite-redirect?") {
            Ok(url) => url,
            Err(_) => {
                // This should ideally never happen.
                panic!("Error parsing `redirect_url` in `KiteConnectClientBuilder::default()`")
            }
        };
        Self {
            base_url_api,
            base_url_login,
            redirect_url,
        }
    }
}

impl KiteConfig {
    /// Constructor for `KiteConfig`.
    ///
    pub fn from_parts(base_url_api: Url, base_url_login: Url, redirect_url: Url) -> Self {
        Self {
            base_url_api,
            base_url_login,
            redirect_url,
        }
    }
}
