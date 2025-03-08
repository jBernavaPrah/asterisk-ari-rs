use crate::{apis, ws};
use futures_util::StreamExt;
use std::collections::HashMap;
use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;
use tracing::debug;

type Handler = Arc<
    dyn Fn(
        Arc<apis::client::Client>,
        ws::models::Event,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>>
    + Send
    + Sync,
>;

/// `AriClient` is a client for interacting with the Asterisk REST Interface (ARI).
/// It manages the connection to the ARI and handles events.
#[derive(Clone)]
pub struct AriClient {
    client: Arc<apis::client::Client>,
    ws: Arc<ws::client::Client>,
    event_handlers: HashMap<
        String,
        Handler,
    >,
}

impl AriClient {
    /// Creates a new `AriClient` with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - A configuration object for the ARI client.
    ///
    /// # Returns
    ///
    /// A new instance of `AriClient`.
    pub fn with_config(config: crate::config::Config) -> Self {
        AriClient {
            client: Arc::new(apis::client::Client::with_config(config.clone())),
            ws: Arc::new(ws::client::Client::with_config(config)),
            event_handlers: HashMap::new(),
        }
    }

    /// Registers a handler for unknown events.
    ///
    /// # Arguments
    ///
    /// * `handler` - A function that handles unknown events.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `AriClient`.
    pub fn on_unknown_event<F, Fut>(&mut self, handler: F) -> &mut Self
    where
        F: Fn(Arc<apis::client::Client>, ws::models::Event) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.event_handlers.insert(
            "Unknown".to_string(),
            Arc::new(move |client, event| Box::pin(handler(client, event))),
        );
        self
    }

    /// Registers a handler for a specific event.
    ///
    /// # Arguments
    ///
    /// * `key` - The event type as a string.
    /// * `handler` - A function that handles the event.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `AriClient`.
    pub fn on_event<F, Fut>(&mut self, key: String, handler: F) -> &mut Self
    where
        F: Fn(Arc<apis::client::Client>, ws::models::Event) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.event_handlers.insert(
            key,
            Arc::new(move |client, event| Box::pin(handler(client, event))),
        );
        self
    }

    /// Starts the ARI client and begins listening for events.
    ///
    /// # Arguments
    ///
    /// * `application_name` - The name of the ARI application.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    pub async fn start(&self, application_name: String) -> crate::errors::Result<()> {
        let mut stream = self
            .ws
            .connect(ws::params::ListenRequest::new(application_name))
            .await?;

        while let Some(event) = stream.next().await {
            if let Some(handler) = self.event_handlers.get(&event.to_string()) {
                handler(self.client.clone(), event).await;
            } else {
                // Log or handle unmatched events if necessary
                debug!(
                    "No handler registered for event type: {}",
                    event.to_string()
                );
            }
        }

        Ok(())
    }

    /// Stops the ARI client.
    pub fn stop(&self) {
        self.ws.disconnect();
    }

    /// Returns a reference to the WebSocket client.
    ///
    /// # Returns
    ///
    /// A reference to the WebSocket client.
    pub fn ws(&self) -> &ws::client::Client {
        &self.ws
    }

    /// Returns a reference to the API client.
    ///
    /// # Returns
    ///
    /// A reference to the API client.
    pub fn client(&self) -> &apis::client::Client {
        &self.client
    }
}

impl Deref for AriClient {
    type Target = apis::client::Client;

    /// Dereferences the `AriClient` to the underlying API client.
    ///
    /// # Returns
    ///
    /// A reference to the API client.
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
                ///
                /// # Arguments
                ///
                /// * `handler` - A function that handles the event.
                ///
                /// # Returns
                ///
                /// A mutable reference to the `AriClient`.
                pub fn $event_name<F, Fut>(&mut self, handler: F) -> &mut Self
                where
                    F: Fn(Arc<apis::client::Client>, ws::models::BaseEvent<ws::models::$event_variant>) -> Fut
                        + Send
                        + Sync
                        + 'static,
                    Fut: Future<Output = ()> + Send + 'static,
                {
                    let handler = Arc::new(handler); // Wrap handler in Arc for shared ownership

                    self.on_event(stringify!($event_variant).to_string(), move |client, event| {
                        let handler = handler.clone(); // Clone Arc for use in the async block
                        async move {
                            if let ws::models::Event::$event_variant(e) = event {
                                handler(client, e).await;
                            } else {
                                unreachable!(); // Ensure safety for mismatched events
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
