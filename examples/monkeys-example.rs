use asterisk_ari::apis::channels;
use asterisk_ari::AriClient;
use asterisk_ari::Config;
use asterisk_ari::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Active the tracing subscriber with DEBUG Level
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let config = Config::new("http://localhost:8088", "asterisk", "asterisk");

    let mut client = AriClient::with_config(config.clone());

    client.on_stasis_start(|client, event| async move {
        println!("Handling StasisStart event: {:?}", event);

        client.channels().answer(&event.data.channel.id).await?;

        client
            .channels()
            .play(channels::params::PlayRequest::new(
                &event.data.channel.id,
                "sound:tt-monkeys",
            ))
            .await?;

        Ok(())
    });

    info!("Applications: {:?}", client.applications().list().await?);
    info!("Ping: {:?}", client.asterisk().ping().await?);
    info!("Info: {:?}", client.asterisk().info().await?);

    client.start("my-ast-app".to_string()).await?;

    // simulate a long-running process...
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    info!("Stopping ws");
    client.stop().await?;

    Ok(())
}
