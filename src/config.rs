/// Configuration for the ARI client.
///
/// This struct holds the necessary information to configure the ARI client,
/// including the API base URL, username, and password.
#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    /// The base URL for the ARI API.
    pub(crate) api_base: String,
    /// The username for authentication with the ARI API.
    pub(crate) username: String,
    /// The password for authentication with the ARI API.
    pub(crate) password: String,
}

impl Default for Config {
    /// Provides a default configuration for the ARI client.
    ///
    /// # Returns
    ///
    /// A `Config` instance with default values.
    fn default() -> Self {
        Config {
            api_base: "http://localhost:8088/ari".to_string(),
            username: "".to_string(),
            password: "".to_string(),
        }
    }
}

impl Config {
    /// Creates a new `Config` instance with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `api_base` - The base URL for the ARI API.
    /// * `username` - The username for authentication with the ARI API.
    /// * `password` - The password for authentication with the ARI API.
    ///
    /// # Returns
    ///
    /// A new `Config` instance with the provided values.
    pub fn new(
        api_base: impl Into<String>,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        Config {
            api_base: api_base.into(),
            username: username.into(),
            password: password.into(),
        }
    }
}
