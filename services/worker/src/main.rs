use std::time::Duration;

use anyhow::Context;
use serde_json::json;
use sqlx::{FromRow, PgPool};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

#[derive(Debug, FromRow)]
struct Job {
    id: Uuid,
    project_id: Option<Uuid>,
    job_type: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "terrassay_worker=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://terrassay:terrassay_dev@localhost:5432/terrassay".to_string());
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(4)
        .connect(&database_url)
        .await
        .context("connect worker to Postgres")?;

    tracing::info!("terrassay worker started");

    loop {
        match claim_job(&pool).await {
            Ok(Some(job)) => run_job(&pool, job).await,
            Ok(None) => tokio::time::sleep(Duration::from_secs(2)).await,
            Err(error) => {
                tracing::error!(?error, "job claim failed");
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    }
}

async fn claim_job(pool: &PgPool) -> anyhow::Result<Option<Job>> {
    let job = sqlx::query_as::<_, Job>(
        "UPDATE jobs
         SET status = 'running', progress = 10, updated_at = now()
         WHERE id = (
           SELECT id FROM jobs
           WHERE status = 'queued'
           ORDER BY created_at ASC
           FOR UPDATE SKIP LOCKED
           LIMIT 1
         )
         RETURNING id, project_id, type AS job_type",
    )
    .fetch_optional(pool)
    .await?;

    Ok(job)
}

async fn run_job(pool: &PgPool, job: Job) {
    tracing::info!(job_id = %job.id, job_type = %job.job_type, "job started");

    let result = match job.job_type.as_str() {
        "connector.sync" => run_mock_sync(pool, job.project_id).await,
        other => Err(anyhow::anyhow!("unsupported job type: {other}")),
    };

    match result {
        Ok(payload) => {
            let _ = sqlx::query(
                "UPDATE jobs
                 SET status = 'succeeded', progress = 100, payload = $2, updated_at = now()
                 WHERE id = $1",
            )
            .bind(job.id)
            .bind(payload)
            .execute(pool)
            .await;
            tracing::info!(job_id = %job.id, "job succeeded");
        }
        Err(error) => {
            let _ = sqlx::query(
                "UPDATE jobs
                 SET status = 'failed', progress = 100, error = $2, updated_at = now()
                 WHERE id = $1",
            )
            .bind(job.id)
            .bind(json!({ "message": error.to_string() }))
            .execute(pool)
            .await;
            tracing::error!(job_id = %job.id, ?error, "job failed");
        }
    }
}

async fn run_mock_sync(pool: &PgPool, project_id: Option<Uuid>) -> anyhow::Result<serde_json::Value> {
    let project_id = project_id.context("sync job has no project")?;

    // Phase 1 simulates the remote round-trip; Phase 2 replaces this with the
    // Revizto adapter and real pagination/rate limits.
    tokio::time::sleep(Duration::from_millis(800)).await;

    let issues = sqlx::query_scalar::<_, i64>(
        "SELECT count(*)::bigint FROM issues WHERE project_id = $1",
    )
    .bind(project_id)
    .fetch_one(pool)
    .await?;
    let clashes = sqlx::query_scalar::<_, i64>(
        "SELECT count(*)::bigint FROM clashes WHERE project_id = $1",
    )
    .bind(project_id)
    .fetch_one(pool)
    .await?;
    let sheets = sqlx::query_scalar::<_, i64>(
        "SELECT count(*)::bigint FROM sheets WHERE project_id = $1",
    )
    .bind(project_id)
    .fetch_one(pool)
    .await?;

    sqlx::query("UPDATE projects SET last_synced_at = now(), updated_at = now() WHERE id = $1")
        .bind(project_id)
        .execute(pool)
        .await?;

    Ok(json!({
        "provider": "mock-revizto",
        "issues": issues,
        "clashes": clashes,
        "sheets": sheets
    }))
}
