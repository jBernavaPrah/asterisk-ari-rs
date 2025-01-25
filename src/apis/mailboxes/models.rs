use serde::Deserialize;

/// Mailbox : Represents the state of a mailbox.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct Mailbox {
    /// Name of the mailbox.
    #[serde(rename = "name")]
    pub name: String,

    /// Count of old messages in the mailbox.,
    #[serde(rename = "old_messages")]
    pub old_messages: u32,

    /// Count of new messages in the mailbox.
    #[serde(rename = "new_messages")]
    pub new_messages: u32,
}
