//! An asynchronous client library for [Zerodha](https://zerodha.com/)'s [Kite Connect](https://kite.trade/)
//! trading APIs (a set of REST-like HTTP APIs).
//!
//! This crate uses the [tokio](https://tokio.rs/#tk-lib-runtime) asynchronous runtime.
//!
//! # Example:
//! ```no_run
//! mod kite;
//! use kite::connect::client_builder::KiteConnectClientBuilder;
//! use kite::connect::credentials::KiteCredentials;
//! use std::error::Error;
//!
//! use tokio;
//! use tracing::info;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     // Load environment variables from a `.env` file
//!     dotenv::dotenv().ok();
//!
//!     // Load Kite user credentials from the environment
//!     let kite_creds = KiteCredentials::load_from_env()?;
//!     info!("{:?}", kite_creds);
//!     
//!     // Construct a `KiteConnect` client from a builder struct
//!     let manja_client = KiteConnectClientBuilder::default()
//!         .with_credentials(kite_creds)
//!         .build()?;
//!
//!     // Generate a `request_token` as part of the login flow
//!     let request_token = manja_client.generate_request_token().await?;
//!     // Use the `request_token` to generate a user session (having an `access_token`)
//!     let user_session = manja_client.generate_session(&request_token).await?;
//!
//!     println!("{:?}", user_session);
//!
//!     Ok(())
//! }
//! ```
pub mod kite;
