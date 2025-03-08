use crate::config::Config;
use crate::errors::AriError;
use crate::ws::{models, params};
use futures_util::{SinkExt, StreamExt as _};
use rand::random;
use std::time::Duration;
use tokio::time::interval;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::Stream;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tokio_util::sync::CancellationToken;
use tracing::{debug, trace, warn};
use url::Url;

/// WebSocket client for ARI.
///
/// This struct manages the WebSocket connection to the ARI server.
#[derive(Clone, Debug)]
pub struct Client {
    config: Config,
    stop_signal: CancellationToken,
}

impl Drop for Client {
    /// Cancels the stop signal when the client is dropped.
    fn drop(&mut self) {
        self.stop_signal.cancel();
    }
}

impl Client {
    /// Creates a new `Client` with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration for the ARI client.
    ///
    /// # Returns
    ///
    /// A new instance of `Client`.
    pub fn with_config(config: Config) -> Self {
        Self {
            config,
            stop_signal: CancellationToken::new(),
        }
    }

    /// Disconnects the WebSocket client.
    pub fn disconnect(&self) {
        self.stop_signal.cancel()
    }

    /// Connects to the ARI WebSocket and starts listening for events.
    ///
    /// # Arguments
    ///
    /// * `request` - The parameters for the listen request.
    ///
    /// # Returns
    ///
    /// A `Result` containing a stream of ARI events or an `AriError`.
    pub async fn connect(
        &self,
        request: params::ListenRequest,
    ) -> Result<impl Stream<Item = models::Event>, AriError> {
        let mut url = Url::parse(self.config.api_base.clone().as_str())?;

        url.set_scheme(if url.scheme().starts_with("https://") {
            "wss"
        } else {
            "ws"
        })
        .unwrap();

        url.set_path("/ari/events");

        url.query_pairs_mut()
            .append_pair(
                "api_key",
                &format!("{}:{}", self.config.username, self.config.password),
            )
            .append_pair("app", request.app.as_str())
            .append_pair(
                "subscribeAll",
                request.subscribe_all.unwrap_or(true).to_string().as_str(),
            );

        debug!("connecting to ws_url: {}", url);

        // if not connect, retry!
        let ws_stream = match connect_async(url.to_string()).await {
            Ok((ws_stream, _)) => ws_stream,
            Err(e) => {
                warn!("error when connecting to the websocket: {:#?}", e);
                return Err(AriError::from(e));
            }
        };
        debug!("websocket connected");

        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        let mut interval = interval(Duration::from_millis(5000));
        let cancel_token = self.stop_signal.child_token();
        let (tx, rx) = tokio::sync::mpsc::channel(100);

        let mut closed = false;
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = cancel_token.cancelled()  => if !closed {
                        //debug!("Stop signal received, leaving the loop!");
                        match ws_sender.close().await{
                            Ok(_) => {
                                debug!("WS connection closed");
                                closed = true;
                            },
                            Err(e) => warn!("error when closing ws connection: {:#?}", e),
                        }
                    },
                    msg = ws_receiver.next() => {
                        match msg {
                            Some(msg) => {
                                match msg {
                                        Ok(Message::Close(close_frame)) => {
                                            debug!(
                                                "Close message received, leaving the loop! {:#?}",
                                                close_frame
                                            );
                                            break;
                                        }
                                        Ok(Message::Pong(_)) => {}
                                        Ok(Message::Ping(data)) => {
                                            let _ = ws_sender.send(Message::Pong(data)).await;
                                        }
                                        Ok(Message::Text(string_msg)) => {

                                            trace!("WS Ari Event: {:#?}", string_msg);
                                            match serde_json::from_str::<models::Event>(&string_msg){
                                                Ok(event ) => {
                                                    if tx.send(event).await.is_err() {
                                                        warn!("error when sending ARI event to the channel");
                                                        break;
                                                    }
                                                }
                                                Err(e) => warn!(
                                                        "error when deserializing ARI event: {:#?}. Event: {:#?}",
                                                        e, string_msg
                                                    ),
                                            }

                                        }
                                        Err(e) => {
                                            warn!("Error when receiving websocket message: {:#?}", e);
                                            break;
                                        }
                                        _ => {
                                            warn!(
                                                "Unknown websocket message received: {:#?}",
                                                msg
                                            );
                                        }
                                    }
                            }
                            None => break,
                        }
                    }
                    _ = interval.tick() => {
                        // every 5 seconds we are sending ping to keep connection alive
                        // https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html
                        let _ = ws_sender.send(Message::Ping(random::<[u8; 32]>().to_vec().into())).await;
                        debug!("ari connection ping sent");
                    }
                }
            }
        });

        Ok(ReceiverStream::new(rx))
    }
}
