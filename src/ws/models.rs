use crate::apis::bridges::models::Bridge;
use crate::apis::channels::models::Channel;
use crate::apis::device_stats::models::DeviceState;
use crate::apis::endpoints::models::{Endpoint, TextMessage};
use crate::apis::playbacks::models::Playback;
use crate::apis::recordings::models::LiveRecording;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::fmt;

/// BaseEvent
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct BaseEvent<T> {
    /// The unique ID for the Asterisk instance that raised this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asterisk_id: Option<String>,

    /// Name of the application receiving the event.
    pub application: String,

    /// Time at which this event was created. E.g. 2020-11-22T20:12:51.214+0000
    pub timestamp: DateTime<Utc>,

    /// The event data.
    #[serde(flatten)]
    pub data: T,
}

/// ApplicationMoveFailed
/// Notification that trying to move a channel to another Stasis application failed.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct ApplicationMoveFailed {
    #[serde(rename = "channel")]
    pub channel: Channel,
    #[serde(rename = "destination")]
    pub destination: String,
    #[serde(rename = "args")]
    pub args: Vec<String>,
}

/// ApplicationMoveFailed
/// Notification that another WebSocket has taken over for an application.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct ApplicationReplaced {}

/// BridgeAttendedTransfer
/// Notification that an attended transfer has occurred.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct BridgeAttendedTransfer {
    /// First leg of the transferer
    #[serde(rename = "transferer_first_leg")]
    pub transferer_first_leg: Channel,
    /// Second leg of the transferer
    #[serde(rename = "transferer_second_leg")]
    pub transferer_second_leg: Channel,

    /// The channel that is replacing transferer_first_leg in the swap
    #[serde(rename = "replace_channel")]
    pub replace_channel: Option<Channel>,

    /// The channel that is being transferred
    #[serde(rename = "transferee")]
    pub transferee: Option<Channel>,

    /// The channel that is being transferred to
    #[serde(rename = "transfer_target")]
    pub transfer_target: Option<Channel>,

    /// The result of the transfer attempt
    #[serde(rename = "result")]
    pub result: String,

    /// Whether the transfer was externally initiated or not
    #[serde(rename = "is_external")]
    pub is_external: bool,

    /// Bridge the transferer first leg is in
    #[serde(rename = "transferer_first_leg_bridge")]
    pub transferer_first_leg_bridge: Option<Bridge>,

    /// Bridge the transferer second leg is in
    #[serde(rename = "transferer_second_leg_bridge")]
    pub transferer_second_leg_bridge: Option<Bridge>,

    /// How the transfer was accomplished
    #[serde(rename = "destination_type")]
    pub destination_type: String,

    /// Bridge that survived the merge result
    #[serde(rename = "destination_bridge")]
    pub destination_bridge: Option<String>,

    /// Application that has been transferred into
    #[serde(rename = "destination_application")]
    pub destination_application: Option<String>,

    /// First leg of a link transfer result
    #[serde(rename = "destination_link_first_leg")]
    pub destination_link_first_leg: Option<Channel>,

    /// Second leg of a link transfer result
    #[serde(rename = "destination_link_second_leg")]
    pub destination_link_second_leg: Option<Channel>,

    /// Transferer channel that survived the threeway result
    #[serde(rename = "destination_threeway_channel")]
    pub destination_threeway_channel: Option<Channel>,

    /// Bridge that survived the threeway result
    #[serde(rename = "destination_threeway_bridge")]
    pub destination_threeway_bridge: Option<Bridge>,
}

/// BridgeBlindTransfer :
/// Notification that a blind transfer has occurred.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct BridgeBlindTransfer {
    /// The channel performing the blind transfer
    #[serde(rename = "channel")]
    pub channel: Channel,

    /// The channel that is replacing transferer when the transferee(s) can not be transferred directly
    #[serde(rename = "replace_channel")]
    pub replace_channel: Option<Channel>,

    /// The channel that is being transferred
    #[serde(rename = "transferee")]
    pub transferee: Option<Channel>,

    /// The extension transferred to
    #[serde(rename = "exten")]
    pub exten: String,
    /// The context transferred to
    #[serde(rename = "context")]
    pub context: String,
    /// The result of the transfer attempt
    #[serde(rename = "result")]
    pub result: String,
    /// Whether the transfer was externally initiated or not
    #[serde(rename = "is_external")]
    pub is_external: bool,

    /// The bridge being transferred
    #[serde(rename = "bridge")]
    pub bridge: Option<Bridge>,
}

/// Notification that a bridge has been created.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct BridgeCreated {
    #[serde(rename = "bridge")]
    pub bridge: Bridge,
}

/// Notification that a bridge has been destroyed.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct BridgeDestroyed {
    #[serde(rename = "bridge")]
    pub bridge: Bridge,
}

#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct BridgeMerged {
    #[serde(rename = "bridge")]
    pub bridge: Bridge,
    #[serde(rename = "bridge_from")]
    pub bridge_from: Bridge,
}

/// Notification that the source of video in a bridge has changed.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct BridgeVideoSourceChanged {
    #[serde(rename = "bridge")]
    pub bridge: Bridge,
    #[serde(rename = "old_video_source_id")]
    pub old_video_source_id: Option<String>,
}

/// Channel changed Caller ID.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct ChannelCallerId {
    /// The integer representation of the Caller Presentation value.
    #[serde(rename = "caller_presentation")]
    pub caller_presentation: u32,

    /// The text representation of the Caller Presentation value.
    #[serde(rename = "caller_presentation_txt")]
    pub caller_presentation_txt: String,

    /// The channel that changed Caller ID.
    #[serde(rename = "channel")]
    pub channel: Channel,
}

/// Channel changed Connected Line.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct ChannelConnectedLine {
    /// The channel whose connected line has changed.
    #[serde(rename = "channel")]
    pub channel: Channel,
}

/// ChannelCreated: Notification that a channel has been created.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct ChannelCreated {
    #[serde(rename = "channel")]
    pub channel: Channel,
}

/// Notification that a channel has been destroyed.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct ChannelDestroyed {
    #[serde(rename = "cause")]
    pub cause: u32,
    /// Text representation of the cause of the hangup
    #[serde(rename = "cause_txt")]
    /// Text representation of the cause of the hangup
    pub cause_txt: String,
    #[serde(rename = "channel")]
    pub channel: Channel,
}
/// Channel changed location in the dialplan.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct ChannelDialplan {
    /// The channel that changed dialplan location.
    #[serde(rename = "channel")]
    pub channel: Channel,
    /// The application about to be executed.
    #[serde(rename = "dialplan_app")]
    pub dialplan_app: String,
    /// The data to be passed to the application.
    #[serde(rename = "dialplan_app_data")]
    pub dialplan_app_data: String,
}

/// ChannelDtmfReceived :
/// DTMF received on a channel.  
/// This event is sent when the DTMF ends. There is no notification about the start of DTMF
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct ChannelDtmfReceived {
    /// DTMF digit received (0-9, A-E, # or *)
    #[serde(rename = "digit")]
    pub digit: String,
    /// Number of milliseconds DTMF was received
    #[serde(rename = "duration_ms")]
    pub duration_ms: u32,
    /// The channel on which DTMF was received
    #[serde(rename = "channel")]
    pub channel: Channel,
}

/// Notification that a channel has entered a bridge.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct ChannelEnteredBridge {
    #[serde(rename = "bridge")]
    pub bridge: Bridge,
    #[serde(rename = "channel")]
    pub channel: Option<Channel>,
}

/// A hangup was requested on the channel.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct ChannelHangupRequest {
    /// Integer representation of the cause of the hangup.
    #[serde(rename = "cause")]
    pub cause: u32,
    /// Whether the hangup request was a soft hangup request.
    #[serde(rename = "soft")]
    pub soft: Option<bool>,
    /// The channel on which the hangup was requested.
    #[serde(rename = "channel")]
    pub channel: Channel,
}

/// A channel initiated a media hold.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct ChannelHold {
    #[serde(rename = "channel")]
    pub channel: Channel,
    /// The music on hold class that the initiator requested.
    #[serde(rename = "musicclass")]
    pub musicclass: Option<String>,
}

/// Notification that a channel has left a bridge.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct ChannelLeftBridge {
    #[serde(rename = "bridge")]
    pub bridge: Bridge,
    #[serde(rename = "channel")]
    pub channel: Channel,
}

/// Notification of a channel's state change.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct ChannelStateChange {
    #[serde(rename = "channel")]
    pub channel: Channel,
}

/// Talking is no longer detected on the channel.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct ChannelTalkingFinished {
    #[serde(rename = "channel")]
    pub channel: Channel,
    #[serde(rename = "duration")]
    pub duration: u32,
}

/// Talking was detected on the channel.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct ChannelTalkingStarted {
    #[serde(rename = "channel")]
    pub channel: Channel,
}

/// Tone was detected on the channel.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct ChannelToneDetected {
    #[serde(rename = "channel")]
    pub channel: Channel,
}

/// A channel initiated a media unhold.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct ChannelUnhold {
    #[serde(rename = "channel")]
    pub channel: Channel,
}

/// User-generated event with additional user-defined fields in the object.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct ChannelUserEvent {
    /// The name of the user event.
    #[serde(rename = "eventname")]
    pub event_name: String,

    ///  A channel that is signaled with the user event.
    #[serde(rename = "channel")]
    pub channel: Option<Channel>,
    /// A bridge that is signaled with the user event.
    #[serde(rename = "bridge")]
    pub bridge: Option<Bridge>,
    /// A endpoint that is signaled with the user event.
    #[serde(rename = "endpoint")]
    pub endpoint: Option<Endpoint>,
    /// Custom Userevent data
    #[serde(rename = "userevent")]
    pub user_event: Option<serde_json::Value>,
}

/// Channel variable changed.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct ChannelVarSet {
    /// The variable that changed.
    #[serde(rename = "variable")]
    pub variable: String,
    /// The new value of the variable.
    #[serde(rename = "value")]
    pub value: String,
    /// The channel on which the variable was set.
    /// If missing, the variable is a global variable.
    #[serde(rename = "channel")]
    pub channel: Option<Channel>,
}

/// Detailed information about a contact on an endpoint.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct ContactInfo {
    /// The location of the contact.
    #[serde(rename = "uri")]
    pub uri: String,
    /// The current status of the contact.
    #[serde(rename = "contact_status")]
    pub contact_status: String,
    /// The Address of Record this contact belongs to.
    #[serde(rename = "aor")]
    pub aor: String,
    /// Current round trip time, in microseconds, for the contact.
    // todo: change this to duration?
    #[serde(rename = "roundtrip_usec")]
    pub roundtrip_usec: Option<String>,
}

/// The state of a contact on an endpoint has changed.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct ContactStatusChange {
    #[serde(rename = "endpoint")]
    pub endpoint: Endpoint,
    #[serde(rename = "contact_info")]
    pub contact_info: ContactInfo,
}

/// Notification that a device state has changed.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct DeviceStateChanged {
    /// Device state object
    #[serde(rename = "device_state")]
    pub device_state: DeviceState,
}

/// Dialing state has changed.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct Dial {
    /// The calling channel.
    #[serde(rename = "caller")]
    pub caller: Option<Channel>,

    /// The dialed channel.
    #[serde(rename = "peer")]
    pub peer: Option<Channel>,
    /// Forwarding target requested by the original dialed channel.
    #[serde(rename = "forward")]
    pub forward: Option<String>,
    #[serde(rename = "forwarded")]
    pub forwarded: Option<Channel>,
    /// The dial string for calling the peer channel.
    #[serde(rename = "dialstring")]
    pub dialstring: Option<String>,
    /// Current status of the dialing attempt to the peer.
    #[serde(rename = "dialstatus")]
    pub dialstatus: String,
}

/// Endpoint state changed.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct EndpointStateChange {
    #[serde(rename = "endpoint")]
    pub endpoint: Endpoint,
}

/// Error event sent when required params are missing.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct MissingParams {
    /// A list of the missing parameters
    #[serde(rename = "params")]
    pub params: Vec<String>,
}

/// Detailed information about a remote peer that communicates with Asterisk.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct Peer {
    /// The current state of the peer. Note that the values of the status are dependent on the underlying peer technology.
    #[serde(rename = "peer_status")]
    pub peer_status: String,
    /// An optional reason associated with the change in peer_status.
    #[serde(rename = "cause")]
    pub cause: Option<String>,
    /// The IP address of the peer.
    #[serde(rename = "address")]
    pub address: Option<String>,
    /// The port of the peer.
    #[serde(rename = "port")]
    pub port: Option<String>,
    /// The last known time the peer was contacted.
    #[serde(rename = "time")]
    pub time: Option<String>,
}

/// The state of a peer associated with an endpoint has changed.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct PeerStatusChange {
    #[serde(rename = "endpoint")]
    pub endpoint: Endpoint,
    #[serde(rename = "peer")]
    pub peer: Peer,
}

/// Event showing the continuation of a media playback operation from one media URI to the next in the list.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct PlaybackContinuing {
    /// Playback control object
    #[serde(rename = "playback")]
    pub playback: Playback,
}

/// Event showing the completion of a media playback operation.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct PlaybackFinished {
    /// Playback control object
    #[serde(rename = "playback")]
    pub playback: Playback,
}

/// Event showing the start of a media playback operation.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct PlaybackStarted {
    #[serde(rename = "playback")]
    pub playback: Playback,
}

/// Event showing failure of a recording operation.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct RecordingFailed {
    /// Recording control object
    #[serde(rename = "recording")]
    pub recording: LiveRecording,
}

/// Event showing the completion of a recording operation.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct RecordingFinished {
    #[serde(rename = "recording")]
    pub recording: LiveRecording,
}

/// Event showing the start of a recording operation.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct RecordingStarted {
    #[serde(rename = "recording")]
    pub recording: LiveRecording,
}

/// Notification that a channel has left a Stasis application.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct StasisEnd {
    #[serde(rename = "channel")]
    pub channel: Channel,
}

/// Notification that a channel has entered a Stasis application.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct StasisStart {
    #[serde(rename = "args")]
    pub args: Vec<String>,
    #[serde(rename = "channel")]
    pub channel: Channel,
    #[serde(rename = "replace_channel")]
    pub replace_channel: Option<Channel>,
}

/// A text message was received from an endpoint.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct TextMessageReceived {
    #[serde(rename = "message")]
    pub message: TextMessage,

    #[serde(rename = "endpoint")]
    pub endpoint: Option<Endpoint>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Event {
    ApplicationMoveFailed(BaseEvent<ApplicationMoveFailed>),
    ApplicationReplaced(BaseEvent<ApplicationReplaced>),
    BridgeAttendedTransfer(BaseEvent<BridgeAttendedTransfer>),
    BridgeBlindTransfer(BaseEvent<BridgeBlindTransfer>),
    BridgeCreated(BaseEvent<BridgeCreated>),
    BridgeDestroyed(BaseEvent<BridgeDestroyed>),
    BridgeMerged(BaseEvent<BridgeMerged>),
    BridgeVideoSourceChanged(BaseEvent<BridgeVideoSourceChanged>),
    ChannelCallerId(BaseEvent<ChannelCallerId>),
    ChannelConnectedLine(BaseEvent<ChannelConnectedLine>),
    ChannelCreated(BaseEvent<ChannelCreated>),
    ChannelDestroyed(BaseEvent<ChannelDestroyed>),
    ChannelDialplan(BaseEvent<ChannelDialplan>),
    ChannelDtmfReceived(BaseEvent<ChannelDtmfReceived>),
    ChannelEnteredBridge(BaseEvent<ChannelEnteredBridge>),
    ChannelHangupRequest(BaseEvent<ChannelHangupRequest>),
    ChannelHold(BaseEvent<ChannelHold>),
    ChannelLeftBridge(BaseEvent<ChannelLeftBridge>),
    ChannelStateChange(BaseEvent<ChannelStateChange>),
    ChannelTalkingFinished(BaseEvent<ChannelTalkingFinished>),
    ChannelTalkingStarted(BaseEvent<ChannelTalkingStarted>),
    ChannelToneDetected(BaseEvent<ChannelToneDetected>),
    ChannelUnhold(BaseEvent<ChannelUnhold>),
    #[serde(rename = "ChannelUserevent")]
    ChannelUserEvent(BaseEvent<ChannelUserEvent>),
    #[serde(rename = "ChannelVarset")]
    ChannelVarSet(BaseEvent<ChannelVarSet>),
    ContactInfo(BaseEvent<ContactInfo>),
    ContactStatusChange(BaseEvent<ContactStatusChange>),
    DeviceStateChanged(BaseEvent<DeviceStateChanged>),
    Dial(BaseEvent<Dial>),
    EndpointStateChange(BaseEvent<EndpointStateChange>),
    MissingParams(BaseEvent<MissingParams>),
    Peer(BaseEvent<Peer>),
    PeerStatusChange(BaseEvent<PeerStatusChange>),
    PlaybackContinuing(BaseEvent<PlaybackContinuing>),
    PlaybackFinished(BaseEvent<PlaybackFinished>),
    PlaybackStarted(BaseEvent<PlaybackStarted>),
    RecordingFailed(BaseEvent<RecordingFailed>),
    RecordingFinished(BaseEvent<RecordingFinished>),
    RecordingStarted(BaseEvent<RecordingStarted>),
    StasisEnd(BaseEvent<StasisEnd>),
    StasisStart(BaseEvent<StasisStart>),
    TextMessageReceived(BaseEvent<TextMessageReceived>),

    #[serde(untagged)]
    Unknown(serde_json::Value),
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Event::ApplicationMoveFailed(_) => write!(f, "ApplicationMoveFailed"),
            Event::ApplicationReplaced(_) => write!(f, "ApplicationReplaced"),
            Event::BridgeAttendedTransfer(_) => write!(f, "BridgeAttendedTransfer"),
            Event::BridgeBlindTransfer(_) => write!(f, "BridgeBlindTransfer"),
            Event::BridgeCreated(_) => write!(f, "BridgeCreated"),
            Event::BridgeDestroyed(_) => write!(f, "BridgeDestroyed"),
            Event::BridgeMerged(_) => write!(f, "BridgeMerged"),
            Event::BridgeVideoSourceChanged(_) => write!(f, "BridgeVideoSourceChanged"),
            Event::ChannelCallerId(_) => write!(f, "ChannelCallerId"),
            Event::ChannelConnectedLine(_) => write!(f, "ChannelConnectedLine"),
            Event::ChannelCreated(_) => write!(f, "ChannelCreated"),
            Event::ChannelDestroyed(_) => write!(f, "ChannelDestroyed"),
            Event::ChannelDialplan(_) => write!(f, "ChannelDialplan"),
            Event::ChannelDtmfReceived(_) => write!(f, "ChannelDtmfReceived"),
            Event::ChannelEnteredBridge(_) => write!(f, "ChannelEnteredBridge"),
            Event::ChannelHangupRequest(_) => write!(f, "ChannelHangupRequest"),
            Event::ChannelHold(_) => write!(f, "ChannelHold"),
            Event::ChannelLeftBridge(_) => write!(f, "ChannelLeftBridge"),
            Event::ChannelStateChange(_) => write!(f, "ChannelStateChange"),
            Event::ChannelTalkingFinished(_) => write!(f, "ChannelTalkingFinished"),
            Event::ChannelTalkingStarted(_) => write!(f, "ChannelTalkingStarted"),
            Event::ChannelToneDetected(_) => write!(f, "ChannelToneDetected"),
            Event::ChannelUnhold(_) => write!(f, "ChannelUnhold"),
            Event::ChannelUserEvent(_) => write!(f, "ChannelUserEvent"),
            Event::ChannelVarSet(_) => write!(f, "ChannelVarSet"),
            Event::ContactInfo(_) => write!(f, "ContactInfo"),
            Event::ContactStatusChange(_) => write!(f, "ContactStatusChange"),
            Event::DeviceStateChanged(_) => write!(f, "DeviceStateChanged"),
            Event::Dial(_) => write!(f, "Dial"),
            Event::EndpointStateChange(_) => write!(f, "EndpointStateChange"),
            Event::MissingParams(_) => write!(f, "MissingParams"),
            Event::Peer(_) => write!(f, "Peer"),
            Event::PeerStatusChange(_) => write!(f, "PeerStatusChange"),
            Event::PlaybackContinuing(_) => write!(f, "PlaybackContinuing"),
            Event::PlaybackFinished(_) => write!(f, "PlaybackFinished"),
            Event::PlaybackStarted(_) => write!(f, "PlaybackStarted"),
            Event::RecordingFailed(_) => write!(f, "RecordingFailed"),
            Event::RecordingFinished(_) => write!(f, "RecordingFinished"),
            Event::RecordingStarted(_) => write!(f, "RecordingStarted"),
            Event::StasisEnd(_) => write!(f, "StasisEnd"),
            Event::StasisStart(_) => write!(f, "StasisStart"),
            Event::TextMessageReceived(_) => write!(f, "TextMessageReceived"),
            Event::Unknown(_) => write!(f, "Unknown"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_unknown_event() {
        let e = "{\n  \"type\": \"NotKnown\",\n  \"timestamp\": \"2021-01-07T22:12:29.571+0100\",\n  \"channel\": {\n    \"id\": \"1610053949.0\",\n    \"name\": \"SIP/1004-00000000\",\n    \"state\": \"Up\",\n    \"caller\": {\n      \"name\": \"Adam\",\n      \"number\": \"1004\"\n    },\n    \"connected\": {\n      \"name\": \"\",\n      \"number\": \"\"\n    },\n    \"accountcode\": \"\",\n    \"dialplan\": {\n      \"context\": \"internal\",\n      \"exten\": \"158\",\n      \"priority\": 10,\n      \"app_name\": \"Stasis\",\n      \"app_data\": \"va-voicegw-rs,freight-cs-voice,en-US\"\n    },\n    \"creationtime\": \"2021-01-07T22:12:29.369+0100\",\n    \"language\": \"en\"\n  },\n  \"asterisk_id\": \"00:50:56:98:74:21\",\n  \"application\": \"va-voicegw-rs\"\n}";
        let ari_event: Event = serde_json::from_str(e).unwrap();
        assert!(matches!(ari_event, Event::Unknown(_)));
    }

    #[test]
    fn test_parse_stasis_start() {
        let e = "{\n  \"type\": \"StasisStart\",\n  \"timestamp\": \"2020-11-22T20:17:06.150+0000\",\n  \"args\": [\n    \"its-va-demo-app\",\n    \"en-US\"\n  ],\n  \"channel\": {\n    \"id\": \"1606076223.3\",\n    \"name\": \"PJSIP/6001-00000003\",\n    \"state\": \"Up\",\n    \"caller\": {\n      \"name\": \"\",\n      \"number\": \"6001\"\n    },\n    \"connected\": {\n      \"name\": \"\",\n      \"number\": \"\"\n    },\n    \"accountcode\": \"\",\n    \"dialplan\": {\n      \"context\": \"from-internal\",\n      \"exten\": \"101\",\n      \"priority\": 6,\n      \"app_name\": \"Stasis\",\n      \"app_data\": \"va-voicegw,its-va-demo-app,en-US\"\n    },\n    \"creationtime\": \"2020-11-22T20:17:03.741+0000\",\n    \"language\": \"en\"\n  },\n  \"asterisk_id\": \"00:15:5d:01:65:04\",\n  \"application\": \"va-voicegw\"\n}";
        let ari_event: Event = serde_json::from_str(e).unwrap();
        assert!(matches!(ari_event, Event::StasisStart(_)));
    }

    // test the Timezone conversion
    #[test]
    fn test_parse_stasis_start_with_timezone() {
        let e = "{\n  \"type\": \"StasisStart\",\n  \"timestamp\": \"2021-01-07T21:12:57.268+0100\",\n  \"args\": [\n    \"freight-cs-voice\",\n    \"en-US\"\n  ],\n  \"channel\": {\n    \"id\": \"1610050377.0\",\n    \"name\": \"SIP/1004-00000000\",\n    \"state\": \"Ring\",\n    \"caller\": {\n      \"name\": \"Adam\",\n      \"number\": \"1004\"\n    },\n    \"connected\": {\n      \"name\": \"\",\n      \"number\": \"\"\n    },\n    \"accountcode\": \"\",\n    \"dialplan\": {\n      \"context\": \"internal\",\n      \"exten\": \"158\",\n      \"priority\": 10,\n      \"app_name\": \"Stasis\",\n      \"app_data\": \"va-voicegw-rs,freight-cs-voice,en-US\"\n    },\n    \"creationtime\": \"2021-01-07T21:12:57.267+0100\",\n    \"language\": \"en\"\n  },\n  \"asterisk_id\": \"00:50:56:98:74:21\",\n  \"application\": \"va-voicegw-rs\"\n}";
        let ari_event: Event = serde_json::from_str(e).unwrap();
        assert!(matches!(ari_event, Event::StasisStart(_)));

        if let Event::StasisStart(base_event) = ari_event {
            assert_eq!(
                base_event.timestamp.to_string(),
                "2021-01-07 20:12:57.268 UTC"
            );
        }
    }

    // cargo test --package asterisk-ari-client -- --show-output test_parse_channel_state_change
    #[test]
    fn test_parse_channel_state_change() {
        let e = "{\n  \"type\": \"ChannelStateChange\",\n  \"timestamp\": \"2021-01-07T22:12:29.571+0100\",\n  \"channel\": {\n    \"id\": \"1610053949.0\",\n    \"name\": \"SIP/1004-00000000\",\n    \"state\": \"Up\",\n    \"caller\": {\n      \"name\": \"Adam\",\n      \"number\": \"1004\"\n    },\n    \"connected\": {\n      \"name\": \"\",\n      \"number\": \"\"\n    },\n    \"accountcode\": \"\",\n    \"dialplan\": {\n      \"context\": \"internal\",\n      \"exten\": \"158\",\n      \"priority\": 10,\n      \"app_name\": \"Stasis\",\n      \"app_data\": \"va-voicegw-rs,freight-cs-voice,en-US\"\n    },\n    \"creationtime\": \"2021-01-07T22:12:29.369+0100\",\n    \"language\": \"en\"\n  },\n  \"asterisk_id\": \"00:50:56:98:74:21\",\n  \"application\": \"va-voicegw-rs\"\n}";
        let ari_event: Event = serde_json::from_str(e).unwrap();
        println!("{:#?}", ari_event);
    }
}
