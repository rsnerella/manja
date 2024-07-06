use std::collections::HashMap;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::kite::connect::models::UserSession;
use crate::kite::error::Result;
use crate::kite::ticker::models::Mode;

use futures_util::Stream;
use secrecy::{ExposeSecret, Secret};
use tungstenite::{client::IntoClientRequest, Message};

use super::models::TickerRequest;

pub const KITECONNECT_WSS_API_BASE: &str = "wss://ws.kite.trade";

#[derive(Debug, Clone)]
pub struct KiteStreamCredentials {
    api_key: Secret<String>,
    access_token: Secret<String>,
}

impl KiteStreamCredentials {
    pub fn from_parts<InS>(api_key: InS, access_token: InS) -> Self
    where
        InS: Into<String>,
    {
        Self {
            api_key: Secret::new(api_key.into()),
            access_token: Secret::new(access_token.into()),
        }
    }

    fn to_query_params(&self) -> String {
        format!(
            "api_key={}&access_token={}",
            self.api_key.expose_secret(),
            self.access_token.expose_secret()
        )
    }
}

impl From<UserSession> for KiteStreamCredentials {
    fn from(value: UserSession) -> Self {
        Self {
            api_key: value.api_key,
            access_token: value.access_token,
        }
    }
}

type InstrumentToken = u32;
type Subscription = HashMap<Mode, Vec<InstrumentToken>>;

#[derive(Debug, Clone)]
pub struct StreamState {
    api_base: String,
    credentials: KiteStreamCredentials,
    subscription: Subscription,
}

impl StreamState {
    pub fn from_parts<InS>(api_base: InS, api_key: InS, access_token: InS) -> Self
    where
        InS: Into<String>,
    {
        Self {
            api_base: api_base.into(),
            credentials: KiteStreamCredentials::from_parts(api_key, access_token),
            subscription: Default::default(),
        }
    }

    pub fn from_credentials(credentials: KiteStreamCredentials) -> Self {
        let api_base = std::env::var("KITECONNECT_WSS_API_BASE")
            .unwrap_or_else(|_| KITECONNECT_WSS_API_BASE.to_string())
            .into();
        Self {
            api_base,
            credentials,
            subscription: Default::default(),
        }
    }

    pub fn to_subcription_stream(self) -> SubscriptionStream {
        SubscriptionStream::with_subscription(&self.subscription)
    }

    pub fn subscribe_token(mut self, mode: Mode, token: u32) -> Self {
        if let Some(vec) = self.subscription.get_mut(&mode) {
            vec.push(token);
        } else {
            self.subscription.insert(mode, vec![token]);
        }
        self
    }

    pub fn to_uri(&self) -> String {
        format!("{}?{}", self.api_base, self.credentials.to_query_params())
    }
}

impl IntoClientRequest for StreamState {
    fn into_client_request(self) -> tungstenite::Result<tungstenite::handshake::client::Request> {
        format!("{}?{}", self.api_base, self.credentials.to_query_params()).into_client_request()
    }
}

pub struct SubscriptionStream {
    pub data: Subscription,
    pub keys: Vec<Mode>,
    pub current_key_idx: usize,
}

impl SubscriptionStream {
    pub fn with_subscription(subcription: &Subscription) -> Self {
        Self {
            data: subcription.clone(),
            keys: subcription.keys().cloned().collect(),
            current_key_idx: 0,
        }
    }
}

impl From<StreamState> for SubscriptionStream {
    fn from(value: StreamState) -> Self {
        let keys = value.subscription.keys().cloned().collect();
        Self {
            data: value.subscription,
            keys: keys,
            current_key_idx: 0,
        }
    }
}

impl Stream for SubscriptionStream {
    type Item = Result<Message>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        if this.current_key_idx >= this.keys.len() {
            // No more items to stream
            return Poll::Ready(None);
        }

        let current_key = &this.keys[this.current_key_idx];
        if let Some(tokens) = this.data.get(current_key) {
            this.current_key_idx += 1;

            let ticker_request =
                TickerRequest::subscribe_with_mode(tokens.clone(), current_key.clone());

            match serde_json::to_string(&ticker_request) {
                Ok(json) => Poll::Ready(Some(Ok(Message::Text(json)))),
                Err(e) => Poll::Ready(Some(Err(e.into()))),
            }
        } else {
            this.current_key_idx += 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
