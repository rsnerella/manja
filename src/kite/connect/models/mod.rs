//! Data types for interacting with Kite Connect (HTTP) API.
//!
//! This module defines the data models used in Kite Connect API. These models
//! represent the various structures used in API requests and responses, making
//! it easier to work with Kite Connect API in a type-safe manner.
//!
//! # Submodules and Types
//!
//! - `session`: Models for the `/session/` API group, including user session management.
//! - `user`: Models for the `/user/` API group, handling user-specific data and settings.
//! - `order`: Models for the `/orders/` API group, facilitating order placement,
//!     modification, and status checks.
//! - `order_enums`: Enumerations used in the `/orders/` API group.
//! - `portfolio`: Models for the `/portfolio/` API group, managing holdings and positions.
//! - `market`: Models for the `/instruments/` and `/quote/` API group, providing
//!     market data and instrument information.
//! - `margins`: Models for the `/margins/` and `/charges/` API group, dealing with
//!     margin requirements and charges.
//! - `exchange`: Enumerations for exchanges supported by Kite Connect API.
//!
use serde::{Deserialize, Serialize};

/// Represents the default response structure used by Kite Connect API.
///
/// The generic type `T` is typically a `HashMap` but can be any type that the
/// specific API response requires.
///
/// # Fields
///
/// - `status`: The status of the API response (e.g., "success" or "error").
/// - `data`: The actual data returned by the API, if any.
/// - `message`: An optional message providing additional information about the response.
/// - `error_type`: An optional error type string, present if the response indicates an error.
///
/// # Example
///
/// ```ignore
/// let response: KiteApiResponse<HashMap<String, String>> = KiteApiResponse {
///     status: String::from("success"),
///     data: Some(HashMap::new()),
///     message: None,
///     error_type: None,
/// };
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct KiteApiResponse<T> {
    pub status: String,
    pub data: Option<T>,
    pub message: Option<String>,
    pub error_type: Option<String>,
}

/// Models for the `/session/` API group, including user session management.
///
mod session;
pub use session::UserSession;

/// Models for the `/user/` API group, handling user-specific data and settings.
///
mod user;
pub use user::{Available, Segment, SegmentKind, UserMargins, UserProfile, Utilised};

/// Models for the `/orders/` API group, facilitating order placement, modification,
/// and status checks.
///
mod order;
mod order_enums;
pub use order::{Order, OrderReceipt, Trade};
pub use order_enums::{
    OrderStatus, OrderType, OrderValidity, OrderVariety, ProductType, TransactionType,
};

/// Models for the `/portfolio/` API group, managing holdings and positions.
///
mod portfolio;
pub use portfolio::{Auction, Holding, Position, PositionConversionRequest};

/// Models for the `/instruments/` and `/quote/` API group, providing market data
/// and instrument information.
///
mod market;
pub(crate) use market::KiteQuote;
pub use market::{FullQuote, Instrument, LTPQuote, OHLCQuote, QuoteMode};

/// Models for the `/margins/` and `/charges/` API group, dealing with margin
/// requirements and charges.
///
mod margins;
pub(crate) use margins::{
    BasketMargin, Charges, OrderCharges, OrderChargesRequest, OrderMargin, OrderMarginRequest, GST,
    PNL,
};

/// Enumerations for exchanges supported by Kite Connect API.
mod exchange;
pub use exchange::Exchange;
