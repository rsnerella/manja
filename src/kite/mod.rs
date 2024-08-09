//! Kite module for interacting with Kite Connect API.
//!
//! This module provides various submodules and traits for connecting to,
//! authenticating, and interacting with Kite Connect API, including handling
//! WebSocket streams for live market data and managing user sessions.
//!
//! # Submodules
//!
//! - `connect`: Contains structures and functions for interacting with Kite Connect
//!     trading APIs, managing configurations and handling user credentials.
//! - `error`: Defines custom error types and results used throughout Kite Connect API interactions.
//! - `login`: Provides automated user login support via Chrome and WebDriver.
//! - `ticker`: Provides functionality for handling WebSocket streams for live market data.
//! - `traits`: Defines traits used for configuring and interacting with Kite Connect API
//!
pub mod connect;
pub mod error;
pub mod login;
pub mod ticker;
pub mod traits;
