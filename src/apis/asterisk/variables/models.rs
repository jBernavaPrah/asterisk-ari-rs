use serde::Deserialize;

/// Variable : The value of a channel variable
#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
pub struct Variable {
    /// The value of the variable requested
    pub value: String,
}
