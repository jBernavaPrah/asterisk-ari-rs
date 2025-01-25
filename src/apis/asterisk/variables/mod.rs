pub mod models;

use crate::apis::client::Client;

pub struct Variables<'c> {
    client: &'c Client,
}

impl<'c> Variables<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }
}

impl<'c> Variables<'c> {
    /// Get the value of a global variable.
    pub async fn get(
        &self,
        variable: impl Into<String> + Send,
    ) -> crate::errors::Result<models::Variable> {
        self.client
            .get_with_query("/asterisk/variable", &[("variable", variable.into())])
            .await
    }

    /// Set the value of a global variable.
    pub async fn set(
        &self,
        variable: impl Into<String> + Send,
        value: Option<impl Into<String> + Send>,
    ) -> crate::errors::Result<()> {
        self.client
            .post_with_query(
                "/asterisk/variable",
                vec![] as Vec<String>,
                &[
                    ("variable", variable.into()),
                    ("value", value.map(|v| v.into()).unwrap_or_default()),
                ],
            )
            .await
    }
}
