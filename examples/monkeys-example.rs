use asterisk_ari_rs::apis::channels;
use asterisk_ari_rs::AriClient;
use asterisk_ari_rs::Config;
use asterisk_ari_rs::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Active the tracing subscriber with DEBUG Level
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
        _client.start("my-ast-app".to_string()).await.unwrap();
    });

    tokio::time::sleep(std::time::Duration::from_secs(30)).await;

    info!("Stopping client");

    client.stop();

    info!("Await client to stop");
    tokio::time::sleep(std::time::Duration::from_secs(4)).await;

    Ok(())
}
