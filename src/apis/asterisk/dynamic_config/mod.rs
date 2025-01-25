pub mod models;
pub mod params;

use crate::apis::client::Client;
use std::fmt::Display;

pub struct DynamicConfiguration<'c> {
    client: &'c Client,
}

impl<'c> DynamicConfiguration<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Retrieve a dynamic configuration object.
    pub async fn get(
        &self,
        config_class: impl Into<String> + Send,
        object_type: impl Into<String> + Send,
        id: impl Into<String> + Send,
    ) -> crate::errors::Result<models::ConfigTuple> {
        self.client
            .get(
                format!(
                    "/asterisk/config/dynamic/{}/{}/{}",
                    config_class.into(),
                    object_type.into(),
                    id.into()
                )
                .as_str(),
            )
            .await
    }

    /// Create or update a dynamic configuration object.
    pub async fn put(&self, request: params::CreateUpdateRequest) -> crate::errors::Result<()> {
        self.client
            .put(
                format!(
                    "/asterisk/config/dynamic/{}/{}/{}",
                    request.config_class, request.object_type, request.id
                )
                .as_str(),
                request,
            )
            .await
    }

    /// Delete a dynamic configuration object
    pub async fn delete(
        &self,
        config_class: impl Into<String> + Display + Send,
        object_type: impl Into<String> + Display + Send,
        id: impl Into<String> + Display + Send,
    ) -> crate::errors::Result<()> {
        self.client
            .delete(format!("/asterisk/config/dynamic/{config_class}/{object_type}/{id}",).as_str())
            .await
    }
}
