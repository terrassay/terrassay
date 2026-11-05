use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub code: Option<String>,
    pub status: String,
    pub analysis_crs: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectSummary {
    pub project_id: Uuid,
    pub issue_count: i64,
    pub critical_count: i64,
    pub clash_count: i64,
    pub sheet_count: i64,
    pub last_synced_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Issue {
    pub id: Uuid,
    pub external_id: String,
    pub title: String,
    pub status: String,
    pub priority: String,
    pub discipline: Option<String>,
    pub assignee_name: Option<String>,
}
