//! KiteConnect API groups

use backoff::{ExponentialBackoff, ExponentialBackoffBuilder};
use std::time::Duration;

// `/session/` API group
mod session;
pub use session::Session;

// `/user/` API group
mod user;
pub use user::User;

// `/orders/` API group
mod orders;
pub use orders::Orders;

/// Creates an ExponentialBackoff policy with a specified rate limit.
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
