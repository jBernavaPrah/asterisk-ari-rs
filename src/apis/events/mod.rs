pub mod models;
pub mod params;

use crate::apis::client::Client;

pub struct Events<'c> {
    client: &'c Client,
}

impl<'c> Events<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }
}

impl<'c> Events<'c> {
    /// Generate a user event.
    pub async fn generate(&self, request: params::GenerateRequest) -> crate::errors::Result<()> {
        self.client
            .post_with_query(
                format!("/events/user/{}", request.event_name).as_str(),
                request.variables.clone(),
                &request,
            )
            .await
    }
}
