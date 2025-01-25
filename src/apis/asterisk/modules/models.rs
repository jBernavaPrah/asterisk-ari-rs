use serde::Deserialize;

/// Module : Details of an Asterisk module
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct Module {
    /// The name of this module
    #[serde(rename = "name")]
    pub name: String,
    /// The description of this module
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "use_count")]
    pub use_count: u32,
    /// The running status of this module
    #[serde(rename = "status")]
    pub status: String,
    /// The support state of this module
    #[serde(rename = "support_level")]
    pub support_level: String,
}
