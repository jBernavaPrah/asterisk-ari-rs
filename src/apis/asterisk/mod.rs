pub mod models;

pub mod dynamic_config;
pub mod logging;
pub mod modules;
pub mod variables;

use crate::apis::client::Client;
use crate::errors::Result;

pub struct Asterisk<'c> {
    client: &'c Client,
}

impl<'c> Asterisk<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }
}

impl Asterisk<'_> {
    pub async fn info(&self) -> Result<models::AsteriskInfo> {
        self.client.get("/asterisk/info").await
    }

    pub async fn ping(&self) -> Result<models::AsteriskPing> {
        self.client.get("/asterisk/ping").await
    }

    pub fn config(&self) -> dynamic_config::DynamicConfiguration {
        dynamic_config::DynamicConfiguration::new(self.client)
    }

    pub fn modules(&self) -> modules::Modules {
        modules::Modules::new(self.client)
    }

    pub fn logging(&self) -> logging::Logging {
        logging::Logging::new(self.client)
    }

    pub fn variables(&self) -> variables::Variables {
        variables::Variables::new(self.client)
    }
}
