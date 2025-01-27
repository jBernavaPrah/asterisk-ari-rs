pub mod models;
pub mod params;

use crate::apis::client::Client;
use std::fmt::Display;

pub struct Endpoints<'c> {
    client: &'c Client,
}

impl<'c> Endpoints<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }
}

impl<'c> Endpoints<'c> {
    /// List all endpoints.
    pub async fn list(&self) -> crate::errors::Result<Vec<models::Endpoint>> {
        self.client.get("/endpoints").await
    }

    /// List available endpoints for a given endpoint technology.
    pub async fn list_by_technology(
        &self,
        tech: impl Into<String> + Display + Send,
    ) -> crate::errors::Result<Vec<models::Endpoint>> {
        self.client.get(format!("/endpoints/{tech}").as_str()).await
    }

    /// Details for an endpoint.
    pub async fn get(
        &self,
        tech: impl Into<String> + Send,
        resource: impl Into<String> + Send,
    ) -> crate::errors::Result<models::Endpoint> {
        self.client
            .get(format!("/endpoints/{}/{}", tech.into(), resource.into()).as_str())
            .await
    }

    /// Send a message to some technology URI or endpoint.
    pub async fn send_message(
        &self,
        request: params::SendMessageRequest,
    ) -> crate::errors::Result<()> {
        self.client
            .put_with_query(
                "/endpoints/sendMessage",
                &request.variables,
                &request,
            )
            .await
    }

    /// Send a message to some endpoint in a technology.
    pub async fn send_message_to_endpoint(
        &self,
        request: params::SendMessageToEndpointRequest,
    ) -> crate::errors::Result<()> {
        self.client
            .put_with_query(
                format!(
                    "/endpoints/{}/{}/sendMessage",
                    request.tech, request.resource
                )
                .as_str(),
                &request.variables,
                &request,
            )
            .await
    }

    /// Refer an endpoint or technology URI to some technology URI or endpoint
    pub async fn refer(&self, request: params::ReferRequest) -> crate::errors::Result<()> {
        self.client
            .put_with_query("/endpoints/refer", &request.variables, &request)
            .await
    }

    /// Refer an endpoint or technology URI to some technology URI or endpoint
    pub async fn refer_to_endpoint(
        &self,
        request: params::ReferToEndpointRequest,
    ) -> crate::errors::Result<()> {
        self.client
            .post_with_query(
                format!("/endpoints/{}/{}/refer", request.tech, request.resource).as_str(),
                &request.variables,
                &request,
            )
            .await
    }
}
