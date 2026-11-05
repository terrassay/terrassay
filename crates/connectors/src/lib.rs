use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProject {
    pub id: String,
    pub name: String,
    pub code: Option<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteIssue {
    pub id: String,
    pub title: String,
    pub status: String,
    pub priority: String,
    pub discipline: Option<String>,
    pub assignee: Option<String>,
}

#[derive(Debug, Error)]
pub enum ConnectorError {
    #[error("remote provider unavailable: {0}")]
    Unavailable(String),
    #[error("unsupported operation: {0}")]
    Unsupported(String),
}

#[async_trait]
pub trait BimConnector: Send + Sync {
    async fn list_projects(&self) -> Result<Vec<RemoteProject>, ConnectorError>;
    async fn list_issues(
        &self,
        project_id: &str,
        limit: usize,
    ) -> Result<Vec<RemoteIssue>, ConnectorError>;
}

/// Deterministic Revizto-shaped fixture used before real API access exists.
pub struct MockRevizto;

impl MockRevizto {
    pub fn new() -> Self {
        Self
    }

    fn project() -> RemoteProject {
        RemoteProject {
            id: "hqz-2026".to_string(),
            name: "Hafenquartier Zürich".to_string(),
            code: Some("HQZ-2026".to_string()),
            status: "active".to_string(),
        }
    }
}

impl Default for MockRevizto {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl BimConnector for MockRevizto {
    async fn list_projects(&self) -> Result<Vec<RemoteProject>, ConnectorError> {
        Ok(vec![Self::project()])
    }

    async fn list_issues(
        &self,
        _project_id: &str,
        limit: usize,
    ) -> Result<Vec<RemoteIssue>, ConnectorError> {
        let disciplines = ["Architecture", "Structure", "MEP", "Landscape"];
        let assignees = ["A. Keller", "M. Rossi", "S. Nguyen", "L. Weber"];
        let count = limit.min(240);

        Ok((1..=count)
            .map(|index| RemoteIssue {
                id: format!("ISSUE-{index:03}"),
                title: format!(
                    "{} coordination item {index}",
                    disciplines[(index - 1) % disciplines.len()]
                ),
                status: if index % 5 == 0 {
                    "closed"
                } else if index % 3 == 0 {
                    "in_progress"
                } else {
                    "open"
                }
                .to_string(),
                priority: if index <= 60 {
                    "critical"
                } else if index <= 140 {
                    "high"
                } else {
                    "normal"
                }
                .to_string(),
                discipline: Some(disciplines[(index - 1) % disciplines.len()].to_string()),
                assignee: Some(assignees[(index - 1) % assignees.len()].to_string()),
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn mock_project_is_deterministic() {
        let connector = MockRevizto::new();
        let projects = connector.list_projects().await.unwrap();
        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].code.as_deref(), Some("HQZ-2026"));
    }

    #[tokio::test]
    async fn mock_issue_count_is_capped_at_240() {
        let connector = MockRevizto::new();
        let issues = connector.list_issues("hqz-2026", 500).await.unwrap();
        assert_eq!(issues.len(), 240);
        assert_eq!(issues[0].priority, "critical");
        assert_eq!(issues[239].priority, "normal");
    }
}
