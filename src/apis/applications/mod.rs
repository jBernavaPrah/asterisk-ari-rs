pub mod models;
pub mod params;

use crate::apis::client::Client;
use crate::errors::Result;

pub struct Applications<'c> {
    client: &'c Client,
}

impl<'c> Applications<'c> {
    pub fn new(client: &'c Client) -> Self {
        Applications { client }
    }
}

impl<'c> Applications<'c> {
    /// List all applications
    pub async fn list(&self) -> Result<Vec<models::Application>> {
        self.client.get("/applications").await
    }

    /// Get details of an application.
    pub async fn get(
        &self,
        application_name: impl Into<String> + Send,
    ) -> Result<models::Application> {
        self.client
            .get(format!("/applications/{}", application_name.into()).as_str())
            .await
    }

    /// Subscribe an application to an event source.
    /// Implementation Notes
    /// Returns the state of the application after the subscriptions have changed
    pub async fn subscribe(
        &self,
        request: params::SubscribeRequest,
    ) -> Result<models::Application> {
        self.client
            .post_with_query(
                format!("/applications/{}/subscription", request.name).as_str(),
                vec![] as Vec<String>,
                &request,
            )
            .await
    }

    /// Unsubscribe an application from an event source.
    /// Implementation Notes
    /// Returns the state of the application after the subscriptions have changed
    pub async fn unsubscribe(
        &self,
        request: params::UnSubscribeRequest,
    ) -> Result<models::Application> {
        self.client
            .delete_with_query(
                format!("/applications/{}/subscription", request.name).as_str(),
                &request,
            )
            .await
    }

    /// Filter application events types
    ///
    /// Allowed and/or disallowed event type filtering can be done.
    pub async fn filter_events(
        &self,
        request: params::FilterEventsRequest,
    ) -> Result<models::Application> {
        self.client
            .put(
                format!("/applications/{}/eventFilter", request.name).as_str(),
                request,
            )
            .await
    }
}
