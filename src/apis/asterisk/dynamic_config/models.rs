use serde::Deserialize;

/// ConfigTuple : A key/value pair that makes up part of a configuration object.
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct ConfigTuple {
    /// A configuration object attribute.
    pub attribute: String,
    /// The value for the attribute.
    pub value: String,
}
