use axum::{
    extract::{Path, State},
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{routes::ApiError, AppState};

#[derive(Debug, Deserialize)]
struct AskRequest {
    question: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Citation {
    kind: String,
    id: String,
    label: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct AskResponse {
    answer: String,
    citations: Vec<Citation>,
    map_action: Option<Value>,
    mode: &'static str,
}

#[derive(Debug, FromRow)]
struct IssueCitation {
    id: Uuid,
    external_id: String,
    title: String,
}

pub fn router() -> Router<AppState> {
    Router::new().route("/projects/{project_id}/assistant/ask", post(ask))
}

async fn ask(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
    Json(request): Json<AskRequest>,
) -> Result<Json<AskResponse>, ApiError> {
    let question = request.question.to_lowercase();

    if question.contains("flood") && (question.contains("issue") || question.contains("critical")) {
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT count(*)::bigint
             FROM issues i
             JOIN project_features f
               ON f.project_id = i.project_id AND f.layer_key = 'flood'
             WHERE i.project_id = $1
               AND i.priority = 'critical'
               AND i.status <> 'closed'
               AND ST_Intersects(i.location, f.geometry)",
        )
        .bind(project_id)
        .fetch_one(&state.pool)
        .await?;

        let issues = sqlx::query_as::<_, IssueCitation>(
            "SELECT i.id, i.external_id, i.title
             FROM issues i
             JOIN project_features f
               ON f.project_id = i.project_id AND f.layer_key = 'flood'
             WHERE i.project_id = $1
               AND i.priority = 'critical'
               AND i.status <> 'closed'
               AND ST_Intersects(i.location, f.geometry)
             ORDER BY i.external_id ASC
             LIMIT 20",
        )
        .bind(project_id)
        .fetch_all(&state.pool)
        .await?;

        let citations = issues
            .iter()
            .map(|issue| Citation {
                kind: "issue".to_string(),
                id: issue.id.to_string(),
                label: format!("{} — {}", issue.external_id, issue.title),
            })
            .chain(std::iter::once(Citation {
                kind: "layer".to_string(),
                id: "flood".to_string(),
                label: "Flood hazard zone".to_string(),
            }))
            .collect::<Vec<_>>();

        let answer = if count == 0 {
            "No critical open issues intersect the flood hazard zone.".to_string()
        } else {
            format!(
                "{count} critical open issue{} intersect the flood hazard zone. The result is computed with a PostGIS spatial join; this Phase 1 assistant is deterministic and cites the issue records below.",
                if count == 1 { "" } else { "s" }
            )
        };

        return Ok(Json(AskResponse {
            answer,
            citations,
            map_action: Some(json!({
                "type": "highlight",
                "layerId": "issues",
                "featureIds": issues.iter().map(|issue| issue.id).collect::<Vec<_>>()
            })),
            mode: "deterministic-tools",
        }));
    }

    if question.contains("summary") || question.contains("status") || question.contains("stand-up") {
        let counts = sqlx::query_as::<_, (i64, i64, i64, i64)>(
            "SELECT
               (SELECT count(*) FROM issues WHERE project_id = $1)::bigint,
               (SELECT count(*) FROM issues WHERE project_id = $1 AND priority = 'critical')::bigint,
               (SELECT count(*) FROM clashes WHERE project_id = $1)::bigint,
               (SELECT count(*) FROM sheets WHERE project_id = $1)::bigint",
        )
        .bind(project_id)
        .fetch_one(&state.pool)
        .await?;

        return Ok(Json(AskResponse {
            answer: format!(
                "The project has {} issues, {} critical, {} clashes, and {} sheets. This is a Phase 1 deterministic summary from the project database.",
                counts.0, counts.1, counts.2, counts.3
            ),
            citations: vec![Citation {
                kind: "project".to_string(),
                id: project_id.to_string(),
                label: "Project record".to_string(),
            }],
            map_action: None,
            mode: "deterministic-tools",
        }));
    }

    Ok(Json(AskResponse {
        answer: "Phase 1 supports project summaries and flood-zone issue queries. Try: “Which critical issues are inside the flood zone?”".to_string(),
        citations: vec![],
        map_action: None,
        mode: "deterministic-tools",
    }))
}
