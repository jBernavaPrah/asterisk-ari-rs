use crate::apis::concat_option_str;
use crate::apis::params::{Direction, IfExists, TerminateOn};
use derive_new::new;
use derive_setters::Setters;
use serde::Serialize;

#[derive(Clone, Debug, Default, PartialEq, Serialize, new, Setters)]
#[setters(prefix = "with_")]
#[setters(into, strip_option)]
#[serde(rename_all = "camelCase")]
pub struct OriginateRequest {
    /// Endpoint to call.
    #[setters(skip)]
    #[new(into)]
    endpoint: String,

    /// The extension to dial after the endpoint answers. Mutually exclusive with 'app'.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    #[new(default)]
    extension: Option<Extension>,

    /// The application that is subscribed to the originated channel.
    ///
    /// When the channel is answered, it will be passed to this Stasis application.
    /// Mutually exclusive with  'extension'.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    #[new(default)]
    app: Option<App>,

    /// CallerID to use when dialing the endpoint or extension.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    caller_id: Option<String>,

    /// The timeout (in seconds) before giving up dialing, or -1 for no timeout.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(value = "Some(30)")]
    timeout: Option<i32>,

    /// The unique id to assign the channel on creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    channel_id: Option<String>,

    /// The unique id to assign the second channel when using local channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    other_channel_id: Option<String>,

    /// The unique id of the channel which is originating this one.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    originator: Option<String>,

    /// The format name capability list to use if originator is not specified.
    /// Ex. "ulaw,slin16".
    ///
    /// Format names can be found with "core show codecs".
    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    #[serde(serialize_with = "concat_option_str")]
    formats: Option<Vec<String>>,

    /// The "variables" key in the body object holds variable key/value pairs to set on the channel on creation.
    ///
    /// Other keys in the body object are interpreted as query parameters.
    /// Ex. { "endpoint": "SIP/Alice", "variables": { "CALLERID(name)": "Alice" } }
    #[serde(skip_serializing)]
    #[new(default)]
    pub(crate) variables: Option<serde_json::Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, new, Setters)]
#[setters(prefix = "with_")]
#[setters(into, strip_option)]
pub struct Extension {
    /// The extension to dial after the endpoint answers.
    #[setters(skip)]
    extension: String,

    /// The context to dial after the endpoint answers.
    /// If omitted, uses 'default'.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    context: Option<String>,

    /// The priority to dial after the endpoint answers.
    /// If omitted, uses 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    priority: Option<String>,

    /// The label to dial after the endpoint answers.
    /// Will supersede 'priority' if provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    label: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, new, Setters)]
#[setters(prefix = "with_")]
#[setters(into, strip_option)]
pub struct App {
    /// Stasis Application to place channel into
    #[serde(rename = "app")]
    #[setters(skip)]
    #[new(into)]
    name: String,
    /// The application arguments to pass to the Stasis application provided by 'app'.
    #[serde(rename = "appArgs", skip_serializing_if = "Option::is_none")]
    #[new(default)]
    args: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, new, Setters)]
#[setters(prefix = "with_")]
#[setters(into, strip_option)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    /// Endpoint for channel communication
    #[setters(skip)]
    #[new(into)]
    endpoint: String,

    /// Stasis Application to place channel into
    #[serde(flatten)]
    #[setters(skip)]
    app: App,

    /// The unique id to assign the channel on creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    channel_id: Option<String>,

    /// The unique id to assign the second channel when using local channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    other_channel_id: Option<String>,

    /// Unique ID of the calling channel
    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    originator: Option<String>,

    /// The format name capability list to use if originator is not specified.
    /// Ex. "ulaw,slin16".
    ///
    /// Format names can be found with "core show codecs".
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "concat_option_str")]
    #[new(default)]
    formats: Option<Vec<String>>,

    /// The "variables" key in the body object holds variable key/value pairs to set on the channel on creation.
    ///
    /// Other keys in the body object are interpreted as query parameters.
    /// Ex. { "endpoint": "SIP/Alice", "variables": { "CALLERID(name)": "Alice" } }
    #[serde(skip_serializing)]
    #[new(default)]
    pub(crate) variables: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, new, Setters)]
#[setters(prefix = "with_")]
#[setters(into, strip_option)]
#[serde(rename_all = "camelCase")]
pub struct OriginateWithIdRequest {
    /// The unique id to assign the channel on creation.
    #[serde(skip_serializing)]
    #[setters(skip)]
    #[new(into)]
    pub(crate) channel_id: String,

    /// Endpoint to call.
    #[setters(skip)]
    #[new(into)]
    endpoint: String,

    /// The extension to dial after the endpoint answers. Mutually exclusive with 'app'.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    #[new(default)]
    extension: Option<Extension>,

    /// The application that is subscribed to the originated channel.
    ///
    /// When the channel is answered, it will be passed to this Stasis application.
    /// Mutually exclusive with  'extension'.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    #[new(default)]
    app: Option<App>,

    /// CallerID to use when dialing the endpoint or extension.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    caller_id: Option<String>,

    /// The timeout (in seconds) before giving up dialing, or -1 for no timeout.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(value = "Some(30)")]
    timeout: Option<i32>,

    /// The unique id to assign the second channel when using local channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    other_channel_id: Option<String>,

    /// The unique id of the channel which is originating this one.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    originator: Option<String>,

    /// The format name capability list to use if originator is not specified.
    /// Ex. "ulaw,slin16".
    ///
    /// Format names can be found with "core show codecs".
    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    #[serde(serialize_with = "concat_option_str")]
    formats: Option<Vec<String>>,

    /// The "variables" key in the body object holds variable key/value pairs to set on the channel on creation.
    ///
    /// Other keys in the body object are interpreted as query parameters.
    /// Ex. { "endpoint": "SIP/Alice", "variables": { "CALLERID(name)": "Alice" } }
    #[serde(skip_serializing)]
    #[new(default)]
    pub(crate) variables: Option<serde_json::Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Default)]
pub enum CancelReason {
    #[serde(rename = "normal")]
    #[default]
    Normal,
    #[serde(rename = "busy")]
    Busy,
    #[serde(rename = "congestion")]
    Congestion,
    #[serde(rename = "no_answer")]
    NoAnswer,
    #[serde(rename = "timeout")]
    Timeout,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "unallocated")]
    Unallocated,
    #[serde(rename = "normal_unspecified")]
    NormalUnspecified,
    #[serde(rename = "number_incomplete")]
    NumberIncomplete,
    #[serde(rename = "codec_mismatch")]
    CodecMismatch,
    #[serde(rename = "interworking")]
    Interworking,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "answered_elsewhere")]
    AnsweredElsewhere,
}

#[derive(Clone, Debug, Serialize, new, Setters)]
#[setters(prefix = "with_")]
#[setters(into, strip_option)]
pub struct DeleteRequest {
    /// Channel's id
    #[serde(skip_serializing)]
    #[setters(skip)]
    #[new(into)]
    pub(crate) channel_id: String,

    /// The reason code for hanging up the channel for detail use.
    ///
    /// Mutually exclusive with 'reason'.
    /// See detail hangup codes at here. <https://docs.asterisk.org/Configuration/Miscellaneous/Hangup-Cause-Mappings/>
    #[serde(rename = "reason_code", skip_serializing_if = "Option::is_none")]
    #[new(default)]
    reason_code: Option<String>,

    /// Reason for hanging up the channel for simple use.
    ///
    /// Mutually exclusive with 'reason_code'.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    #[new(default)]
    reason: Option<CancelReason>,
}

#[derive(Clone, Debug, Serialize, new, Setters)]
#[setters(prefix = "with_")]
#[setters(into, strip_option)]
pub struct ContinueRequest {
    /// Channel's id
    #[serde(skip_serializing)]
    #[setters(skip)]
    #[new(into)]
    pub(crate) channel_id: String,

    /// The extension to continue to.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    #[new(default)]
    extension: Option<Extension>,
}

#[derive(Clone, Debug, Serialize, new, Setters)]
#[setters(prefix = "with_")]
#[setters(into, strip_option)]
pub struct MoveRequest {
    /// Channel's id
    #[serde(skip_serializing)]
    #[setters(skip)]
    #[new(into)]
    pub(crate) channel_id: String,

    /// The channel will be passed to this Stasis application.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    #[new(default)]
    app: Option<App>,
}

#[derive(Clone, Debug, Serialize, new, Setters)]
#[setters(prefix = "with_")]
#[setters(into, strip_option)]
pub struct DtmfRequest {
    /// Channel's id
    #[serde(skip_serializing)]
    #[setters(skip)]
    #[new(into)]
    pub(crate) channel_id: String,

    /// DTMF To send.
    #[setters(skip)]
    #[new(into)]
    dtmf: String,

    /// Amount of time to wait before DTMF digits (specified in milliseconds) start.
    #[serde(rename = "before", skip_serializing_if = "Option::is_none")]
    #[new(default)]
    before: Option<u32>,

    /// Amount of time in between DTMF digits (specified in milliseconds).
    #[serde(rename = "between", skip_serializing_if = "Option::is_none")]
    #[new(value = "Some(100)")]
    between: Option<u32>,

    /// Length of each DTMF digit (specified in milliseconds).
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    #[new(value = "Some(100)")]
    duration: Option<u32>,

    /// Amount of time to wait after DTMF digits (specified in milliseconds) end.
    #[serde(rename = "after", skip_serializing_if = "Option::is_none")]
    #[new(default)]
    after: Option<u32>,
}

#[derive(Clone, Debug, Serialize, new, Setters)]
#[setters(prefix = "with_")]
#[setters(into, strip_option)]
pub struct MohRequest {
    /// Channel's id.
    #[serde(skip_serializing)]
    #[setters(skip)]
    #[new(into)]
    pub(crate) channel_id: String,

    /// Music on hold class to use.
    #[serde(rename = "mohClass", skip_serializing_if = "Option::is_none")]
    #[new(default)]
    moh_class: Option<String>,
}

#[derive(Clone, Debug, Serialize, new, Setters)]
#[setters(prefix = "with_")]
#[setters(into, strip_option)]
pub struct PlayRequest {
    /// Channel's id.
    #[serde(skip_serializing)]
    #[setters(skip)]
    #[new(into)]
    pub(crate) channel_id: String,

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
pub struct PlayWithPlaybackIdRequest {
    /// Channel's id.
    #[serde(skip_serializing)]
    #[setters(skip)]
    #[new(into)]
    pub(crate) channel_id: String,

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
    /// Channel's id.
    #[serde(skip_serializing)]
    #[setters(skip)]
    #[new(into)]
    pub(crate) channel_id: String,

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

#[derive(Clone, Debug, Serialize, new, Setters)]
#[setters(prefix = "with_")]
#[setters(into, strip_option)]
pub struct SnoopRequest {
    /// Channel's id
    #[serde(skip_serializing)]
    #[setters(skip)]
    #[new(into)]
    pub(crate) channel_id: String,

    /// The channel will be passed to this Stasis application.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    #[new(default)]
    app: Option<App>,

    /// Direction of audio to spy on.
    #[serde(rename = "spy", skip_serializing_if = "Option::is_none")]
    #[new(default)]
    spy: Option<Direction>,

    /// Direction of audio to whisper into.
    #[serde(rename = "whisper", skip_serializing_if = "Option::is_none")]
    #[new(default)]
    whisper: Option<Direction>,
}

#[derive(Clone, Debug, Serialize, new, Setters)]
#[setters(prefix = "with_")]
#[setters(into, strip_option)]
pub struct SnoopWithIdRequest {
    /// Channel's id
    #[serde(skip_serializing)]
    #[setters(skip)]
    #[new(into)]
    pub(crate) channel_id: String,

    /// Channel's id
    #[serde(skip_serializing)]
    #[setters(skip)]
    #[new(into)]
    pub(crate) snoop_id: String,

    /// The channel will be passed to this Stasis application.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    #[new(default)]
    app: Option<App>,

    /// Direction of audio to spy on.
    #[serde(rename = "spy", skip_serializing_if = "Option::is_none")]
    #[new(default)]
    spy: Option<Direction>,

    /// Direction of audio to whisper into.
    #[serde(rename = "whisper", skip_serializing_if = "Option::is_none")]
    #[new(default)]
    whisper: Option<Direction>,
}

#[derive(Clone, Debug, Serialize, new, Setters)]
#[setters(prefix = "with_")]
#[setters(into, strip_option)]
pub struct DialRequest {
    /// Channel's id
    #[serde(skip_serializing)]
    #[setters(skip)]
    #[new(into)]
    pub(crate) channel_id: String,

    /// Channel ID of caller.
    #[serde(rename = "caller", skip_serializing_if = "Option::is_none")]
    #[new(default)]
    caller: Option<String>,

    /// Dial timeout.
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    #[new(default)]
    timeout: Option<u32>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, new, Setters)]
#[setters(prefix = "with_")]
#[setters(into, strip_option)]
#[serde(rename_all = "camelCase")]
pub struct ExternalMediaRequest {
    /// Stasis Application to place channel into.
    #[setters(skip)]
    #[new(into)]
    app: String,

    /// Hostname/ip:port of external host.
    #[setters(skip)]
    #[new(into)]
    external_host: String,

    /// Format to encode audio in.
    #[setters(skip)]
    #[new(into)]
    format: String,

    /// The unique id to assign the channel on creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    channel_id: Option<String>,

    /// The "variables" key in the body object holds variable key/value pairs to set on the channel on creation.
    ///
    /// Other keys in the body object are interpreted as query parameters.
    /// Ex. { "endpoint": "SIP/Alice", "variables": { "CALLERID(name)": "Alice" } }
    #[serde(skip_serializing)]
    #[new(default)]
    pub(crate) variables: Option<serde_json::Value>,

    /// Payload encapsulation protocol.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    encapsulation: Option<Encapsulation>,

    /// Transport protocol.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    transport: Option<Transport>,

    /// Connection type (client/server).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    connection_type: Option<ConnectionType>,

    /// External media direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    direction: Option<Direction>,

    /// An arbitrary data field.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    data: Option<String>,
}

#[derive(Clone, Debug, Serialize, Default, PartialEq)]
pub enum ConnectionType {
    #[default]
    #[serde(rename = "client")]
    Client,

    #[serde(rename = "server")]
    Server,
}

#[derive(Clone, Debug, Serialize, Default, PartialEq)]
pub enum Transport {
    #[serde(rename = "udp")]
    #[default]
    Udp,

    #[serde(rename = "tcp")]
    Tcp,
}

#[derive(Clone, Debug, Serialize, Default, PartialEq)]
pub enum Encapsulation {
    #[serde(rename = "rtp")]
    #[default]
    Rtp,

    #[serde(rename = "audiosocket")]
    AudioSocket,
}
