use serde::Serializer;

pub mod applications;
pub mod asterisk;
pub mod bridges;
pub mod channels;
pub mod client;
pub mod device_stats;
pub mod endpoints;
pub mod events;
pub mod mailboxes;
pub mod playbacks;
pub mod recordings;
pub mod sounds;

pub mod params {
    use serde::{Deserialize, Serialize};

    /// Represents the direction of a call or event.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub enum Direction {
        /// Both directions (incoming and outgoing).
        #[default]
        Both,
        /// Outgoing direction.
        Out,
        /// Incoming direction.
        In,
    }

    /// Specifies the behavior if a resource already exists.
    #[derive(Clone, Debug, PartialEq, Serialize, Default)]
    pub enum IfExists {
        /// Overwrite the existing resource.
        #[serde(rename = "overwrite")]
        #[default]
        Overwrite,
        /// Fail if the resource exists.
        #[serde(rename = "fail")]
        Fail,
        /// Append to the existing resource.
        #[serde(rename = "append")]
        Append,
    }

    /// Specifies the termination condition for a call or event.
    #[derive(Clone, Debug, PartialEq, Serialize, Default)]
    pub enum TerminateOn {
        /// No termination condition.
        #[serde(rename = "none")]
        #[default]
        None,
        /// Terminate on any condition.
        #[serde(rename = "any")]
        Any,
        /// Terminate on the '*' key press.
        #[serde(rename = "*")]
        Start,
        /// Terminate on the '#' key press.
        #[serde(rename = "#")]
        Hash,
    }
}

/// Serializes an optional vector of strings by concatenating them with commas.
///
/// # Arguments
///
/// * `x` - The optional vector of strings to serialize.
/// * `s` - The serializer to use.
///
/// # Returns
///
/// A result containing the serialized string or an error.
fn concat_option_str<S>(x: &Option<Vec<String>>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match x {
        Some(x) => s.serialize_str(x.join(",").as_str()),
        None => s.serialize_none(),
    }
}

/// Serializes a vector of strings by concatenating them with commas.
///
/// # Arguments
///
/// * `x` - The vector of strings to serialize.
/// * `s` - The serializer to use.
///
/// # Returns
///
/// A result containing the serialized string or an error.
fn concat_str<S>(x: &Vec<String>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(x.join(",").as_str())
}