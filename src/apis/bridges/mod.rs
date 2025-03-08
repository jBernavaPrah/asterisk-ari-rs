use crate::apis::client::Client;
use std::fmt::Display;

pub mod models;
mod params;

pub struct Bridges<'c> {
    client: &'c Client,
}

impl<'c> Bridges<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }
}

impl Bridges<'_> {
    pub async fn list(&self) -> crate::errors::Result<Vec<models::Bridge>> {
        self.client.get("/bridges").await
    }

    pub async fn create(
        &self,
        request: params::CreateRequest,
    ) -> crate::errors::Result<models::Bridge> {
        self.client
            .post_with_query("/bridges", vec![] as Vec<String>, &request)
            .await
    }

    pub async fn create_with_id(
        &self,
        request: params::CreateWithIdRequest,
    ) -> crate::errors::Result<models::Bridge> {
        self.client
            .post_with_query(
                format!("/bridges/{}", request.bridge_id).as_str(),
                vec![] as Vec<String>,
                &request,
            )
            .await
    }

    pub async fn get(
        &self,
        bridge_id: impl Into<String> + Display + Send,
    ) -> crate::errors::Result<models::Bridge> {
        self.client
            .get(format!("/bridges/{bridge_id}").as_str())
            .await
    }

    pub async fn delete(
        &self,
        bridge_id: impl Into<String> + Display + Send,
    ) -> crate::errors::Result<models::Bridge> {
        self.client
            .delete(format!("/bridges/{bridge_id}").as_str())
            .await
    }

    pub async fn add_channel(
        &self,
        request: params::AddChannelRequest,
    ) -> crate::errors::Result<()> {
        self.client
            .post_with_query(
                format!("/bridges/{}", request.bridge_id).as_str(),
                vec![] as Vec<String>,
                &request,
            )
            .await
    }

    pub async fn remove_channel(
        &self,
        bridge_id: impl Into<String> + Display + Send,
        channel: impl Into<String> + Send,
    ) -> crate::errors::Result<()> {
        self.client
            .post_with_query(
                format!("/bridges/{bridge_id}").as_str(),
                vec![] as Vec<String>,
                &[("channel", channel.into())],
            )
            .await
    }

    /// Play music on hold to a bridge or change the MOH class that is playing.
    pub async fn moh(&self, request: params::MohRequest) -> crate::errors::Result<()> {
        self.client
            .post_with_query(
                format!("/bridges/{}/moh", request.bridge_id).as_str(),
                vec![] as Vec<String>,
                &request,
            )
            .await
    }

    /// Stop playing music on hold to a bridge.
    pub async fn unmoh(
        &self,
        bridge_id: impl Into<String> + Display + Send,
    ) -> crate::errors::Result<()> {
        self.client
            .delete(format!("/bridges/{bridge_id}/moh").as_str())
            .await
    }

    /// Start playback of media.
    ///
    /// *Implementation Notes*
    /// The media URI may be any of a number of URI's. Currently sound:, recording:, number:, digits:, characters:, and tone: URI's are supported. This operation creates a playback resource that can be used to control the playback of media (pause, rewind, fast forward, etc.)
    pub async fn play(
        &self,
        request: params::PlayRequest,
    ) -> crate::errors::Result<crate::apis::playbacks::models::Playback> {
        self.client
            .post_with_query(
                format!("/bridges/{}/play", request.bridge_id).as_str(),
                vec![] as Vec<String>,
                &request,
            )
            .await
    }

    /// Start playback of media.
    ///
    /// *Implementation Notes*
    /// The media URI may be any of a number of URI's. Currently sound:, recording:, number:, digits:, characters:, and tone: URI's are supported. This operation creates a playback resource that can be used to control the playback of media (pause, rewind, fast forward, etc.)
    pub async fn play_with_id(
        &self,
        request: params::PlayWithIdRequest,
    ) -> crate::errors::Result<crate::apis::playbacks::models::Playback> {
        self.client
            .post_with_query(
                format!(
                    "/bridges/{}/play/{}",
                    request.bridge_id, request.playback_id
                )
                .as_str(),
                vec![] as Vec<String>,
                &request,
            )
            .await
    }

    /// Start a recording
    ///
    /// *Implementation Notes*
    /// This records the mixed audio from all channels participating in this bridge.
    pub async fn record(
        &self,
        request: params::RecordRequest,
    ) -> crate::errors::Result<crate::apis::recordings::models::LiveRecording> {
        self.client
            .post_with_query(
                format!("/bridges/{}/record", request.bridge_id).as_str(),
                vec![] as Vec<String>,
                &request,
            )
            .await
    }
}
