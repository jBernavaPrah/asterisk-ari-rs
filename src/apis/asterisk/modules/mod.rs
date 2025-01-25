pub mod models;

use crate::apis::client::Client;

pub struct Modules<'c> {
    client: &'c Client,
}

impl<'c> Modules<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }
}

impl<'c> Modules<'c> {
    /// List Asterisk modules.
    pub async fn list(&self) -> crate::errors::Result<Vec<models::Module>> {
        self.client.get("/asterisk/modules").await
    }

    /// Get Asterisk module information.
    pub async fn get(
        &self,
        name: impl Into<String> + Send,
    ) -> crate::errors::Result<models::Module> {
        self.client
            .get(format!("/asterisk/modules/{}", name.into(),).as_str())
            .await
    }

    /// Load an Asterisk module.
    pub async fn load(&self, name: impl Into<String> + Send) -> crate::errors::Result<()> {
        self.client
            .post(
                format!("/asterisk/modules/{}", name.into(),).as_str(),
                vec![] as Vec<String>,
            )
            .await
    }

    /// Unload an Asterisk module.
    pub async fn unload(&self, name: impl Into<String> + Send) -> crate::errors::Result<()> {
        self.client
            .delete(format!("/asterisk/modules/{}", name.into()).as_str())
            .await
    }

    /// Reload an Asterisk module
    pub async fn reload(&self, name: impl Into<String> + Send) -> crate::errors::Result<()> {
        self.client
            .put(
                format!("/asterisk/modules/{}", name.into()).as_str(),
                vec![] as Vec<String>,
            )
            .await
    }
}
