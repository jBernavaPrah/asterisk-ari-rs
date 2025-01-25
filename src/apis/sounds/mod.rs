pub mod models;

use crate::apis::client::Client;
use std::fmt::Display;

pub struct Sounds<'c> {
    client: &'c Client,
}

impl<'c> Sounds<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }
}

impl<'c> Sounds<'c> {
    /// List all sounds.
    pub async fn list(&self) -> crate::errors::Result<Vec<models::Sound>> {
        self.client.get("/sounds").await
    }

    /// Get a sound's details.
    pub async fn get(
        &self,
        sound_id: impl Into<String> + Display + Send,
    ) -> crate::errors::Result<models::Sound> {
        self.client
            .get(format!("/sounds/{sound_id}").as_str())
            .await
    }
}
