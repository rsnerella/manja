use serde::{Deserialize, Serialize};

/// Default `T` is `HashMap`
#[derive(Serialize, Deserialize, Debug)]
pub struct KiteApiResponse<T> {
    pub status: String,
    pub data: Option<T>,
    pub message: Option<String>,
    pub error_type: Option<String>,
}

// Models for the `/session` API group
mod session;
pub use session::UserSession;

// Models for the `/user/` API group
mod user;
pub use user::{Available, Segment, SegmentKind, UserMargins, UserProfile, Utilised};

// Models for the `/orders/ API group
mod order;
mod order_enums;
pub use order::{Order, OrderReceipt, Trade};

// Models for the `/portfolio/` API group
mod portfolio;
pub use portfolio::{Auction, Holding, Position, PositionConversionRequest};

// Models for the `/instruments/` and `/quote/` API group
mod market;
pub(crate) use market::KiteQuote;
pub use market::{FullQuote, Instrument, LTPQuote, OHLCQuote, QuoteMode};

// Enums for `exchange`
mod exchange;
pub use exchange::Exchange;
