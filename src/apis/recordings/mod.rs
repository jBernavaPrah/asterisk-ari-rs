use crate::apis::client::Client;

pub mod models;

pub struct Recordings<'c> {
    client: &'c Client,
}

impl<'c> Recordings<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    pub fn stored(&self) -> StoredRecordings {
        StoredRecordings::new(self.client)
    }

    pub fn live(&self) -> LiveRecordings {
        LiveRecordings::new(self.client)
    }
}

pub struct StoredRecordings<'c> {
    client: &'c Client,
}

impl<'c> StoredRecordings<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }
}

impl StoredRecordings<'_> {
    /// List recordings that are complete.
    pub async fn list(&self) -> crate::errors::Result<Vec<models::StoredRecording>> {
        self.client.get("/recordings/stored").await
    }

    /// Get a stored recording's details.
    pub async fn get(
        &self,
        recording_name: impl Into<String> + Send,
    ) -> crate::errors::Result<models::StoredRecording> {
        self.client
            .get(format!("/recordings/stored/{}", recording_name.into()).as_str())
            .await
    }

    pub async fn delete(
        &self,
        recording_name: impl Into<String> + Send,
    ) -> crate::errors::Result<()> {
        self.client
            .delete(format!("/recordings/stored/{}", recording_name.into()).as_str())
            .await
    }

    /// Get the file associated with the stored recording.
    pub async fn file(
        &self,
        _recording_name: impl Into<String> + Send,
    ) -> crate::errors::Result<Vec<u8>> {
        unimplemented!("This function is not implemented yet"); // todo implement first teh get_raw, so will return the raw bytes
                                                                // self.client
                                                                //     .get(format!("/recordings/stored/{}/file", recording_name.into()).as_str())
                                                                //     .await
    }

    /// Copy a stored recording.
    pub async fn copy(
        &self,
        recording_name: impl Into<String> + Send,
        destination: impl Into<String> + Send,
    ) -> crate::errors::Result<models::StoredRecording> {
        self.client
            .post_with_query(
                format!("/recordings/stored/{}/copy", recording_name.into()).as_str(),
                vec![] as Vec<String>,
                &[("destination", destination.into())],
            )
            .await
    }
}

pub struct LiveRecordings<'c> {
    client: &'c Client,
}

impl<'c> LiveRecordings<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }
}

impl LiveRecordings<'_> {
    /// List live recordings
    pub async fn list(&self) -> crate::errors::Result<Vec<models::LiveRecording>> {
        self.client.get("/recordings/live").await
    }

    /// Get a live recording's details.
    pub async fn get(
        &self,
        recording_name: impl Into<String> + Send,
    ) -> crate::errors::Result<models::LiveRecording> {
        self.client
            .get(format!("/recordings/live/{}", recording_name.into()).as_str())
            .await
    }

    /// Stop a live recording and discard it.
    pub async fn discard(
        &self,
        recording_name: impl Into<String> + Send,
    ) -> crate::errors::Result<()> {
        self.client
            .delete(format!("/recordings/live/{}", recording_name.into()).as_str())
            .await
    }

    /// Stop a live recording and store it.
    pub async fn stop(
        &self,
        recording_name: impl Into<String> + Send,
    ) -> crate::errors::Result<()> {
        self.client
            .post(
                format!("/recordings/live/{}/stop", recording_name.into()).as_str(),
                vec![] as Vec<String>,
            )
            .await
    }

    /// Pause a live recording.
    ///
    /// Implementation Notes
    /// Pausing a recording suspends silence detection, which will be restarted when the recording is unpaused. Paused time is not included in the accounting for maxDurationSeconds.
    pub async fn pause(
        &self,
        recording_name: impl Into<String> + Send,
    ) -> crate::errors::Result<()> {
        self.client
            .post(
                format!("/recordings/live/{}/pause", recording_name.into()).as_str(),
                vec![] as Vec<String>,
            )
            .await
    }

    /// Unpause a live recording.
    pub async fn unpause(
        &self,
        recording_name: impl Into<String> + Send,
    ) -> crate::errors::Result<()> {
        self.client
            .delete(format!("/recordings/live/{}/pause", recording_name.into()).as_str())
            .await
    }

    /// Mute a live recording.
    ///
    /// Implementation Notes
    /// Muting a recording suspends silence detection, which will be restarted when the recording is unmuted.
    pub async fn mute(
        &self,
        recording_name: impl Into<String> + Send,
    ) -> crate::errors::Result<()> {
        self.client
            .post(
                format!("/recordings/live/{}/mute", recording_name.into()).as_str(),
                vec![] as Vec<String>,
            )
            .await
    }

    /// Unmute a live recording.
    pub async fn unmute(
        &self,
        recording_name: impl Into<String> + Send,
    ) -> crate::errors::Result<()> {
        self.client
            .delete(format!("/recordings/live/{}/mute", recording_name.into()).as_str())
            .await
    }
}
