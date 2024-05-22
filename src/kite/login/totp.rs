use crate::kite::error::{ManjaError, Result};
use std::time::{SystemTime, UNIX_EPOCH};

/// Get Unix epoch time (in secs).
///
fn epoch_time() -> u64 {
    // Get the current system time
    let now = SystemTime::now();
    // Convert to seconds since the Unix epoch.
    now.duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

/// Decode a Base32 encoded string into bytes.
///
fn decode_base32(secret: &str) -> Result<Vec<u8>> {
    // Convert the input string to uppercase to handle case insensitivity
    let upper_secret = secret.to_uppercase();
    // Decode the base32 string
    base32::decode(base32::Alphabet::Rfc4648 { padding: false }, &upper_secret)
        .ok_or_else(|| ManjaError::TotpError(String::from("Failed to decode base32")))
}

/// Generate an RFC4648 compliant TOTP token using a private key.
///
/// The private key is used with HMAC-SHA1 to encode the number of seconds
/// since Jan 01, 1970 (epoch time counter). A token is then extracted from
/// this generated 160-bit HMAC.
pub fn generate_totp(totp_key: &str) -> String {
    let secret = decode_base32(totp_key).expect("Error while decoding base32 `totp_key`.");
    // Create a TOTP instance
    let totp_miner = totp_rs::TOTP::new(
        // Default algorithm
        totp_rs::Algorithm::SHA1,
        // Default digit length
        6,
        // Default step size
        1,
        // Default time step (interval)
        30,
        // Base32 decoded bytes
        secret,
    )
    .expect("Error while creating `totp_miner`.");
    // Generate the TOTP code for the current time
    totp_miner.generate(epoch_time())
}
