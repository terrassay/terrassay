use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{routes::ApiError, AppState};

#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProjectRow {
    id: Uuid,
    name: String,
    code: Option<String>,
    status: String,
    analysis_crs: String,
    created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
struct SummaryRow {
    issue_count: i64,
    critical_count: i64,
    clash_count: i64,
    sheet_count: i64,
    last_synced_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
struct IssueRow {
    id: Uuid,
    external_id: String,
    title: String,
    status: String,
    priority: String,
    discipline: Option<String>,
    assignee_name: Option<String>,
    lon: Option<f64>,
    lat: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct IssueQuery {
    limit: Option<i64>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/projects", get(list_projects))
        .route("/projects/{project_id}/summary", get(project_summary))
        .route("/projects/{project_id}/issues", get(list_issues))
}

async fn list_projects(State(state): State<AppState>) -> Result<Json<Vec<ProjectRow>>, ApiError> {
    let projects = sqlx::query_as::<_, ProjectRow>(
        "SELECT id, name, code, status, analysis_crs, created_at
         FROM projects
         ORDER BY created_at ASC",
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(projects))
}

async fn project_summary(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
) -> Result<Json<SummaryRow>, ApiError> {
    let summary = sqlx::query_as::<_, SummaryRow>(
        "SELECT
           (SELECT count(*) FROM issues WHERE project_id = $1) AS issue_count,
           (SELECT count(*) FROM issues WHERE project_id = $1 AND priority = 'critical') AS critical_count,
           (SELECT count(*) FROM clashes WHERE project_id = $1) AS clash_count,
           (SELECT count(*) FROM sheets WHERE project_id = $1) AS sheet_count,
           (SELECT last_synced_at FROM projects WHERE id = $1) AS last_synced_at",
    )
    .bind(project_id)
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(summary))
}

async fn list_issues(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
    Query(query): Query<IssueQuery>,
) -> Result<Json<Vec<IssueRow>>, ApiError> {
    let limit = query.limit.unwrap_or(25).clamp(1, 100);
    let issues = sqlx::query_as::<_, IssueRow>(
        "SELECT id, external_id, title, status, priority, discipline, assignee_name,
                ST_X(location) AS lon, ST_Y(location) AS lat
         FROM issues
         WHERE project_id = $1
         ORDER BY
           CASE priority WHEN 'critical' THEN 0 WHEN 'high' THEN 1 WHEN 'normal' THEN 2 ELSE 3 END,
           external_id ASC
         LIMIT $2",
    )
    .bind(project_id)
    .bind(limit)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(issues))
}
