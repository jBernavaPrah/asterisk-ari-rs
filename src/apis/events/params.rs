use derive_new::new;
use derive_setters::Setters;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, new, Setters)]
#[setters(prefix = "with_")]
#[setters(into, strip_option)]
pub struct GenerateRequest {
    /// Event name
    #[serde(skip_serializing)]
    #[setters(skip)]
    #[new(into)]
    pub(crate) event_name: String,

    /// The name of the application that will receive this event
    #[setters(skip)]
    #[new(into)]
    pub(crate) application: String,

    /// URI for event source (channel:{channelId}, bridge:{bridgeId}, endpoint:{tech}/{resource}, deviceState:{deviceName}
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    #[new(default)]
    source: Option<String>,

    /// The "variables" key in the body object holds custom key/value pairs to add to the user event.
    /// Ex. { "variables": { "key": "value" } }
    #[serde(skip_serializing)]
    #[new(default)]
    pub(crate) variables: Option<serde_json::Value>,
}
