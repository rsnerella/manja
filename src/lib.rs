//! An asynchronous client library for [Zerodha](https://zerodha.com/)'s [Kite Connect](https://kite.trade/)
//! trading APIs (a set of REST-like HTTP APIs).
//!
//! This crate uses the [tokio](https://tokio.rs/#tk-lib-runtime) asynchronous runtime.
//!
//! # Example:
//! ```ignore
//! use std::error::Error;
//!
//! mod kite;
//! use kite::connect::client::HTTPClient;
//!
//! use kite::login::flow::browser_login_flow;
//! use kite::ticker::{client::WebSocketClient, models::Mode};
//! use kite::ticker::{KiteStreamCredentials, StreamState};
//! use kite::traits::KiteLoginFlow;
//!
//! use futures_util::StreamExt;
//!
//! use tokio;
//! use tracing::{error, info};
//! use tungstenite::client::IntoClientRequest;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     // Setup tracing
//!     tracing_subscriber::fmt()
//!         .with_max_level(tracing::Level::INFO)
//!         .init();
//!
//!     // Load env vars
//!     dotenv::dotenv().ok();
//!
//!     // Create a default HTTPClient
//!     let mut manja_client = HTTPClient::default();
//!
//!     let session = manja_client.session();
//!
//!     // Login flow I: request token
//!     let request_token = session.gen_request_token(browser_login_flow).await?;
//!
//!     // Login flow II: user session
//!     let kite_session = manja_client
//!         .session()
//!         .generate_session(&request_token)
//!         .await?;
//!
//!     let stream_creds = KiteStreamCredentials::from(kite_session.data.unwrap());
//!     let stream_state = StreamState::from_credentials(stream_creds)
//!         // INFY
//!         .subscribe_token(Mode::Full, 408065)
//!         // TATAMOTORS
//!         .subscribe_token(Mode::Full, 884737);
//!     
//!
//!     info!(
//!         "StreamState = {:?}",
//!         stream_state.clone().into_client_request()
//!     );
//!
//!     if let Ok(mut ticker) = WebSocketClient::connect(stream_state).await {
//!         for _ in 0..120 {
//!             tokio::time::sleep(std::time::Duration::from_millis(500)).await;
//!             if let Some(maybe_msg) = ticker.next().await {
//!                 match maybe_msg {
//!                     Ok(msg) => info!("Message: {}", msg),
//!                     Err(e) => error!("Error: {}", e),
//!                 }
//!             }
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
#![allow(private_interfaces, unused)]
pub mod kite;
