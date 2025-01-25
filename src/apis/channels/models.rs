use serde::Deserialize;

use chrono::{DateTime, Utc};

/// Channel : A specific communication connection between Asterisk and an Endpoint.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct Channel {
    /// Unique identifier of the channel.  This is the same as the Uniqueid field in AMI.
    #[serde(rename = "id")]
    pub id: String,
    /// Protocol id from underlying channel driver (i.e. Call-ID for chan_pjsip; will be empty if not applicable or not implemented by driver).
    #[serde(rename = "protocol_id")]
    pub protocol_id: Option<String>,
    /// Name of the channel (i.e. SIP/foo-0000a7e3)
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "state")]
    pub state: ChannelState,
    #[serde(rename = "caller")]
    pub caller: CallerId,
    #[serde(rename = "connected")]
    pub connected: CallerId,
    #[serde(rename = "accountcode")]
    pub account_code: String,
    #[serde(rename = "dialplan")]
    pub dialplan: DialplanCep,
    #[serde(rename = "creationtime")]
    pub creation_time: DateTime<Utc>,
    /// The default spoken language
    #[serde(rename = "language")]
    pub language: String,
    /// Channel variables
    #[serde(rename = "channelvars")]
    pub channel_vars: Option<serde_json::Value>,
    /// The Caller ID RDNIS
    #[serde(rename = "caller_rdnis")]
    pub caller_rdnis: Option<String>,
    /// The Tenant ID for the channel
    #[serde(rename = "tenantid")]
    pub tenant_id: Option<String>,
}

/// CallerId : Caller identification
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct CallerId {
    pub name: String,
    pub number: String,
}

/// DialplanCep : Dialplan location (context/extension/priority)
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct DialplanCep {
    /// Context in the dialplan
    pub context: String,

    /// Extension in the dialplan
    #[serde(rename = "exten")]
    pub extension: String,

    /// Priority in the dialplan,
    pub priority: f32,

    #[serde(flatten)]
    pub app_name: App,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct App {
    /// Name of current dialplan application
    #[serde(rename = "app_name")]
    name: String,
    /// Parameter of current dialplan application
    #[serde(rename = "app_data")]
    data: Option<String>,
}

/// RTPStatistics : A statistics of RTP.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct RTPStatistics {
    /// Number of packets transmitted.,
    pub txcount: u64,
    /// Number of packets received.
    pub rxcount: u64,
    /// Jitter on transmitted packets.
    pub txjitter: Option<f64>,

    /// Jitter on received packets.
    pub rxjitter: Option<f64>,

    /// Maximum jitter on remote side.
    pub remote_maxjitter: Option<f64>,

    /// Minimum jitter on remote side.
    pub remote_minjitter: Option<f64>,

    /// Average jitter on remote side.
    pub remote_normdevjitter: Option<f64>,

    /// Standard deviation jitter on remote side.
    pub remote_stdevjitter: Option<f64>,

    /// Maximum jitter on local side.
    pub local_maxjitter: Option<f64>,

    /// Minimum jitter on local side.
    pub local_minjitter: Option<f64>,

    /// Average jitter on local side.
    pub local_normdevjitter: Option<f64>,

    /// Standard deviation jitter on local side.
    pub local_stdevjitter: Option<f64>,

    /// Number of transmitted packets lost.
    pub txploss: u64,

    /// Number of received packets lost.
    pub rxploss: u64,

    /// Maximum number of packets lost on remote side.
    pub remote_maxrxploss: Option<f64>,

    /// Minimum number of packets lost on remote side.
    pub remote_minrxploss: Option<f64>,

    /// Average number of packets lost on remote side.
    pub remote_normdevrxploss: Option<f64>,

    /// Standard deviation packets lost on remote side.
    pub remote_stdevrxploss: Option<f64>,

    /// Maximum number of packets lost on local side.
    pub local_maxrxploss: Option<f64>,

    /// Minimum number of packets lost on local side.
    pub local_minrxploss: Option<f64>,

    /// Average number of packets lost on local side.
    pub local_normdevrxploss: Option<f64>,

    /// Standard deviation packets lost on local side.
    pub local_stdevrxploss: Option<f64>,

    /// Total round trip time.
    pub rtt: Option<f64>,

    /// Maximum round trip time.
    pub maxrtt: Option<f64>,

    /// Minimum round trip time.
    pub minrtt: Option<f64>,

    /// Average round trip time.
    pub normdevrtt: Option<f64>,

    /// Standard deviation round trip time.
    pub stdevrtt: Option<f64>,

    /// Our SSRC.
    pub local_ssrc: u64,

    /// Their SSRC.
    pub remote_ssrc: u64,

    /// Number of octets transmitted.
    pub txoctetcount: u64,

    /// Number of octets received.
    pub rxoctetcount: u64,
    /// The Asterisk channel's unique ID that owns this instance.
    pub channel_uniqueid: String,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub enum ChannelState {
    Down,
    Rsrved,
    Reserved,
    OffHook,
    Dialing,
    Ring,
    Ringing,
    Up,
    Busy,
    #[serde(rename = "Dialing Offhook")]
    DialingOffhook,
    #[serde(rename = "Pre-ring")]
    PreRing,
    #[default]
    Unknown,
}
