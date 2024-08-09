//! Automated authentication flow using user credentials.
//!
//! This module serves as the main entry point for `manja` crate's automated
//! login support. It includes submodules that provide functionality for interacting
//! with a Chrome browser for authentication, managing the authentication flow,
//! and generating Time-based One-Time Passwords (TOTP).
//!
//! # Submodules
//!
//! - `chrome`: Contains functions and utilities for launching and interacting
//!     with a Chrome browser instance using WebDriver.
//! - `flow`: Manages the overall Kite Connect authentication flow, including
//!     login and token retrieval.
//! - `totp`: Provides functions for generating Time-based One-Time Passwords
//!     (TOTP) compliant with RFC4648.
//!
//! # Re-exports
//!
//! This module also re-exports commonly used types from the `fantoccini` and
//! `tokio` crates for convenience.
//!
pub mod chrome;
pub mod flow;
pub mod totp;

use fantoccini::client::Client as BrowserClient;
use tokio::time::sleep as tokio_sleep;
use tokio::time::Duration as TokioDuration;
