//! > **Manja** (IPA: /maːŋdʒʱaː/) n.: A type of abrasive string utilized primarily for flying fighter kites, especially prevalent in South Asian countries. It is crafted by coating cotton string with powdered glass or a similar abrasive substance.
//!
//! An asynchronous client library for [Zerodha](https://zerodha.com/)'s [Kite Connect](https://kite.trade/)
//! trading APIs (a set of REST-like HTTP APIs).
//!
//! # `manja` Features
//!
//! - **Type safe**
//!    - *Compile-time Type Checking*: type safety ensures that errors related to type mismatches are caught during compilation rather than at runtime.
//!    - *Consistent Data Models*: `manja` uses strongly typed data models that match Kite Connect API's expected inputs and outputs.
//!    - *Enhanced Security*: by ensuring that only valid data types are sent to and received from the API, the risk of data-related vulnerabilities is reduced.
//!    - *Automatic Serialization/Deserialization*: `manja` handles the serialization (converting data structures to JSON) and deserialization (converting JSON responses back to data structures) automatically and correctly. This ensures that the data sent to and received from Kite Connect API adheres to the expected types.
//!    
//! - **Asynchronous**: built on the performant `tokio` async-runtime, `manja` delivers unmatched performance, ensuring your applications run faster and more efficiently than ever before.
//!    - *Resource Efficiency*: maximize the use of your system's resources. `manja`'s asynchronous nature allows for optimal resource management, reducing overhead and improving overall performance.
//!    - *Concurrent Task Handling*: manage multiple tasks simultaneously without sacrificing performance or reliability.
//!    - *Improved latency*: experience reduced latency and faster response times, ensuring your applications are always responsive.
//!
//! - **Distributed Logging**: stay ahead of issues with real-time distributed logging using the `tracing` crate.
//!    - *Streamline Development*: facilitate smoother development cycles with better debugging and faster issue resolution.
//!    - *Reduce Downtime*: with real-time insights and quick access to logs, identify and resolve issues faster, minimizing downtime.
//!    - *Enhance User Experience*: quickly address errors and performance bottlenecks to provide a better experience for your users.
//!
//! - **WebSocket** support for streaming binary market data.
//!    - *Auto-reconnect Mechanism*: `manja` provides a reliable and stateful async WebSocket client with a configurable exponential backoff retry mechanism.
//!
//! - **WebDriver** integration for retrieving `request token` from the redirect URL after successfully authenticating with the Kite platform.
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
//!
//! # Disclaimer
//!
//! **Important Notice**:
//!
//! * The `manja` crate is currently in development and should be considered unstable. The API is subject to change without notice, and breaking changes are likely to occur.
//!
//! * The software is provided "as-is" without any warranties, express or implied. The author and contributors of this SDK do not take responsibility for any financial losses, damages, or other issues that may arise from the use of this project.
#![warn(rust_2018_idioms)]
#![allow(private_interfaces, unused)]

pub mod kite;
