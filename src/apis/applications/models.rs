use serde::Deserialize;

/// Application : Details of a Stasis application
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct Application {
    /// Name of this application
    #[serde(rename = "name")]
    pub name: String,
    /// Id's for channels subscribed to.,
    #[serde(rename = "channel_ids")]
    pub channel_ids: Vec<String>,

    /// Id's for bridges subscribed to.,
    #[serde(rename = "bridge_ids")]
    pub bridge_ids: Vec<String>,

    /// {tech}/{resource} for endpoints subscribed to.,
    #[serde(rename = "endpoint_ids")]
    pub endpoint_ids: Vec<String>,

    /// Names of the devices subscribed to.,
    #[serde(rename = "device_names")]
    pub device_names: Vec<String>,

    /// Event types sent to the application.,
    #[serde(rename = "events_allowed")]
    pub events_allowed: Vec<serde_json::Value>,

    /// Event types not sent to the application.
    #[serde(rename = "events_disallowed")]
    pub events_disallowed: Vec<serde_json::Value>,
}
