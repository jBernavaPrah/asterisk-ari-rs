pub mod models;

use crate::apis::client::Client;
use serde::Serialize;

pub struct DeviceStats<'c> {
    client: &'c Client,
}

impl<'c> DeviceStats<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }
}

impl<'c> DeviceStats<'c> {
    pub async fn list(&self) -> crate::errors::Result<Vec<models::DeviceState>> {
        self.client.get("/deviceStates").await
    }

    /// Retrieve the current state of a device
    pub async fn get(
        &self,
        device_name: impl Into<String> + Send,
    ) -> crate::errors::Result<models::DeviceState> {
        self.client
            .get(format!("/deviceStates/{}", device_name.into()).as_str())
            .await
    }

    /// Change the state of a device controlled by ARI. (Note - implicitly creates the device state).
    pub async fn change(
        &self,
        device_name: impl Into<String> + Send,
        state: State,
    ) -> crate::errors::Result<()> {
        self.client
            .put_with_query(
                format!("/deviceStates/{}", device_name.into()).as_str(),
                vec![] as Vec<String>,
                &[("deviceState", state)],
            )
            .await
    }

    pub async fn destroy(
        &self,
        device_name: impl Into<String> + Send,
    ) -> crate::errors::Result<()> {
        self.client
            .delete(format!("/deviceStates/{}", device_name.into()).as_str())
            .await
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum State {
    #[serde(rename = "NOT_INUSE")]
    NotInuse,
    #[serde(rename = "INUSE")]
    Inuse,
    #[serde(rename = "BUSY")]
    Busy,
    #[serde(rename = "INVALID")]
    Invalid,
    #[serde(rename = "UNAVAILABLE")]
    Unavailable,
    #[serde(rename = "RINGING")]
    Ringing,
    #[serde(rename = "RINGINUSE")]
    Ringinuse,
    #[serde(rename = "ONHOLD")]
    OnHold,
}
