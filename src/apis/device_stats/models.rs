use serde::Deserialize;

/// DeviceState : Represents the state of a device.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct DeviceState {
    /// Name of the device.
    #[serde(rename = "name")]
    pub name: String,
    /// Device's state
    #[serde(rename = "state")]
    pub state: DeviceStateState,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Default)]
pub enum DeviceStateState {
    #[serde(rename = "UNKNOWN")]
    #[default]
    Unknown,
    #[serde(rename = "NOT_INUSE")]
    NotInuse,
    #[serde(rename = "INUSE")]
    Inuse,
    #[serde(rename = "BUSY")]
    Busy,
    #[serde(rename = "INVALID")]
    Invalid,
    #[serde(rename = "UNAVAILABLE")]
    Unavailable,
    #[serde(rename = "RINGING")]
    Ringing,
    #[serde(rename = "RINGINUSE")]
    Ringinuse,
    #[serde(rename = "ONHOLD")]
    Onhold,
}
