use serde::Deserialize;

/// Playback : Object representing the playback of media to a channel
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct Playback {
    /// ID for this playback operation
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The URI for the media currently being played back.
    #[serde(rename = "media_uri")]
    pub media_uri: Option<String>,
    /// If a list of URIs is being played, the next media URI to be played back.
    #[serde(rename = "next_media_uri")]
    pub next_media_uri: Option<String>,
    /// URI for the channel or bridge to play the media on
    #[serde(rename = "target_uri")]
    pub target_uri: Option<String>,
    /// For media types that support multiple languages, the language requested for playback.
    #[serde(rename = "language")]
    pub language: Option<String>,
    /// Current state of the playback operation.
    #[serde(rename = "state")]
    pub state: PlaybackState,
}
#[derive(Clone, Debug, PartialEq, Deserialize, Default)]
pub enum PlaybackState {
    #[serde(rename = "queued")]
    #[default]
    Queued,
    #[serde(rename = "playing")]
    Playing,
    #[serde(rename = "continuing")]
    Continuing,
    #[serde(rename = "done")]
    Done,
    #[serde(rename = "failed")]
    Failed,
}
