use derive_new::new;
use derive_setters::Setters;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, new, Setters)]
#[setters(prefix = "with_")]
#[setters(into, strip_option)]
pub struct ReferRequest {
    /// The endpoint resource or technology specific URI to send the message to. Valid resources are pjsip, and xmpp.
    #[setters(skip)]
    #[new(into)]
    pub(crate) to: String,

    /// The endpoint resource or technology specific identity to send this message from. Valid resources are pjsip, and xmpp.
    #[setters(skip)]
    #[new(into)]
    pub(crate) from: String,

    /// The endpoint resource or technology specific URI to refer to.
    #[setters(skip)]
    #[new(into)]
    pub(crate) refer_to: String,

    /// If true and "refer_to" refers to an Asterisk endpoint, the "refer_to" value is set to point to this Asterisk endpoint - so the referee is referred to Asterisk. Otherwise, use the contact URI associated with the endpoint.
    #[serde(rename = "to_self", skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub(crate) to_self: Option<bool>,

    /// The "variables" key in the body object holds technology specific key/value pairs to append to the message.
    ///
    /// These can be interpreted and used by the various resource types;
    /// for example, the pjsip resource type will add the key/value pairs as SIP headers.
    /// The "display_name" key is used by the PJSIP technology.
    /// Its value will be prepended as a display name to the Refer-To URI.
    #[serde(skip_serializing)]
    #[new(default)]
    pub(crate) variables: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, new, Setters)]
#[setters(prefix = "with_")]
#[setters(into, strip_option)]
pub struct SendMessageRequest {
    /// The endpoint resource or technology specific URI to send the message to. Valid resources are pjsip, and xmpp.
    #[setters(skip)]
    #[new(into)]
    pub(crate) to: String,

    /// The endpoint resource or technology specific identity to send this message from. Valid resources are pjsip, and xmpp.
    #[setters(skip)]
    #[new(into)]
    pub(crate) from: String,

    /// The body of the message.
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub(crate) body: Option<String>,

    /// The "variables" key in the body object holds technology specific key/value pairs to append to the message.
    ///
    /// These can be interpreted and used by the various resource types;
    /// for example, the pjsip resource type will add the key/value pairs as SIP headers.
    /// The "display_name" key is used by the PJSIP technology.
    /// Its value will be prepended as a display name to the Refer-To URI.
    #[serde(skip)]
    #[new(default)]
    pub(crate) variables: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, new, Setters)]
#[setters(prefix = "with_")]
#[setters(into, strip_option)]
pub struct SendMessageToEndpointRequest {
    /// Technology of the endpoint
    #[serde(skip_serializing)]
    #[setters(skip)]
    #[new(into)]
    pub(crate) tech: String,

    /// ID of the endpoint
    #[serde(skip_serializing)]
    #[setters(skip)]
    #[new(into)]
    pub(crate) resource: String,

    /// The endpoint resource or technology specific identity to send this message from. Valid resources are pjsip, and xmpp.
    #[setters(skip)]
    #[new(into)]
    pub(crate) from: String,

    /// The body of the message.
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    #[new(default)]
    pub(crate) body: Option<String>,

    /// The "variables" key in the body object holds technology specific key/value pairs to append to the message.
    ///
    /// These can be interpreted and used by the various resource types;
    /// for example, the pjsip resource type will add the key/value pairs as SIP headers.
    /// The "display_name" key is used by the PJSIP technology.
    /// Its value will be prepended as a display name to the Refer-To URI.
    #[serde(skip)]
    #[new(default)]
    pub(crate) variables: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, new, Setters)]
#[setters(prefix = "with_")]
#[setters(into, strip_option)]
pub struct ReferToEndpointRequest {
    /// Technology of the endpoint
    #[serde(skip_serializing)]
    #[setters(skip)]
    #[new(into)]
    pub(crate) tech: String,

    /// ID of the endpoint
    #[serde(skip_serializing)]
    #[setters(skip)]
    #[new(into)]
    pub(crate) resource: String,

    /// The endpoint resource or technology specific identity to send this message from.
    ///
    /// Valid resources are pjsip, and xmpp.
    #[setters(skip)]
    #[new(into)]
    from: String,

    /// The endpoint resource or technology specific URI to refer to.
    #[setters(skip)]
    #[new(into)]
    refer_to: String,

    /// If true and "refer_to" refers to an Asterisk endpoint,
    /// the "refer_to" value is set to point to this Asterisk endpoint - so the referee is referred to Asterisk.
    /// Otherwise, use the contact URI associated with the endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[new(default)]
    to_self: Option<bool>,

    /// The "variables" key in the body object holds technology specific key/value pairs to append to the message.
    ///
    /// These can be interpreted and used by the various resource types;
    /// for example, the pjsip resource type will add the key/value pairs as SIP headers.
    /// The "display_name" key is used by the PJSIP technology.
    /// Its value will be prepended as a display name to the Refer-To URI.
    #[serde(skip_serializing)]
    #[new(default)]
    pub(crate) variables: Option<serde_json::Value>,
}
