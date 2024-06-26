use crate::kite::error::{ManjaError, Result};

use chrono::NaiveDate;
use serde::Deserialize;

/// Represents a trading instrument.
///
/// Between multiple exchanges and segments, there are tens of thousands of different kinds
/// of instruments that trade. Any application that facilitates trading needs to have a
/// master list of these instruments. The instruments API provides a consolidated,
/// import-ready CSV list of instruments available for trading.
///
/// # CSV response columns
///
/// - `instrument_token`: Numerical identifier used for subscribing to live market quotes with the WebSocket API.
/// - `exchange_token`: The numerical identifier issued by the exchange representing the instrument.
/// - `tradingsymbol`: Exchange tradingsymbol of the instrument.
/// - `name`: Name of the company (for equity instruments). This can be `None` for non-equity instruments.
/// - `last_price`: Last traded market price.
/// - `expiry`: Expiry date (for derivatives). Optional because it may not be presYent for some instruments.
/// - `strike`: Strike price (for options). Optional because it may not be present for some instruments.
/// - `tick_size`: Value of a single price tick.
/// - `lot_size`: Quantity of a single lot.
/// - `instrument_type`: Type of the instrument (e.g., EQ, FUT, CE, PE).
/// - `segment`: Segment the instrument belongs to.
/// - `exchange`: Exchange where the instrument is traded.
#[derive(Debug, Deserialize, Clone)]
struct Instrument {
    /// Numerical identifier used for subscribing to live market quotes with the WebSocket API.
    instrument_token: i32,
    /// The numerical identifier issued by the exchange representing the instrument.
    exchange_token: String,
    /// Exchange tradingsymbol of the instrument.
    tradingsymbol: String,
    /// Name of the company (for equity instruments). This can be `None` for non-equity instruments.
    name: Option<String>,
    /// Last traded market price.
    last_price: f64,
    /// Expiry date (for derivatives). Optional because it may not be present for some instruments.
    expiry: Option<NaiveDate>,
    /// Strike price (for options). Optional because it may not be present for some instruments.
    strike: Option<f64>,
    /// Value of a single price tick.
    tick_size: f64,
    /// Quantity of a single lot.
    lot_size: i64,
    /// Type of the instrument (e.g., EQ, FUT, CE, PE).
    instrument_type: String,
    /// Segment the instrument belongs to.
    segment: String,
    /// Exchange where the instrument is traded.
    exchange: String,
}

/// Parses the CSV response into a vector of `Instrument`.
///
/// The instrument list API returns a gzipped CSV dump of instruments across all exchanges that can
/// be imported into a database. The dump is generated once every day and hence last_price is not real time.
///
/// # Arguments
///
/// * `data` - A string slice that holds the CSV data.
///
/// # Returns
///
/// A `Result` containing a vector of `Instrument` structs if successful, or a `ManjaError` otherwise.
///
/// # Errors
///
/// This function will return an error if the CSV data cannot be parsed.
fn parse_instruments(data: &str) -> Result<Vec<Instrument>> {
    let mut rdr = csv::Reader::from_reader(data.as_bytes());
    let mut records = Vec::new();

    for result in rdr.deserialize() {
        let record: Instrument =
            result.map_err(|_| ManjaError::Internal(format!("CSV parse error")))?;
        records.push(record);
    }

    Ok(records)
}
