use std::future::Future;
use std::io;

use std::pin::Pin;
use std::task::Poll;

use crate::kite::ticker::stream::{StreamState, SubscriptionStream};

// use futures::stream;
use futures_util::{SinkExt, Stream, StreamExt};

use stubborn_io::tokio::{StubbornIo, UnderlyingIo};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tracing::{debug, error, info};
use tungstenite::client::IntoClientRequest;

pub struct TickerStream {
    /// WebSocket stream
    pub ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    /// Stream state
    pub stream_state: StreamState,
}

impl UnderlyingIo<StreamState> for TickerStream
where
    StreamState: IntoClientRequest + Clone + Send + Unpin + 'static,
    SubscriptionStream: From<StreamState>,
{
    fn establish(
        stream_state: StreamState,
    ) -> Pin<Box<dyn Future<Output = io::Result<Self>> + Send>> {
        Box::pin(async move {
            // TODO: Fix `unwrap`
            let request = stream_state.clone().into_client_request().unwrap();
            let kite_uri = format!("{}", request.uri());
            match tokio_tungstenite::connect_async(kite_uri).await {
                Ok((mut ws_stream, response)) => {
                    info!("Connected to the server");
                    info!("Response HTTP code: {}", response.status());
                    info!("Response contains the following headers:");
                    for (header, value) in response.headers() {
                        info!("* {}: {:?}", header, value);
                    }
                    let mut subscribe_stream = SubscriptionStream::from(stream_state.clone());
                    while let Some(maybe_msg) = subscribe_stream.next().await {
                        match maybe_msg {
                            Ok(msg) => {
                                debug!("Ticker request: {}", msg);
                                match ws_stream.send(msg).await {
                                    Ok(_) => (),
                                    Err(e) => error!("Error sending a ticker request: {}", e),
                                }
                            }
                            Err(e) => {
                                error!("Error serializing TickerRequest: {}", e)
                            }
                        }
                    }
                    Ok(TickerStream {
                        ws_stream,
                        stream_state: stream_state,
                    })
                }
                Err(e) => Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Big problem := {}", e),
                )),
            }
        })
    }
}

pub struct WebSocketClient(StubbornIo<TickerStream, StreamState>);

impl WebSocketClient {
    pub async fn connect(stream_state: StreamState) -> io::Result<Self> {
        match StubbornIo::connect(stream_state).await {
            Ok(stubborn) => Ok(WebSocketClient(stubborn)),
            Err(e) => Err(e),
        }
    }
}

impl Stream for WebSocketClient {
    type Item = Result<tungstenite::protocol::Message, tungstenite::Error>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.0.ws_stream).poll_next(cx)
    }
}
