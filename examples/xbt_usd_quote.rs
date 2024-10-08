use anyhow::Result;
use bitmex_stream::Network;
use futures::TryStreamExt;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info,bitmex_stream=trace")
        .init();

    let mut stream = bitmex_stream::subscribe(
        ["quoteBin1m:XBTUSD".to_owned()],
        Network::Mainnet,
        Duration::from_secs(10),
    );

    while let Some(result) = stream.try_next().await? {
        tracing::info!("{result}");
    }

    Ok(())
}
