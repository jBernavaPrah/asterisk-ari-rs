pub mod models;

use crate::apis::client::Client;
use serde::Serialize;

pub struct Mailboxes<'c> {
    client: &'c Client,
}

impl<'c> Mailboxes<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }
}

impl<'c> Mailboxes<'c> {
    /// List all mailboxes.
    pub async fn list(&self) -> crate::errors::Result<Vec<models::Mailbox>> {
        self.client.get("/mailboxes").await
    }

    /// Retrieve the current state of a device
    pub async fn get(
        &self,
        mailbox_name: impl Into<String> + Send,
    ) -> crate::errors::Result<models::Mailbox> {
        self.client
            .get(format!("/mailboxes/{}", mailbox_name.into()).as_str())
            .await
    }

    /// Change the state of a mailbox.
    ///
    /// Note - implicitly creates the mailbox.
    pub async fn change(
        &self,
        mailbox_name: impl Into<String> + Send,
        old_messages: u32,
        new_messages: u32,
    ) -> crate::errors::Result<()> {
        self.client
            .put_with_query(
                format!("/mailboxes/{}", mailbox_name.into()).as_str(),
                vec![] as Vec<String>,
                &[("oldMessages", old_messages), ("newMessages", new_messages)],
            )
            .await
    }

    /// Destroy a mailbox.
    pub async fn destroy(
        &self,
        mailbox_name: impl Into<String> + Send,
    ) -> crate::errors::Result<()> {
        self.client
            .delete(format!("/mailboxes/{}", mailbox_name.into()).as_str())
            .await
    }
}
