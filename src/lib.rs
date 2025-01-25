//! Asterisk ARI Client
//!
//! This crate provides a simple yet powerful Rust library for managing the
//! [Asterisk](https://www.asterisk.org/) ARI (Asterisk REST Interface). It offers
//! full implementation of Asterisk's REST APIs and WebSocket event handling,
//! enabling developers to build custom telephony applications with ease.
//!
//! ## Features
//!
//! - Comprehensive coverage of Asterisk REST APIs.
//! - WebSocket support for real-time ARI event streaming.
//! - Designed for simplicity and ease of use.
//! - Ideal for telephony application development and integration.
//!
//! ## Quick Start
//!
//! To use this crate, add it to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! asterisk-ari-rs = "x.y.z" // Replace with the latest version
//! ```
//!
//! ### Example
//!
//! ```rust
//! use asterisk_ari_rs::apis::channels;
//! use asterisk_ari_rs::AriClient;
//! use asterisk_ari_rs::Config;
//! use asterisk_ari_rs::Result;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!
//!     let config = Config::new("http://localhost:8088", "asterisk", "asterisk");
//!
//!     let mut client = AriClient::with_config(config);
//!
//!     client.on_stasis_start(|client, event| async move {
//!         println!("Handling StasisStart event: {:?}", event);
//!
//!         client
//!             .channels()
//!             .answer(&event.data.channel.id)
//!             .await
//!             .unwrap();
//!
//!         client
//!             .channels()
//!             .play(channels::params::PlayRequest::new(
//!                 &event.data.channel.id,
//!                 "sound:tt-monkeys",
//!             ))
//!             .await
//!             .unwrap();
//!     });
//!
//!     println!("Applications: {:?}", client.applications().list().await?);
//!     println!("Ping: {:?}", client.asterisk().ping().await?);
//!     println!("Info: {:?}", client.asterisk().info().await?);
//!
//!     let _client = client.clone();
//!     tokio::spawn(async move {
//!         _client.start("my-application".to_string()).await.unwrap();
//!     });
//!
//!     tokio::time::sleep(std::time::Duration::from_secs(30)).await;
//!
//!     println!("Stopping client");
//!     client.stop();
//!
//!     println!("Await client to stop");
//!     tokio::time::sleep(std::time::Duration::from_secs(4)).await;
//!
//!     Ok(())
//! }
//! ```
//!
//!
//! ## License
//!
//! This crate is dual-licensed under the [Apache License 2.0](LICENSE-APACHE) or
//! [MIT License](LICENSE-MIT). Choose the license that suits your project.

/// Apis implementation
pub mod apis;
mod config;
pub use config::*;
mod errors;
pub use errors::*;
mod client;
pub use client::*;
/// WebSocket implementation
pub mod ws;
