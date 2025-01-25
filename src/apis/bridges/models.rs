use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Bridge:
///
/// The merging of media from one or more channels.
/// Everyone on the bridge receives the same audio.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct Bridge {
    /// Unique identifier for this bridge
    #[serde(rename = "id")]
    pub id: String,
    /// Name of the current bridging technology
    #[serde(rename = "technology")]
    pub technology: String,
    /// Type of bridge technology
    #[serde(rename = "bridge_type")]
    pub bridge_type: BridgeType,
    /// Bridging class
    #[serde(rename = "bridge_class")]
    pub bridge_class: String,
    /// Entity that created the bridge
    #[serde(rename = "creator")]
    pub creator: String,
    /// Name the creator gave the bridge
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "channels")]
    pub channels: Vec<String>,
    /// The video mode the bridge is using. One of 'none', 'talker', 'sfu', or 'single'.
    #[serde(rename = "video_mode")]
    pub video_mode: Option<VideoMode>,
    /// The ID of the channel that is the source of video in this bridge, if one exists.
    #[serde(rename = "video_source_id")]
    pub video_source_id: Option<String>,
    #[serde(rename = "creationtime")]
    pub creation_time: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub enum BridgeType {
    #[serde(rename = "mixing")]
    #[default]
    Mixing,
    #[serde(rename = "holding")]
    Holding,
    #[serde(rename = "dtmf_events")]
    DtmfEvents,
    #[serde(rename = "proxy_media")]
    ProxyMedia,
    #[serde(rename = "video_sfu")]
    VideoSFU,
    #[serde(rename = "video_single")]
    VideoSingle,
    #[serde(rename = "sdp_label")]
    SDPLabel,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Default)]
pub enum VideoMode {
    #[serde(rename = "none")]
    #[default]
    None,
    #[serde(rename = "talker")]
    Talker,
    #[serde(rename = "sfu")]
    Sfu,
    #[serde(rename = "single")]
    Single,
}
