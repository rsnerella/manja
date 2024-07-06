//! Asynchronous WebSocket-based client for streaming data
//!
//! # Sample use
//! ```ignore
//! use kite::ticker::KiteTickerClient;
//! ```
pub mod client;
pub use client::{TickerStream, WebSocketClient};

mod stream;
pub use stream::{KiteStreamCredentials, StreamState};

pub mod models;
pub use models::{Mode, TickerRequest};
