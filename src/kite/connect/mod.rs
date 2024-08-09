//! HTTP Client and additional functionality.
//!
//! This module serves as the main entry point for the `manja` crate. It includes
//! submodules that provide functionality for interacting with Kite Connect trading
//! APIs, managing configurations, handling user credentials, and working with
//! various models and utilities.
//!

/// Provides API endpoint definitions and functions for interacting with Kite
/// Connect API.
///
pub mod api;

/// Defines the HTTP client for making requests to Kite Connect API.
///
pub mod client;

/// Contains configuration settings for the HTTP client, including API URLs and
/// credentials management.
///
pub mod config;

/// Manages user credentials required for accessing the KiteConnect APIs.
///
pub mod credentials;

/// Defines various models used in Kite Connect API responses and requests.
///
pub mod models;

/// Contains utility functions and helpers used across the `manja` crate.
///
mod utils;
