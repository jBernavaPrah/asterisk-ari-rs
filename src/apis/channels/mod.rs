use crate::apis::client::Client;
use std::fmt::Display;

pub mod models;
pub mod params;

pub struct Channels<'c> {
    client: &'c Client,
}

impl<'c> Channels<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }
}

impl<'c> Channels<'c> {
    /// List all active channels in Asterisk.
    pub async fn list(&self) -> crate::errors::Result<Vec<models::Channel>> {
        self.client.get("/channels").await
    }

    /// Channel details
    pub async fn get(
        &self,
        channel_id: impl Into<String> + Display + Send,
    ) -> crate::errors::Result<models::Channel> {
        self.client
            .get(format!("/channels/{channel_id}").as_str())
            .await
    }

    /// Create a new channel
    ///
    /// *Implementation Notes*
    /// The new channel is created immediately and a snapshot of it returned.
    /// If a Stasis application is provided it will be automatically subscribed to the originated channel for further events and updates.
    pub async fn originate(
        &self,
        request: params::OriginateRequest,
    ) -> crate::errors::Result<models::Channel> {
        self.client
            .post_with_query("/channels", request.variables.clone(), &request)
            .await
    }

    /// Create a channel
    ///
    /// *Implementation Notes*
    /// The new channel is created immediately and a snapshot of it returned.
    /// If a Stasis application is provided it will be automatically subscribed to the originated channel for further events and updates.
    pub async fn create(
        &self,
        request: params::CreateRequest,
    ) -> crate::errors::Result<models::Channel> {
        self.client
            .post_with_query("/channels", request.variables.clone(), &request)
            .await
    }

    /// Create a new channel (originate with id).
    ///
    /// *Implementation Notes*
    /// The new channel is created immediately and a snapshot of it returned.
    /// If a Stasis application is provided it will be automatically subscribed to the originated channel for further events and updates.
    pub async fn create_with_id(
        &self,
        request: params::OriginateWithIdRequest,
    ) -> crate::errors::Result<models::Channel> {
        self.client
            .post_with_query(
                format!("/channels/{}", request.channel_id).as_str(),
                request.variables.clone(),
                &request,
            )
            .await
    }

    /// Delete (i.e. hangup) a channel.
    pub async fn delete(&self, request: params::DeleteRequest) -> crate::errors::Result<()> {
        self.client
            .delete_with_query(
                format!("/channels/{}", request.channel_id).as_str(),
                &request,
            )
            .await
    }

    /// Exit application; continue execution in the dialplan.
    pub async fn r#continue(&self, request: params::ContinueRequest) -> crate::errors::Result<()> {
        self.client
            .post_with_query(
                format!("/channels/{}/continue", request.channel_id).as_str(),
                vec![] as Vec<String>,
                &request,
            )
            .await
    }

    /// Move the channel from one Stasis application to another
    pub async fn r#move(&self, request: params::MoveRequest) -> crate::errors::Result<()> {
        self.client
            .post_with_query(
                format!("/channels/{}/move", request.channel_id).as_str(),
                vec![] as Vec<String>,
                &request,
            )
            .await
    }

    /// Redirect the channel to a different location.
    pub async fn redirect(
        &self,
        channel_id: impl Into<String> + Display + Send,
        endpoint: impl Into<String> + Send,
    ) -> crate::errors::Result<()> {
        self.client
            .post_with_query(
                format!("/channels/{channel_id}/redirect").as_str(),
                vec![] as Vec<String>,
                &[("endpoint", endpoint.into())],
            )
            .await
    }

    /// Answer a channel
    pub async fn answer(
        &self,
        channel_id: impl Into<String> + Display + Send,
    ) -> crate::errors::Result<()> {
        self.client
            .post(
                format!("/channels/{channel_id}/answer",).as_str(),
                vec![] as Vec<String>,
            )
            .await
    }

    /// Indicate ringing to a channel.
    pub async fn ring(
        &self,
        channel_id: impl Into<String> + Display + Send,
    ) -> crate::errors::Result<()> {
        self.client
            .post(
                format!("/channels/{channel_id}/ring").as_str(),
                vec![] as Vec<String>,
            )
            .await
    }

    /// Stop ringing indication on a channel if locally generated.
    pub async fn stop_ring(
        &self,
        channel_id: impl Into<String> + Display + Send,
    ) -> crate::errors::Result<()> {
        self.client
            .delete(format!("/channels/{channel_id}/ring").as_str())
            .await
    }

    /// Send provided DTMF to a given channel.
    pub async fn dtmf(&self, request: params::DtmfRequest) -> crate::errors::Result<()> {
        self.client
            .post_with_query(
                format!("/channels/{}/dtmf", request.channel_id).as_str(),
                vec![] as Vec<String>,
                &request,
            )
            .await
    }

    /// Mute a channel.
    pub async fn mute(
        &self,
        channel_id: impl Into<String> + Display + Send,
        direction: crate::apis::params::Direction,
    ) -> crate::errors::Result<()> {
        self.client
            .post_with_query(
                format!("/channels/{channel_id}/mute",).as_str(),
                vec![] as Vec<String>,
                &[("direction", direction)],
            )
            .await
    }

    /// Unmute a channel.
    pub async fn unmute(
        &self,
        channel_id: impl Into<String> + Display + Send,
        direction: crate::apis::params::Direction,
    ) -> crate::errors::Result<()> {
        self.client
            .delete_with_query(
                format!("/channels/{channel_id}/mute").as_str(),
                &[("direction", direction)],
            )
            .await
    }

    /// Hold a channel.
    pub async fn hold(
        &self,
        channel_id: impl Into<String> + Display + Send,
    ) -> crate::errors::Result<()> {
        self.client
            .post(
                format!("/channels/{channel_id}/hold").as_str(),
                vec![] as Vec<String>,
            )
            .await
    }

    /// Remove a channel from hold.
    pub async fn unhold(
        &self,
        channel_id: impl Into<String> + Display + Send,
    ) -> crate::errors::Result<()> {
        self.client
            .delete(format!("/channels/{channel_id}/hold").as_str())
            .await
    }

    /// Play music on hold to a channel.
    ///
    /// *Implementation Notes*
    /// Using media operations such as /play on a channel playing MOH in this manner will suspend MOH without resuming automatically.
    /// If continuing music on hold is desired, the stasis application must reinitiate music on hold.
    pub async fn moh(&self, request: params::MohRequest) -> crate::errors::Result<()> {
        self.client
            .post_with_query(
                format!("/channels/{}/moh", request.channel_id).as_str(),
                vec![] as Vec<String>,
                &request,
            )
            .await
    }

    /// Stop playing music on hold to a channel.
    pub async fn unmoh(
        &self,
        channel_id: impl Into<String> + Display + Send,
    ) -> crate::errors::Result<()> {
        self.client
            .delete(format!("/channels/{channel_id}/moh",).as_str())
            .await
    }

    /// Play silence to a channel.
    ///
    /// *Implementation Notes*
    /// Using media operations such as /play on a channel playing silence in this manner will suspend silence without resuming automatically.
    pub async fn silence(
        &self,
        channel_id: impl Into<String> + Display + Send,
    ) -> crate::errors::Result<()> {
        self.client
            .post(
                format!("/channels/{channel_id}/silence").as_str(),
                vec![] as Vec<String>,
            )
            .await
    }

    /// Stop playing silence to a channel.
    pub async fn stop_silence(
        &self,
        channel_id: impl Into<String> + Display + Send,
    ) -> crate::errors::Result<()> {
        self.client
            .delete(format!("/channels/{channel_id}/silence").as_str())
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
                format!("/channels/{}/play", request.channel_id).as_str(),
                vec![] as Vec<String>,
                &request,
            )
            .await
    }

    /// Start playback of media and specify the playbackId.
    ///
    /// *Implementation Notes*
    /// The media URI may be any of a number of URI's. Currently sound:, recording:, number:, digits:, characters:, and tone: URI's are supported. This operation creates a playback resource that can be used to control the playback of media (pause, rewind, fast forward, etc.)
    pub async fn play_with_playback_id(
        &self,
        request: params::PlayWithPlaybackIdRequest,
    ) -> crate::errors::Result<crate::apis::playbacks::models::Playback> {
        self.client
            .post_with_query(
                format!(
                    "/channels/{}/play/{}",
                    request.channel_id, request.playback_id
                )
                .as_str(),
                vec![] as Vec<String>,
                &request,
            )
            .await
    }

    /// Start a recording.
    ///
    /// *Implementation Notes*
    /// Record audio from a channel.
    /// Note that this will not capture audio sent to the channel.
    /// The bridge itself has a record feature if that's what you want.
    pub async fn record(
        &self,
        request: params::RecordRequest,
    ) -> crate::errors::Result<crate::apis::recordings::models::LiveRecording> {
        self.client
            .post_with_query(
                format!("/channels/{}/record", request.channel_id).as_str(),
                vec![] as Vec<String>,
                &request,
            )
            .await
    }

    /// Get the value of a channel variable or function
    pub async fn get_variable(
        &self,
        channel_id: impl Into<String> + Display + Send,
        variable: impl Into<String> + Send,
    ) -> crate::errors::Result<crate::apis::asterisk::variables::models::Variable> {
        self.client
            .post_with_query(
                format!("/channels/{channel_id}/variable").as_str(),
                vec![] as Vec<String>,
                &[("variable", variable.into())],
            )
            .await
    }

    /// Set the value of a channel variable or function.
    pub async fn set_variable(
        &self,
        channel_id: impl Into<String> + Display + Send,
        variable: impl Into<String> + Send,
        value: impl Into<String> + Send,
    ) -> crate::errors::Result<crate::apis::asterisk::variables::models::Variable> {
        self.client
            .post_with_query(
                format!("/channels/{channel_id}/variable").as_str(),
                vec![] as Vec<String>,
                &[("variable", variable.into()), ("value", value.into())],
            )
            .await
    }

    /// Start snooping.
    ///
    /// *Implementation Notes*
    /// Snoop (spy/whisper) on a specific channel.
    pub async fn snoop(
        &self,
        request: params::SnoopRequest,
    ) -> crate::errors::Result<models::Channel> {
        self.client
            .post_with_query(
                format!("/channels/{}/snoop", request.channel_id).as_str(),
                vec![] as Vec<String>,
                &request,
            )
            .await
    }

    /// Start snooping with specific ID.
    ///
    /// *Implementation Notes*
    /// Snoop (spy/whisper) on a specific channel.
    pub async fn snoop_with_id(
        &self,
        request: params::SnoopWithIdRequest,
    ) -> crate::errors::Result<models::Channel> {
        self.client
            .post_with_query(
                format!(
                    "/channels/{}/snoop/{}",
                    request.channel_id, request.snoop_id
                )
                .as_str(),
                vec![] as Vec<String>,
                &request,
            )
            .await
    }

    /// Dial a created channel.
    pub async fn dial(&self, request: params::DialRequest) -> crate::errors::Result<()> {
        self.client
            .post_with_query(
                format!("/channels/{}/dial", request.channel_id).as_str(),
                vec![] as Vec<String>,
                &request,
            )
            .await
    }

    /// RTP stats on a channel.
    pub async fn rtp_statistics(
        &self,
        channel_id: impl Into<String> + Display + Send,
    ) -> crate::errors::Result<models::RTPStatistics> {
        self.client
            .get(format!("/channels/{channel_id}/rtp_statistics").as_str())
            .await
    }
}
