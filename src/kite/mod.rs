//! Kite module for interacting with Kite Connect API.
//!
//! This module provides various submodules and traits for connecting to,
//! authenticating, and interacting with Kite Connect API, including handling
//! WebSocket streams for live market data and managing user sessions.
//!
pub mod connect;
pub mod error;
pub mod login;
pub mod ticker;
pub mod traits;
