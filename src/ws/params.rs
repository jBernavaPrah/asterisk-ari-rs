use derive_new::new;
use derive_setters::Setters;
use serde::Serialize;

/// Represents a request to listen for ARI events.
///
/// This struct is used to configure the parameters for the listen request,
/// including the application name and whether to subscribe to all events.
#[derive(Clone, Debug, Serialize, new, Setters)]
#[setters(prefix = "with_")]
#[setters(into, strip_option)]
pub struct ListenRequest {
    /// The name of the application that will receive this event.
    ///
    /// This field is required and specifies the application that will handle the events.
    #[setters(skip)]
    #[new(into)]
    pub(crate) app: String,

    /// Subscribe to all Asterisk events.
    ///
    /// If provided, the applications listed will be subscribed to all events,
    /// effectively disabling the application-specific subscriptions.
    ///
    /// Default is `false`.
    #[serde(rename = "subscribeAll", skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub(crate) subscribe_all: Option<bool>,
}
