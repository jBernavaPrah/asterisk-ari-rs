pub mod models;
pub mod params;

use crate::apis::client::Client;
use std::fmt::Display;

pub struct Logging<'c> {
    client: &'c Client,
}

impl<'c> Logging<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }
}

impl<'c> Logging<'c> {
    /// Gets Asterisk log channel information.
    pub async fn list(&self) -> crate::errors::Result<Vec<models::LogChannel>> {
        self.client.get("/asterisk/logging").await
    }

    /// Add a log channel
    ///
    /// Example: NOTICE WARNING ERROR VERBOSE
    pub async fn add(&self, request: params::AddRequest) -> crate::errors::Result<()> {
        self.client
            .post_with_query(
                format!("/asterisk/logging/{}", request.name).as_str(),
                vec![] as Vec<String>,
                &request,
            )
            .await
    }

    /// Deletes a log channel.
    pub async fn delete(
        &self,
        name: impl Into<String> + Display + Send,
    ) -> crate::errors::Result<()> {
        self.client
            .delete(format!("/asterisk/logging/{name}").as_str())
            .await
    }

    /// Rotates a log channel
    pub async fn rotate(
        &self,
        name: impl Into<String> + Display + Send,
    ) -> crate::errors::Result<()> {
        self.client
            .put(
                format!("/asterisk/logging/{name}/rotate",).as_str(),
                vec![] as Vec<String>,
            )
            .await
    }
}
