use crate::{apis, ws};
use futures_util::StreamExt;
use std::collections::HashMap;
use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::{Arc, RwLock};
use tracing::{debug, error};

type Handler = Arc<
    dyn Fn(
            Arc<apis::client::Client>,
            ws::models::Event,
        ) -> Pin<Box<dyn Future<Output = crate::errors::Result<()>> + Send>>
        + Send
        + Sync,
>;

/// `AriClient` is a client for interacting with the Asterisk REST Interface (ARI).
/// It manages the connection to the ARI and handles events.
#[derive(Clone)]
pub struct AriClient {
    client: Arc<apis::client::Client>,
    ws: Arc<tokio::sync::Mutex<ws::client::Client>>,
    event_handlers: Arc<RwLock<HashMap<String, Handler>>>,
}

impl AriClient {
    /// Creates a new `AriClient` with the given configuration.
    pub fn with_config(config: crate::config::Config) -> Self {
        AriClient {
            client: Arc::new(apis::client::Client::with_config(config.clone())),
            ws: Arc::new(tokio::sync::Mutex::new(ws::client::Client::with_config(
                config,
            ))),
            event_handlers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Registers a handler for unknown events.
    pub fn on_unknown_event<F, Fut>(&mut self, handler: F) -> &mut Self
    where
        F: Fn(Arc<apis::client::Client>, ws::models::Event) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = crate::errors::Result<()>> + Send + 'static,
    {
        self.event_handlers.write().unwrap().insert(
            "Unknown".to_string(),
            Arc::new(move |client, event| Box::pin(handler(client, event))),
        );
        self
    }

    /// Registers a handler for a specific event.
    pub fn on_event<F, Fut>(&mut self, key: impl Into<String>, handler: F) -> &mut Self
    where
        F: Fn(Arc<apis::client::Client>, ws::models::Event) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = crate::errors::Result<()>> + Send + 'static,
    {
        self.event_handlers.write().unwrap().insert(
            key.into(),
            Arc::new(move |client, event| Box::pin(handler(client, event))),
        );
        self
    }

    /// Starts the ARI client and begins listening for events.
    pub async fn start(
        &mut self,
        application_name: impl Into<String>,
    ) -> crate::errors::Result<()> {
        let mut stream = self
            .ws
            .lock()
            .await
            .connect(ws::params::ListenRequest::new(application_name))
            .await?;

        let event_handlers = self.event_handlers.clone();
        let client = self.client.clone();
        tokio::task::spawn(async move {
            while let Some(event) = stream.next().await {
                // Get a read lock to safely access the handlers.
                let maybe_handler = {
                    let handlers = event_handlers.read().unwrap();
                    handlers.get(&event.to_string()).cloned()
                };

                if let Some(handler) = maybe_handler {
                    match handler(client.clone(), event).await {
                        Ok(_) => {}
                        Err(e) => {
                            error!("Error handling event: {:?}", e);
                        }
                    }
                } else {
                    debug!(
                        "No handler registered for event type: {}",
                        event.to_string()
                    );
                }
            }
        });

        Ok(())
    }

    /// Stops the ARI client.
    pub async fn stop(&mut self) -> Result<(), crate::errors::AriError> {
        self.ws.lock().await.disconnect().await
    }

    /// Returns a reference to the API client.
    pub fn client(&self) -> &apis::client::Client {
        &self.client
    }

    /// Returns a reference to the API client.
    pub fn client_arc(&self) -> Arc<apis::client::Client> {
        self.client.clone()
    }
}

impl Deref for AriClient {
    type Target = apis::client::Client;
    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

/// Macro to create event handler methods for specific events.
macro_rules! create_event_handler {
    ($($event_name:ident => $event_variant:ident),*) => {
        impl AriClient {
            $(
                /// Registers a handler for the `$event_variant` event.
                pub fn $event_name<F, Fut>(&mut self, handler: F) -> &mut Self
                where
                    F: Fn(Arc<apis::client::Client>, ws::models::BaseEvent<ws::models::$event_variant>) -> Fut
                        + Send
                        + Sync
                        + 'static,
                    Fut: Future<Output = crate::errors::Result<()>> + Send + 'static,
                {
                    let handler = Arc::new(handler);
                    self.on_event(stringify!($event_variant).to_string(), move |client, event| {
                        let handler = handler.clone();
                        async move {
                            if let ws::models::Event::$event_variant(e) = event {
                                handler(client, e).await
                            } else {
                                unreachable!();
                            }
                        }
                    });
                    self
                }
            )*
        }
    };
}

create_event_handler!(
    on_application_move_failed => ApplicationMoveFailed,
    on_application_replaced => ApplicationReplaced,
    on_bridge_attended_transfer => BridgeAttendedTransfer,
    on_bridge_blind_transfer => BridgeBlindTransfer,
    on_bridge_created => BridgeCreated,
    on_bridge_destroyed => BridgeDestroyed,
    on_bridge_merged => BridgeMerged,
    on_bridge_video_source_changed => BridgeVideoSourceChanged,
    on_channel_caller_id => ChannelCallerId,
    on_channel_connected_line => ChannelConnectedLine,
    on_channel_created => ChannelCreated,
    on_channel_destroyed => ChannelDestroyed,
    on_channel_dialplan => ChannelDialplan,
    on_channel_dtmf_received => ChannelDtmfReceived,
    on_channel_entered_bridge => ChannelEnteredBridge,
    on_channel_hangup_request => ChannelHangupRequest,
    on_channel_hold => ChannelHold,
    on_channel_left_bridge => ChannelLeftBridge,
    on_channel_state_change => ChannelStateChange,
    on_channel_talking_finished => ChannelTalkingFinished,
    on_channel_talking_started => ChannelTalkingStarted,
    on_channel_tone_detected => ChannelToneDetected,
    on_channel_unhold => ChannelUnhold,
    on_channel_user_event => ChannelUserEvent,
    on_channel_var_set => ChannelVarSet,
    on_contact_info => ContactInfo,
    on_contact_status_change => ContactStatusChange,
    on_device_state_changed => DeviceStateChanged,
    on_dial => Dial,
    on_endpoint_state_change => EndpointStateChange,
    on_missing_params => MissingParams,
    on_peer => Peer,
    on_peer_status_change => PeerStatusChange,
    on_playback_continuing => PlaybackContinuing,
    on_playback_finished => PlaybackFinished,
    on_playback_started => PlaybackStarted,
    on_recording_failed => RecordingFailed,
    on_recording_finished => RecordingFinished,
    on_recording_started => RecordingStarted,
    on_stasis_end => StasisEnd,
    on_stasis_start => StasisStart,
    on_text_message_received => TextMessageReceived
);
