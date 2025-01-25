use crate::apis::bridges::models;
use crate::apis::params::{IfExists, TerminateOn};
use derive_new::new;
use derive_setters::Setters;
use serde::Serialize;

#[derive(Clone, Default, Debug, PartialEq, Serialize, new, Setters)]
#[setters(prefix = "with_")]
#[setters(into, strip_option)]
pub struct CreateRequest {
    /// Comma separated list of bridge type attributes (mixing, holding, dtmf_events, proxy_media, video_sfu, video_single, sdp_label).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    #[setters(rename = "with_type")]
    #[new(default)]
    r#type: Option<models::BridgeType>,

    /// Unique ID to give to the bridge being created.
    #[serde(rename = "bridgeId", skip_serializing_if = "Option::is_none")]
    #[new(default)]
    bridge_id: Option<String>,

    /// Name to give to the bridge being created.
    #[serde(rename = "bridgeId", skip_serializing_if = "Option::is_none")]
    #[new(default)]
    name: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, new, Setters)]
#[setters(prefix = "with_")]
#[setters(into, strip_option)]
pub struct CreateWithIdRequest {
    /// Unique ID to give to the bridge being created.
    #[serde(skip_serializing)]
    #[setters(skip)]
    #[new(into)]
    pub(crate) bridge_id: String,

    /// Comma separated list of bridge type attributes (mixing, holding, dtmf_events, proxy_media, video_sfu, video_single, sdp_label).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    #[setters(rename = "with_type")]
    #[new(default)]
    r#type: Option<models::BridgeType>,

    /// Name to give to the bridge being created.
    #[serde(rename = "bridgeId", skip_serializing_if = "Option::is_none")]
    #[new(default)]
    name: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, new, Setters)]
#[setters(prefix = "with_")]
#[setters(into, strip_option)]
pub struct AddChannelRequest {
    /// Unique ID to give to the bridge being created.
    #[serde(skip_serializing)]
    #[setters(skip)]
    #[new(into)]
    pub(crate) bridge_id: String,

    /// Ids of channels to add to bridge.
    #[setters(skip)]
    #[new(into)]
    pub(crate) channel: String,

    /// Channel's role in the bridge
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    #[new(default)]
    role: Option<String>,

    /// Absorb DTMF coming from this channel, preventing it to pass through to the bridge
    #[serde(rename = "absorbDTMF", skip_serializing_if = "Option::is_none")]
    #[new(default)]
    absorb_dtmf: Option<bool>,

    /// Mute audio from this channel, preventing it to pass through to the bridge.
    #[serde(rename = "mute", skip_serializing_if = "Option::is_none")]
    #[new(default)]
    mute: Option<bool>,

    /// Do not present the identity of the newly connected channel to other bridge members.
    #[serde(
        rename = "inhibitConnectedLineUpdates",
        skip_serializing_if = "Option::is_none"
    )]
    #[new(default)]
    inhibit_connected_line_updates: Option<bool>,
}

#[derive(Clone, Debug, Serialize, new, Setters)]
#[setters(prefix = "with_")]
#[setters(into, strip_option)]
pub struct MohRequest {
    /// Bridge's id
    #[serde(skip_serializing)]
    #[setters(skip)]
    #[new(into)]
    pub(crate) bridge_id: String,

    /// Music on hold class to use.
    #[serde(rename = "mohClass", skip_serializing_if = "Option::is_none")]
    #[new(default)]
    moh_class: Option<String>,
}

#[derive(Clone, Debug, Serialize, new, Setters)]
#[setters(prefix = "with_")]
#[setters(into, strip_option)]
pub struct PlayRequest {
    /// Bridge's id.
    #[serde(skip_serializing)]
    #[setters(skip)]
    #[new(into)]
    pub(crate) bridge_id: String,

    /// Media URIs to play.
    #[setters(skip)]
    #[new(into)]
    media: String,

    /// For sounds, selects language for sound.
    #[serde(rename = "lang", skip_serializing_if = "Option::is_none")]
    #[new(default)]
    lang: Option<String>,

    /// Number of milliseconds to skip before playing. Only applies to the first URI if multiple media URIs are specified.
    #[serde(rename = "offsetms", skip_serializing_if = "Option::is_none")]
    #[new(default)]
    offset_ms: Option<u32>,

    /// Number of milliseconds to skip for forward/reverse operations.
    #[serde(rename = "skipms", skip_serializing_if = "Option::is_none")]
    #[new(default)]
    skip_ms: Option<u32>,
}

#[derive(Clone, Debug, Serialize, new, Setters)]
#[setters(prefix = "with_")]
#[setters(into, strip_option)]
pub struct PlayWithIdRequest {
    /// Bridge's id.
    #[serde(skip_serializing)]
    #[setters(skip)]
    #[new(into)]
    pub(crate) bridge_id: String,

    /// Playback ID.
    #[serde(skip_serializing)]
    #[setters(skip)]
    #[new(into)]
    pub(crate) playback_id: String,

    /// Media URIs to play.
    #[setters(skip)]
    #[new(into)]
    pub(crate) media: String,

    /// For sounds, selects language for sound.
    #[serde(rename = "lang", skip_serializing_if = "Option::is_none")]
    #[new(default)]
    lang: Option<String>,

    /// Number of milliseconds to skip before playing. Only applies to the first URI if multiple media URIs are specified.
    #[serde(rename = "offsetms", skip_serializing_if = "Option::is_none")]
    #[new(default)]
    offset_ms: Option<u32>,

    /// Number of milliseconds to skip for forward/reverse operations.
    #[serde(rename = "skipms", skip_serializing_if = "Option::is_none")]
    #[new(default)]
    skip_ms: Option<u32>,
}

#[derive(Clone, Debug, Serialize, new, Setters)]
#[setters(prefix = "with_")]
#[setters(into, strip_option)]
pub struct RecordRequest {
    /// Bridge's id.
    #[serde(skip_serializing)]
    #[setters(skip)]
    #[new(into)]
    pub(crate) bridge_id: String,

    /// Recording's filename
    #[setters(skip)]
    #[new(into)]
    pub(crate) name: String,

    /// Format to encode audio in
    #[setters(skip)]
    #[new(into)]
    pub(crate) format: String,

    /// Maximum duration of the recording, in seconds. 0 for no limit.
    #[serde(rename = "maxDurationSeconds", skip_serializing_if = "Option::is_none")]
    #[new(default)]
    max_duration_seconds: Option<u32>,

    /// Maximum duration of silence, in seconds. 0 for no limit.
    #[serde(rename = "maxSilenceSeconds", skip_serializing_if = "Option::is_none")]
    #[new(default)]
    max_silence_seconds: Option<u32>,

    /// Action to take if a recording with the same name already exists.
    #[serde(rename = "ifExists", skip_serializing_if = "Option::is_none")]
    #[new(default)]
    if_exists: Option<IfExists>,

    /// Play beep when recording begins.
    #[serde(rename = "beep", skip_serializing_if = "Option::is_none")]
    #[new(default)]
    beep: Option<bool>,

    /// DTMF input to terminate recording.
    #[serde(rename = "terminateOn", skip_serializing_if = "Option::is_none")]
    #[new(default)]
    terminate_on: Option<TerminateOn>,
}
