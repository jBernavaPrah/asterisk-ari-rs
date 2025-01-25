use chrono::{DateTime, Utc};
use serde::Deserialize;

/// AsteriskInfo : Asterisk system information
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct AsteriskInfo {
    /// Info about how Asterisk was built,
    pub build: Option<BuildInfo>,
    /// Info about the system running Asterisk,
    pub system: Option<SystemInfo>,
    /// Info about Asterisk configuration,
    pub config: Option<ConfigInfo>,
    /// Info about Asterisk status
    pub status: Option<StatusInfo>,
}

/// AsteriskPing : Asterisk ping information
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct AsteriskPing {
    /// Asterisk id info
    pub asterisk_id: String,
    /// Always string value is pong
    pub ping: String,
    /// The timestamp string of request received time
    pub timestamp: DateTime<Utc>,
}

/// BuildInfo : Info about how Asterisk was built
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct BuildInfo {
    /// OS Asterisk was built on.
    pub os: String,
    /// Kernel version Asterisk was built on.
    pub kernel: String,
    /// Compile time options, or empty string if default.
    pub options: String,
    /// Machine architecture (x86_64, i686, ppc, etc.)
    pub machine: String,
    /// Date and time when Asterisk was built.
    pub date: String,
    /// Username that build Asterisk
    pub user: String,
}

/// ConfigInfo : Info about Asterisk configuration
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct ConfigInfo {
    /// Asterisk system name.
    pub name: String,
    /// Default language for media playback.
    pub default_language: String,
    /// Maximum number of simultaneous channels.,
    pub max_channels: Option<u32>,
    /// Maximum number of open file handles (files, sockets).,
    pub max_open_files: Option<u32>,
    /// Maximum load avg on system.,
    pub max_load: Option<f32>,
    /// Effective user/group id for running Asterisk.
    pub set_id: Option<SetId>,
}

/// SetId : Effective user/group id
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct SetId {
    /// Effective user id.
    pub user: String,
    /// Effective group id.
    pub group: String,
}

/// StatusInfo : Info about Asterisk status
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct StatusInfo {
    /// Time when Asterisk was started.
    pub startup_time: DateTime<Utc>,
    /// Time when Asterisk was last reloaded.
    pub last_reload_time: DateTime<Utc>,
}

/// SystemInfo : Info about Asterisk
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct SystemInfo {
    /// Asterisk version.
    pub version: String,

    pub entity_id: String,
}
