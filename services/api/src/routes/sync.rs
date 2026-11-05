use axum::{
    extract::{Path, State},
    routing::post,
    Json, Router,
};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{routes::ApiError, AppState};

pub fn router() -> Router<AppState> {
    Router::new().route("/projects/{project_id}/sync", post(trigger_sync))
}

async fn trigger_sync(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    let job_id = sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO jobs (project_id, type, status, payload)
         VALUES ($1, 'connector.sync', 'queued', '{\"provider\":\"mock-revizto\"}'::jsonb)
         RETURNING id",
    )
    .bind(project_id)
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(json!({
        "jobId": job_id,
        "status": "queued"
    })))
}
