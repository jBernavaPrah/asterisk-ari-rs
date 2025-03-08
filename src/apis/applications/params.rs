use derive_new::new;
use derive_setters::Setters;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize, new, Setters)]
#[setters(prefix = "with_")]
#[setters(into, strip_option)]
#[serde(rename_all = "camelCase")]
pub struct UnSubscribeRequest {
    /// Application's name
    #[setters(skip)]
    #[serde(skip_serializing)]
    pub(crate) name: String,

    /// The application to subscribe to.
    #[setters(skip)]
    #[new(into_iter = "String")]
    pub(crate) event_source: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, new, Setters)]
#[setters(prefix = "with_")]
#[setters(into, strip_option)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeRequest {
    /// Application's name
    #[setters(skip)]
    #[serde(skip_serializing)]
    pub(crate) name: String,

    /// URI for event source (channel:{channelId}, bridge:{bridgeId}, endpoint:{tech}[/{resource}], deviceState:{deviceName}
    #[setters(skip)]
    #[new(into_iter = "String")]
    pub(crate) event_source: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, new, Setters)]
#[setters(prefix = "with_")]
#[setters(into, strip_option)]
pub struct FilterEventsRequest {
    /// Name of the application
    #[serde(skip_serializing)]
    #[setters(skip)]
    #[new(into)]
    pub(crate) name: String,

    /// Specify which event types to allow/disallow
    ///
    /// The body (parameter) should specify a JSON key/value object that describes the type of event filtering needed.
    /// One, or both of the following keys can be designated:
    ///
    /// \"allowed\" - Specifies an allowed list of event types
    /// \"disallowed\" - Specifies a disallowed list of event types
    ///
    /// Further, each of those key's value should be a JSON array that holds zero, or more JSON key/value objects.
    /// Each of these objects must contain the following key with an associated value:
    /// \"type\" - The type name of the event to filter
    ///
    /// The value must be the string name (case sensitive) of the event type that needs filtering.
    /// For example:
    ///
    /// `{ \"allowed\": [ { \"type\": \"StasisStart\" }, { \"type\": \"StasisEnd\" } ] }`
    ///
    /// As this specifies only an allowed list, then only those two event type messages are sent to the application.
    /// No other event messages are sent.
    ///
    /// The following rules apply:
    ///
    /// * If the body is empty, both the allowed and disallowed filters are set empty.
    /// * If both list types are given then both are set to their respective values (note, specifying an empty array for a given type sets that type to empty).
    /// * If only one list type is given then only that type is set. The other type is not updated.
    /// * An empty \"allowed\" list means all events are allowed.
    /// * An empty \"disallowed\" list means no events are disallowed.
    /// * Disallowed events take precedence over allowed events if the event type is specified in both lists.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub(crate) filter: Option<Filter>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, new, Setters)]
#[setters(prefix = "with_")]
#[setters(into, strip_option)]
pub struct Filter {
    /// Specifies an allowed list of event types
    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    allowed: Option<Vec<FilterType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    disallowed: Option<Vec<FilterType>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, new)]
pub struct FilterType {
    #[serde(rename = "type")]
    #[new(into)]
    name: String,
}
