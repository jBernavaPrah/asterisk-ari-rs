use serde::Deserialize;

/// Endpoint : An external device that may offer/accept calls to/from Asterisk.
///
/// Unlike most resources, which have a single unique identifier, an endpoint is uniquely identified by the technology/resource pair.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct Endpoint {
    /// Technology of the endpoint
    pub technology: String,
    /// Identifier of the endpoint, specific to the given technology.
    pub resource: String,
    /// Endpoint's state
    pub state: Option<State>,
    /// Id's of channels associated with this endpoint
    pub channel_ids: Vec<String>,
}

///  ['unknown' or 'offline' or 'online']
#[derive(Clone, Debug, PartialEq, Deserialize, Default)]
pub enum State {
    #[serde(rename = "unknown")]
    #[default]
    Unknown,
    #[serde(rename = "offline")]
    Offline,
    #[serde(rename = "online")]
    Online,
}

/// TextMessage : A text message.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct TextMessage {
    /// A technology specific URI specifying the source of the message. For pjsip technology, any SIP URI can be specified. For xmpp, the URI must correspond to the client connection being used to send the message.
    pub from: Option<String>,
    /// A technology specific URI specifying the destination of the message. Valid technologies include pjsip, and xmp. The destination of a message should be an endpoint.
    pub to: Option<String>,
    /// The text of the message.
    pub body: Option<String>,
    /// Technology specific key/value pairs (JSON object) associated with the message.
    pub variables: Option<serde_json::Value>,
}
