mod routes;

use anyhow::Context;
use axum::{routing::get, Router};
use sqlx::PgPool;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "terrassay_api=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://terrassay:terrassay_dev@localhost:5432/terrassay".to_string());
    let bind = std::env::var("API_BIND").unwrap_or_else(|_| "0.0.0.0:8080".to_string());

    let pool = terrassay_db::connect(&database_url)
        .await
        .context("connect to Postgres")?;

    let app = Router::new()
        .route("/healthz", get(routes::health::healthz))
        .route("/readyz", get(routes::health::readyz))
        .nest("/api/v1", routes::projects::router())
        .nest("/api/v1", routes::project_data::router())
        .nest("/api/v1", routes::assistant::router())
        .nest("/api/v1", routes::sync::router())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(AppState { pool });

    let listener = tokio::net::TcpListener::bind(&bind)
        .await
        .with_context(|| format!("bind {bind}"))?;

    tracing::info!(%bind, "terrassay api listening");
    axum::serve(listener, app).await.context("serve api")?;
    Ok(())
}
