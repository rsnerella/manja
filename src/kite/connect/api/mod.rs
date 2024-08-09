//! API endpoint definitions and functions for interacting with Kite Connect API.
//!
//! This module organizes the various API groups for Kite Connect API. It includes
//! submodules for managing sessions, user data, orders, portfolio, market data,
//! and margins. Each submodule corresponds to a specific set of endpoints in
//! Kite Connect API, making it easier to interact with different aspects of the
//! trading platform.
//!
//! # Submodules
//!
//! - `session`: Handles the `/session/` API group, including authentication and
//!     session management.
//! - `user`: Manages the `/user/` API group, providing access to user-specific
//!     data and settings.
//! - `orders`: Handles the `/orders/` API group, facilitating order placement,
//!     modification, and status checks.
//! - `portfolio`: Manages the `/portfolio/` API group, including holdings and positions.
//! - `market`: Handles the `/instruments/` and `/quote/` API group, providing
//!     market data and instrument information.
//! - `margins`: Manages the `/margins/` and `/charges/` API group, dealing with
//!     margin requirements and charges.
//!
use backoff::{ExponentialBackoff, ExponentialBackoffBuilder};
use std::time::Duration;

// Manages the `/session/` API group, including authentication and session management.
mod session;
pub use session::Session;

// Manages the `/user/` API group, providing access to user-specific data
// and settings.
mod user;
pub use user::User;

// Manages the `/orders/` API group, facilitating order placement, modification,
// and status checks.
mod orders;
pub use orders::Orders;

// Manages the `/portfolio/` API group, including holdings and positions.
//
mod portfolio;
pub use portfolio::Portfolio;

// Manages the `/instruments/` and `/quote/` API group, providing market data
// and instrument information.
mod market;
pub use market::Market;

// Manages the `/margins/` and `/charges/` API group, dealing with margin
// requirements and charges.
mod margins;
pub use margins::{Charges, Margins};

/// Creates an ExponentialBackoff policy with a specified rate limit.
///
/// This function sets up an exponential backoff policy to control the rate of
/// API requests, ensuring compliance with rate limits by introducing a minimum
/// interval between requests.
///
/// # Arguments
///
/// * `rate_limit_per_second` - The number of allowed API requests per second.
///
/// # Returns
///
/// An `ExponentialBackoff` instance configured with the specified rate limit.
///
/// # Example
///
/// ```ignore
/// let backoff_policy = create_backoff_policy(10); // 10 requests per second
/// ```
fn create_backoff_policy(rate_limit_per_second: u64) -> ExponentialBackoff {
    // Calculate the minimum duration between requests
    let min_interval = Duration::from_secs_f64(1.0 / rate_limit_per_second as f64);

    ExponentialBackoffBuilder::new()
        .with_initial_interval(min_interval)
        .with_multiplier(1.0) // No exponential increase in delay
        .with_max_interval(min_interval) // Ensure max interval does not exceed rate limit
        .with_max_elapsed_time(None) // No maximum elapsed time for retries
        .build()
}
