use serde::Deserialize;

/// LogChannel : Details of an Asterisk log channel
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct LogChannel {
    /// The log channel path
    pub channel: String,
    /// Types of logs for the log channel
    pub r#type: String,
    /// Whether a log type is enabled
    pub status: String,
    /// The various log levels
    pub configuration: String,
}
