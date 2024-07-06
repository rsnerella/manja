use crate::kite::ticker::models::Mode;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
///
/// Websocket request actions
///
enum RequestActions {
    Subscribe,
    Unsubscribe,
    Mode,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
///
/// Websocket request data
///
enum RequestData {
    InstrumentTokens(Vec<u32>),
    InstrumentTokensWithMode(Mode, Vec<u32>),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
///
/// Websocket request structure
///
pub struct TickerRequest {
    a: RequestActions,
    v: RequestData,
}

impl TickerRequest {
    fn new(action: RequestActions, value: RequestData) -> TickerRequest {
        TickerRequest {
            a: action,
            v: value,
        }
    }

    ///
    /// Subscribe to a list of instrument tokens
    ///
    pub fn subscribe(instrument_tokens: Vec<u32>) -> TickerRequest {
        TickerRequest::new(
            RequestActions::Subscribe,
            RequestData::InstrumentTokens(instrument_tokens),
        )
    }

    ///
    /// Subscribe to a list of instrument tokens with mode
    ///
    pub fn subscribe_with_mode(instrument_tokens: Vec<u32>, mode: Mode) -> TickerRequest {
        TickerRequest::new(
            RequestActions::Mode,
            RequestData::InstrumentTokensWithMode(mode, instrument_tokens),
        )
    }

    ///
    /// Unsubscribe from a list of instrument tokens
    ///
    pub fn unsubscribe(instrument_tokens: Vec<u32>) -> TickerRequest {
        TickerRequest::new(
            RequestActions::Unsubscribe,
            RequestData::InstrumentTokens(instrument_tokens),
        )
    }
}

impl ToString for TickerRequest {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("failed to serialize TickerInput to JSON")
    }
}
