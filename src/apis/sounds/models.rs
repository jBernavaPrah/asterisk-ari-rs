use serde::Deserialize;

/// Sound : A media file that may be played back.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct Sound {
    /// Sound's identifier.
    #[serde(rename = "id")]
    pub id: String,
    /// Text description of the sound, usually the words spoken.
    #[serde(rename = "text")]
    pub text: Option<String>,
    /// The formats and languages in which this sound is available.
    #[serde(rename = "formats")]
    pub formats: Vec<FormatLanguage>,
}

/// FormatLanguage : Identifies the format and language of a sound file
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct FormatLanguage {
    #[serde(rename = "language")]
    pub language: String,
    #[serde(rename = "format")]
    pub format: String,
}
