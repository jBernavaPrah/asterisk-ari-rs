use crate::config::Config;
use crate::errors::AriError;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Display;

/// Represents the ARI client.
///
/// This struct holds the configuration and HTTP client for making requests to the ARI API.
#[derive(Clone, Debug)]
pub struct Client {
    /// Configuration for the ARI client.
    pub(crate) config: Config,
    /// HTTP client for making requests.
    pub(crate) client: reqwest::Client,
}

impl Client {
    /// Creates a new client with the given configuration and HTTP client.
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration for the ARI client.
    /// * `client` - The HTTP client for making requests.
    ///
    /// # Returns
    ///
    /// A new instance of `Client`.
    pub fn build(config: Config, client: reqwest::Client) -> Self {
        Client { config, client }
    }

    /// Creates a new client with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration for the ARI client.
    ///
    /// # Returns
    ///
    /// A new instance of `Client`.
    pub fn with_config(config: Config) -> Self {
        Client {
            config,
            client: reqwest::Client::builder()
                .connect_timeout(std::time::Duration::from_secs(5))
                .http2_keep_alive_interval(Some(std::time::Duration::from_secs(5)))
                .http2_keep_alive_while_idle(true)
                .build()
                .unwrap(),
        }
    }

    /// Sets the HTTP client for making requests.
    ///
    /// # Arguments
    ///
    /// * `client` - The HTTP client to use.
    ///
    /// # Returns
    ///
    /// The updated `Client` instance.
    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = client;
        self
    }

    /// Returns an instance of the `Applications` API.
    pub fn applications(&self) -> crate::apis::applications::Applications {
        crate::apis::applications::Applications::new(self)
    }

    /// Returns an instance of the `Asterisk` API.
    pub fn asterisk(&self) -> crate::apis::asterisk::Asterisk {
        crate::apis::asterisk::Asterisk::new(self)
    }

    /// Returns an instance of the `Endpoints` API.
    pub fn endpoints(&self) -> crate::apis::endpoints::Endpoints {
        crate::apis::endpoints::Endpoints::new(self)
    }

    /// Returns an instance of the `Channels` API.
    pub fn channels(&self) -> crate::apis::channels::Channels {
        crate::apis::channels::Channels::new(self)
    }

    /// Returns an instance of the `Bridges` API.
    pub fn bridges(&self) -> crate::apis::bridges::Bridges {
        crate::apis::bridges::Bridges::new(self)
    }

    /// Returns an instance of the `Recordings` API.
    pub fn recordings(&self) -> crate::apis::recordings::Recordings {
        crate::apis::recordings::Recordings::new(self)
    }

    /// Returns an instance of the `Sounds` API.
    pub fn sounds(&self) -> crate::apis::sounds::Sounds {
        crate::apis::sounds::Sounds::new(self)
    }

    /// Returns an instance of the `Playbacks` API.
    pub fn playbacks(&self) -> crate::apis::playbacks::Playbacks {
        crate::apis::playbacks::Playbacks::new(self)
    }

    /// Returns an instance of the `DeviceStats` API.
    pub fn device_stats(&self) -> crate::apis::device_stats::DeviceStats {
        crate::apis::device_stats::DeviceStats::new(self)
    }

    /// Returns an instance of the `Mailboxes` API.
    pub fn mailboxes(&self) -> crate::apis::mailboxes::Mailboxes {
        crate::apis::mailboxes::Mailboxes::new(self)
    }

    /// Returns an instance of the `Events` API.
    pub fn events(&self) -> crate::apis::events::Events {
        crate::apis::events::Events::new(self)
    }

    /// Makes a GET request to the specified path and deserializes the response body.
    ///
    /// # Arguments
    ///
    /// * `path` - The path for the GET request.
    ///
    /// # Returns
    ///
    /// A `Result` containing the deserialized response body or an `AriError`.
    pub(crate) async fn get<O>(&self, path: &str) -> Result<O, AriError>
    where
        O: DeserializeOwned,
    {
        let request_maker = || async {
            Ok(self
                .client
                .get(self.url(path))
                .headers(self.headers())
                .build()?)
        };

        self.execute(request_maker).await
    }

    /// Makes a GET request to the specified path with the given query and deserializes the response body.
    ///
    /// # Arguments
    ///
    /// * `path` - The path for the GET request.
    /// * `query` - The query parameters for the GET request.
    ///
    /// # Returns
    ///
    /// A `Result` containing the deserialized response body or an `AriError`.
    pub(crate) async fn get_with_query<Q, O>(&self, path: &str, query: &Q) -> Result<O, AriError>
    where
        O: DeserializeOwned,
        Q: Serialize + ?Sized,
    {
        let request_maker = || async {
            Ok(self
                .client
                .get(self.url(path))
                .query(query)
                .headers(self.headers())
                .build()?)
        };

        self.execute(request_maker).await
    }

    /// Makes a DELETE request to the specified path and deserializes the response body.
    ///
    /// # Arguments
    ///
    /// * `path` - The path for the DELETE request.
    ///
    /// # Returns
    ///
    /// A `Result` containing the deserialized response body or an `AriError`.
    pub(crate) async fn delete<O>(&self, path: &str) -> Result<O, AriError>
    where
        O: DeserializeOwned,
    {
        let request_maker = || async {
            Ok(self
                .client
                .delete(self.url(path))
                .headers(self.headers())
                .build()?)
        };

        self.execute(request_maker).await
    }

    /// Makes a DELETE request to the specified path with the given query and deserializes the response body.
    ///
    /// # Arguments
    ///
    /// * `path` - The path for the DELETE request.
    /// * `query` - The query parameters for the DELETE request.
    ///
    /// # Returns
    ///
    /// A `Result` containing the deserialized response body or an `AriError`.
    pub(crate) async fn delete_with_query<O, Q>(&self, path: &str, query: &Q) -> Result<O, AriError>
    where
        O: DeserializeOwned,
        Q: Serialize + ?Sized,
    {
        let request_maker = || async {
            Ok(self
                .client
                .delete(self.url(path))
                .query(query)
                .headers(self.headers())
                .build()?)
        };

        self.execute(request_maker).await
    }

    /// Makes a POST request to the specified path with the given request body and deserializes the response body.
    ///
    /// # Arguments
    ///
    /// * `path` - The path for the POST request.
    /// * `request` - The request body for the POST request.
    ///
    /// # Returns
    ///
    /// A `Result` containing the deserialized response body or an `AriError`.
    pub(crate) async fn post<I, O>(&self, path: &str, request: I) -> Result<O, AriError>
    where
        I: Serialize,
        O: DeserializeOwned,
    {
        let request_maker = || async {
            Ok(self
                .client
                .post(self.url(path))
                .headers(self.headers())
                .json(&request)
                .build()?)
        };

        self.execute(request_maker).await
    }

    /// Makes a POST request to the specified path with the given request body and query parameters, and deserializes the response body.
    ///
    /// # Arguments
    ///
    /// * `path` - The path for the POST request.
    /// * `request` - The request body for the POST request.
    /// * `query` - The query parameters for the POST request.
    ///
    /// # Returns
    ///
    /// A `Result` containing the deserialized response body or an `AriError`.
    pub(crate) async fn post_with_query<I, Q, O>(
        &self,
        path: &str,
        request: I,
        query: &Q,
    ) -> Result<O, AriError>
    where
        I: Serialize,
        O: DeserializeOwned,
        Q: Serialize + ?Sized,
    {
        let request_maker = || async {
            Ok(self
                .client
                .post(self.url(path))
                .query(query)
                .json(&request)
                .headers(self.headers())
                .build()?)
        };

        self.execute(request_maker).await
    }

    /// Makes a PUT request to the specified path with the given request body and query parameters, and deserializes the response body.
    ///
    /// # Arguments
    ///
    /// * `path` - The path for the PUT request.
    /// * `request` - The request body for the PUT request.
    /// * `query` - The query parameters for the PUT request.
    ///
    /// # Returns
    ///
    /// A `Result` containing the deserialized response body or an `AriError`.
    pub(crate) async fn put_with_query<I, O, Q>(
        &self,
        path: &str,
        request: I,
        query: &Q,
    ) -> Result<O, AriError>
    where
        I: Serialize,
        O: DeserializeOwned,
        Q: Serialize + ?Sized,
    {
        let request_maker = || async {
            Ok(self
                .client
                .put(self.url(path))
                .headers(self.headers())
                .query(query)
                .json(&request)
                .build()?)
        };

        self.execute(request_maker).await
    }

    /// Makes a PUT request to the specified path with the given request body and deserializes the response body.
    ///
    /// # Arguments
    ///
    /// * `path` - The path for the PUT request.
    /// * `request` - The request body for the PUT request.
    ///
    /// # Returns
    ///
    /// A `Result` containing the deserialized response body or an `AriError`.
    pub(crate) async fn put<I, O>(&self, path: &str, request: I) -> Result<O, AriError>
    where
        I: Serialize,
        O: DeserializeOwned,
    {
        let request_maker = || async {
            Ok(self
                .client
                .put(self.url(path))
                .headers(self.headers())
                .json(&request)
                .build()?)
        };

        self.execute(request_maker).await
    }

    /// Constructs the full URL for the given path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to append to the base URL.
    ///
    /// # Returns
    ///
    /// The full URL as a string.
    pub(crate) fn url(&self, path: impl Into<String> + Display) -> String {
        format!("{}/ari{}", self.config.api_base, path)
    }

    /// Constructs the headers for the HTTP requests.
    ///
    /// # Returns
    ///
    /// A `HeaderMap` containing the necessary headers.
    pub(crate) fn headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        headers.insert(
            reqwest::header::AUTHORIZATION,
            format!(
                "Basic {}",
                BASE64_STANDARD
                    .encode(format!("{}:{}", self.config.username, self.config.password))
            )
            .parse()
            .unwrap(),
        );
        headers
    }

    /// Executes an HTTP request and retries on rate limit.
    ///
    /// # Arguments
    ///
    /// * `request_maker` - A closure that creates the request.
    ///
    /// # Returns
    ///
    /// A `Result` containing the deserialized response body or an `AriError`.
    ///
    /// The `request_maker` serves one purpose: to be able to create the request again
    /// to retry the API call after getting rate limited. `request_maker` is async because
    /// `reqwest::multipart::Form` is created by async calls to read files for uploads.
    async fn execute<O, M, Fut>(&self, request_maker: M) -> Result<O, AriError>
    where
        O: DeserializeOwned,
        M: Fn() -> Fut,
        Fut: core::future::Future<Output = Result<reqwest::Request, AriError>>,
    {
        match self
            .client
            .execute(request_maker().await?)
            .await?
            .error_for_status()
        {
            Ok(resp) => {
                let body = resp.text().await?;
                if body.is_empty() {
                    return Ok(serde_json::from_str("null")?);
                }

                Ok(serde_json::from_str(&body)?)
            }
            Err(e) => Err(e.into()),
        }
    }
}
