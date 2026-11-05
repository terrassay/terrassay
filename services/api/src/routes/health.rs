use axum::{extract::State, http::StatusCode, Json};
use serde_json::json;

use crate::AppState;

pub async fn healthz() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok", "service": "terrassay-api" }))
}

pub async fn readyz(State(state): State<AppState>) -> (StatusCode, Json<serde_json::Value>) {
    match sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&state.pool)
        .await
    {
        Ok(_) => (StatusCode::OK, Json(json!({ "status": "ready" }))),
        Err(error) => {
            tracing::warn!(?error, "database readiness check failed");
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({ "status": "not_ready" })),
            )
        }
    }
}
