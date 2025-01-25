use crate::apis::concat_str;
use derive_new::new;
use serde::Serialize;

#[derive(Clone, Default, Debug, PartialEq, Serialize, new)]
pub struct AddRequest {
    #[serde(skip_serializing)]
    pub(crate) name: String,

    #[serde(serialize_with = "concat_str")]
    configuration: Vec<String>,
}
