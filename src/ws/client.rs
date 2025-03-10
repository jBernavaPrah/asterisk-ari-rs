use crate::config::Config;
use crate::errors::AriError;
use crate::ws::{models, params};
use futures_util::{SinkExt, StreamExt as _};
use rand::random;
use std::time::Duration;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::Stream;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, trace, warn};
use url::Url;
#[derive(Debug)]
pub struct Client {
    config: Config,
    stop_signal: CancellationToken,
    _ws_join_handle: Option<tokio::task::JoinHandle<Result<(), AriError>>>,
}

impl Drop for Client {
    fn drop(&mut self) {
        self.stop_signal.cancel();
    }
}

impl Client {
    pub fn with_config(config: Config) -> Self {
        Self {
            config,
            stop_signal: CancellationToken::new(),
            _ws_join_handle: None,
        }
    }

    /// Disconnects the WebSocket client and waits for the join handler to finish.
    pub async fn disconnect(&mut self) -> Result<(), AriError> {
        self.stop_signal.cancel();

        if let Some(handle) = self._ws_join_handle.take() {
            return handle.await.unwrap_or_else(|e| {
                warn!("error when waiting for ws join handle: {:#?}", e);
                Err(AriError::Internal(e.to_string()))
            });
        }

        Ok(())
    }

    /// Connects to the ARI WebSocket and starts listening for events.
    pub async fn connect(
        &mut self,
        request: params::ListenRequest,
    ) -> Result<impl Stream<Item = models::Event>, AriError> {
        let mut url = Url::parse(self.config.api_base.as_str())?;

        url.set_scheme(if url.scheme().starts_with("https") {
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

        let ws_stream = match connect_async(url.to_string()).await {
            Ok((ws_stream, _)) => ws_stream,
            Err(e) => {
                warn!("error when connecting to the websocket: {:#?}", e);
                return Err(e.into());
            }
        };
        debug!("websocket connected");

        let (mut ws_sender, mut ws_receiver) = ws_stream.split();
        let mut refresh_interval = tokio::time::interval(Duration::from_millis(5000));
        let cancel_token = self.stop_signal.child_token();
        let (tx, rx) = tokio::sync::mpsc::channel(100);

        self._ws_join_handle = Some(tokio::spawn(async move {
            let mut connected = true;

            'outer: loop {
                while connected {
                    tokio::select! {
                        _ = cancel_token.cancelled() => {
                                if let Err(e) = ws_sender.close().await {
                                    return Err(AriError::from(e));
                                }
                                debug!("WS connection closed due to cancellation");
                                break 'outer;

                        },
                        msg = ws_receiver.next() =>  {

                                let Some(msg) = msg else {
                                    // If the receiver returns None, mark connection as lost.
                                    connected = false;
                                    continue;
                                };

                                match msg {
                                    Ok(Message::Close(close_frame)) => {
                                        warn!("Close message received: {:#?}", close_frame);
                                        connected = false;
                                        continue;
                                    }
                                    Ok(Message::Pong(_)) => {},
                                    Ok(Message::Ping(data)) => {
                                        let _ = ws_sender.send(Message::Pong(data)).await;
                                    }
                                    Ok(Message::Text(string_msg)) => {
                                        trace!("WS Ari Event: {:#?}", string_msg);
                                        match serde_json::from_str::<models::Event>(&string_msg) {
                                            Ok(event) => {
                                                if tx.send(event).await.is_err() {
                                                    debug!("Receiver closed the connection. Stopping WS client");
                                                    break;
                                                }
                                            }
                                            Err(e) => warn!("error when deserializing ARI event: {:#?}. Event: {:#?}", e, string_msg),
                                        }
                                    }
                                    Err(e) => {
                                        warn!("Error when receiving websocket message: {:#?}", e);
                                        connected = false;
                                        continue;
                                    }
                                    _ => {}
                                }

                        },
                        _ = refresh_interval.tick() => {

                                let _ = ws_sender.send(Message::Ping(random::<[u8; 32]>().to_vec().into())).await;
                                debug!("ARI connection ping sent");

                        }
                    }
                }

                let mut i = 0;
                loop {
                    i += 1;
                    if cancel_token.is_cancelled() {
                        debug!("Cancellation detected during reconnection attempts");
                        break 'outer;
                    }
                    info!("Attempting to reconnect ({i})");

                    match connect_async(url.to_string()).await {
                        Ok((ws_stream, _)) => {
                            info!("Reconnected successfully");
                            connected = true;
                            let (new_ws_sender, new_ws_receiver) = ws_stream.split();
                            ws_sender = new_ws_sender;
                            ws_receiver = new_ws_receiver;
                            continue 'outer;
                        }
                        Err(e) => {
                            error!("Failed to reconnect ({i}): {e}");
                        }
                    }
                    tokio::time::sleep(std::cmp::min(
                        Duration::from_millis(500 * i),
                        Duration::from_secs(90),
                    ))
                    .await;
                }
            }

            Ok(())
        }));

        Ok(ReceiverStream::new(rx))
    }
}
