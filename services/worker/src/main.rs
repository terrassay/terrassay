use std::time::Duration;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "terrassay_worker=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("terrassay worker started; job queue wiring arrives in Phase 1");

    loop {
        tokio::time::sleep(Duration::from_secs(30)).await;
        tracing::debug!("worker heartbeat");
    }
}
