pub mod models;

use crate::apis::client::Client;
use serde::Serialize;

pub struct Playbacks<'c> {
    client: &'c Client,
}

impl<'c> Playbacks<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }
}

impl<'c> Playbacks<'c> {
    pub async fn get(
        &self,
        playback_id: impl Into<String> + Sized,
    ) -> crate::errors::Result<Vec<models::Playback>> {
        self.client
            .get(format!("/playback/{}", playback_id.into()).as_str())
            .await
    }

    pub async fn control(
        &self,
        playback_id: impl Into<String> + Sized,
        operation: Operation,
    ) -> crate::errors::Result<()> {
        self.client
            .post_with_query(
                format!("/playback/{}/control", playback_id.into()).as_str(),
                vec![] as Vec<String>,
                &[("operation", operation)],
            )
            .await
    }

    /// Stop a playback
    pub async fn stop(&self, playback_id: impl Into<String> + Sized) -> crate::errors::Result<()> {
        self.client
            .delete(format!("/playback/{}", playback_id.into()).as_str())
            .await
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum Operation {
    #[serde(rename = "restart")]
    Restart,
    #[serde(rename = "pause")]
    Pause,
    #[serde(rename = "unpause")]
    Unpause,
    #[serde(rename = "reverse")]
    Reverse,
    #[serde(rename = "forward")]
    Forward,
}
