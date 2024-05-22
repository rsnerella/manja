use sha2::{Digest, Sha256};

/// Checksum needed for retreiving user access token from KiteConnect API.
///
pub fn create_checksum(api_key: &str, request_token: &str, api_secret: &str) -> String {
    // SHA256 hasher instance
    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}{}", api_key, request_token, api_secret));
    // Encode to hexadecimal string
    hex::encode(hasher.finalize())
}
