use crate::kite::error::Result;
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
struct Instrument {
    instrument_token: i32,
    exchange_token: String,
    tradingsymbol: String,
    name: Option<String>, // This can be None for non-equity instruments
    last_price: f64,
    expiry: Option<NaiveDate>, // Optional because it may not be present for some instruments
    strike: Option<f64>,
    tick_size: f64,
    lot_size: i64,
    instrument_type: String,
    segment: String,
    exchange: String,
}

// Function to parse the CSV response into a vector of `Instrument`
fn parse_instruments(data: &str) -> Result<Vec<Instrument>> {
    let mut rdr = csv::Reader::from_reader(data.as_bytes());
    let mut records = Vec::new();

    for result in rdr.deserialize() {
        let record: Instrument = result?;
        records.push(record);
    }

    Ok(records)
}
