use serde::Deserialize;

/// LiveRecording:
///
/// A recording that is in progress
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct LiveRecording {
    /// Base name for the recording
    #[serde(rename = "name")]
    pub name: String,
    /// Recording format (wav, gsm, etc.)
    #[serde(rename = "format")]
    pub format: String,
    /// URI for the channel or bridge being recorded
    #[serde(rename = "target_uri")]
    pub target_uri: String,
    #[serde(rename = "state")]
    pub state: LiveRecordingState,
    /// Duration in seconds of the recording.
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,
    /// Duration of talking, in seconds, detected in the recording.
    /// This is only available if the recording was initiated with a non-zero maxSilenceSeconds.
    #[serde(rename = "talking_duration", skip_serializing_if = "Option::is_none")]
    pub talking_duration: Option<u32>,
    /// Duration of silence, in seconds, detected in the recording.
    /// This is only available if the recording was initiated with a non-zero maxSilenceSeconds.,
    #[serde(rename = "silence_duration", skip_serializing_if = "Option::is_none")]
    pub silence_duration: Option<u32>,
    /// Cause for recording failure if failed
    #[serde(rename = "cause", skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Default)]
pub enum LiveRecordingState {
    #[serde(rename = "recording")]
    #[default]
    Recording,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "done")]
    Done,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "queued")]
    Queued,
}

/// StoredRecording:
///
/// A past recording that may be played back.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct StoredRecording {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "format")]
    pub format: String,
}
