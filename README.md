# Asterisk ARI Client

[![CI](https://github.com/jbernavaprah/asterisk-ari-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/jbernavaprah/asterisk-ari-rs/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/License-Apache-blue.svg)](LICENSE-APACHE)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE-MIT)
[![Version](https://img.shields.io/crates/v/asterisk-ari-rs)](https://crates.io/crates/asterisk-ari-rs)
[![Docs](https://docs.rs/asterisk-ari-rs/badge.svg)](https://docs.rs/asterisk-ari-rs)

A simple yet powerful library for managing the [Asterisk](https://www.asterisk.org/) ARI (Asterisk REST Interface). This library implements all Asterisk REST APIs and WebSocket events documented in the [ARI API Documentation](https://docs.asterisk.org/Latest_API/API_Documentation/Asterisk_REST_Interface/).

## Features

- Full implementation of Asterisk REST APIs.
- WebSocket support for handling ARI events.
- Designed for simplicity and ease of use.
- Suitable for building custom telephony applications and integrations.

## Installation

To use this library, add the following line to your `Cargo.toml` file:

```toml
[dependencies]
asterisk-ari-rs = "x.y.z" # Replace x.y.z with the latest version
```

## Usage

Here's a basic example of how to use the library:

```rust
use asterisk_ari_rs::apis::channels;
use asterisk_ari_rs::AriClient;
use asterisk_ari_rs::Config;
use asterisk_ari_rs::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let config = Config::new("http://localhost:8088", "asterisk", "asterisk");

    let mut client = AriClient::with_config(config);

    client.on_stasis_start(|client, event| async move {
        println!("Handling StasisStart event: {:?}", event);

        client
            .channels()
            .answer(&event.data.channel.id)
            .await
            .unwrap();

        client
            .channels()
            .play(channels::params::PlayRequest::new(
                &event.data.channel.id,
                "sound:tt-monkeys",
            ))
            .await
            .unwrap();
    });

    info!("Applications: {:?}", client.applications().list().await?);
    info!("Ping: {:?}", client.asterisk().ping().await?);
    info!("Info: {:?}", client.asterisk().info().await?);

    let _client = client.clone();
    tokio::spawn(async move {
        _client.start("my-application".to_string()).await.unwrap();
    });

    tokio::time::sleep(std::time::Duration::from_secs(30)).await;

    info!("Stopping client");
    client.stop();

    info!("Await client to stop");
    tokio::time::sleep(std::time::Duration::from_secs(4)).await;

    Ok(())
}

```

For detailed usage and API documentation, visit the [docs](https://docs.rs/asterisk-ari-rs).

## Configuration

Ensure that your Asterisk instance is configured to enable ARI. Update your `ari.conf` file with the appropriate settings:

```ini
[general]
enabled = yes
pretty = yes ; not mandatory.

[asterisk]
type = user
read_only = no
password = asterisk
```

## Contributing

Contributions are welcome! If you'd like to contribute to this project, please follow these steps:

1. Fork the repository.
2. Create a new branch: `git checkout -b my-feature-branch`.
3. Make your changes and commit them: `git commit -m 'Add some feature'`.
4. Push to the branch: `git push origin my-feature-branch`.
5. Submit a pull request.

Before submitting, ensure your code follows the project’s coding standards and passes all tests.

## Development

### Running Tests

To run the tests, use:

```bash
cargo test
```

### Linting

Ensure your code adheres to the Rust style guide by running:

```bash
cargo fmt -- --check
```

## Roadmap

- [ ] Add more examples and documentation.

## Issues and Feedback

If you encounter any issues, please [create a new issue](https://github.com/jbernavaprah/asterisk-ari-rs/issues). Feedback and feature requests are always appreciated!

## License

This project is licensed under either of the following licenses, at your option:

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

For more information, see the [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) files.

## Acknowledgments

Special thanks to the [Asterisk](https://www.asterisk.org/) community for creating such a powerful telephony platform.

