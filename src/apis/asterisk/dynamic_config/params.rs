use derive_new::new;
use derive_setters::Setters;
use serde::Serialize;

#[derive(Clone, Default, Debug, PartialEq, Serialize, new, Setters)]
#[setters(prefix = "with_")]
#[setters(into, strip_option)]
#[serde(rename_all = "camelCase")]
pub struct CreateUpdateRequest {
    /// The configuration class containing dynamic configuration objects.
    #[serde(skip_serializing)]
    #[setters(skip)]
    #[new(into)]
    /// The type of configuration object to retrieve.
    pub(crate) config_class: String,
    #[serde(skip_serializing)]
    #[setters(skip)]
    #[new(into)]
    pub(crate) object_type: String,

    /// The unique identifier of the object to retrieve.
    #[serde(skip_serializing)]
    #[setters(skip)]
    #[new(into)]
    pub(crate) id: String,

    /// The body object should have a value that is a list of ConfigTuples, which provide the fields to update.
    ///
    /// Ex. `[ { "attribute": "directmedia", "value": "false" } ]`
    #[serde(rename = "fields")]
    #[new(default)]
    config: Option<Vec<ConfigTuple>>,
}

/// ConfigTuple : A key/value pair that makes up part of a configuration object.
#[derive(Clone, Default, Debug, PartialEq, Serialize, new)]
pub struct ConfigTuple {
    /// A configuration object attribute.
    #[new(into)]
    pub attribute: String,
    /// The value for the attribute.
    #[new(into)]
    pub value: String,
}
