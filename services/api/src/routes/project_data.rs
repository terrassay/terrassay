use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::{json, Value};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{routes::ApiError, AppState};

#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
struct LayerRow {
    id: Uuid,
    name: String,
    layer_type: String,
    visible: bool,
    opacity: f64,
    layer_key: Option<String>,
    licence: Option<String>,
    attribution: Option<String>,
}

#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
struct LocalizationRow {
    method: String,
    source_crs: String,
    target_crs: String,
    transform: Value,
    accuracy_m: Option<f64>,
    confidence: String,
    verified: bool,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
struct FeatureRow {
    properties: Value,
    geometry: Value,
}

#[derive(Debug, FromRow)]
struct IssueGeoRow {
    id: Uuid,
    external_id: String,
    title: String,
    priority: String,
    status: String,
    lon: f64,
    lat: f64,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/projects/{project_id}/layers", get(list_layers))
        .route("/projects/{project_id}/localization", get(get_localization))
        .route(
            "/projects/{project_id}/features/{layer_key}",
            get(layer_geojson),
        )
        .route("/projects/{project_id}/issues.geojson", get(issues_geojson))
}

async fn list_layers(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
) -> Result<Json<Vec<LayerRow>>, ApiError> {
    let layers = sqlx::query_as::<_, LayerRow>(
        "SELECT id, name, type AS layer_type, visible, opacity, layer_key, licence, attribution
         FROM layers
         WHERE project_id = $1
         ORDER BY z_index ASC, name ASC",
    )
    .bind(project_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(layers))
}

async fn get_localization(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
) -> Result<Json<Option<LocalizationRow>>, ApiError> {
    let localization = sqlx::query_as::<_, LocalizationRow>(
        "SELECT method, source_crs, target_crs, transform, accuracy_m, confidence, verified, updated_at
         FROM project_localizations
         WHERE project_id = $1",
    )
    .bind(project_id)
    .fetch_optional(&state.pool)
    .await?;

    Ok(Json(localization))
}

async fn layer_geojson(
    State(state): State<AppState>,
    Path((project_id, layer_key)): Path<(Uuid, String)>,
) -> Result<Json<Value>, ApiError> {
    let features = sqlx::query_as::<_, FeatureRow>(
        "SELECT properties, ST_AsGeoJSON(geometry)::json AS geometry
         FROM project_features
         WHERE project_id = $1 AND layer_key = $2
         ORDER BY id ASC",
    )
    .bind(project_id)
    .bind(&layer_key)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(feature_collection(
        features
            .into_iter()
            .map(|feature| {
                json!({
                    "type": "Feature",
                    "geometry": feature.geometry,
                    "properties": feature.properties
                })
            })
            .collect(),
    )))
}

async fn issues_geojson(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    let issues = sqlx::query_as::<_, IssueGeoRow>(
        "SELECT id, external_id, title, priority, status, ST_X(location) AS lon, ST_Y(location) AS lat
         FROM issues
         WHERE project_id = $1 AND location IS NOT NULL
         ORDER BY external_id ASC",
    )
    .bind(project_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(feature_collection(
        issues
            .into_iter()
            .map(|issue| {
                json!({
                    "type": "Feature",
                    "id": issue.id,
                    "geometry": { "type": "Point", "coordinates": [issue.lon, issue.lat] },
                    "properties": {
                        "id": issue.id,
                        "externalId": issue.external_id,
                        "title": issue.title,
                        "priority": issue.priority,
                        "status": issue.status
                    }
                })
            })
            .collect(),
    )))
}

fn feature_collection(features: Vec<Value>) -> Value {
    json!({ "type": "FeatureCollection", "features": features })
}
