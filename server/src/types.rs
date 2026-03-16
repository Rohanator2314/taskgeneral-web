use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub description: String,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct UpdateTaskRequest {
    pub description: Option<String>,
    pub project: Option<String>,
    pub tags: Option<Vec<String>>,
    pub priority: Option<String>,
    pub due: Option<String>,
    pub wait: Option<String>,
    pub recur: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TaskFilterQuery {
    pub status: Option<String>,
    pub project: Option<String>,
    pub tag: Option<String>,
    pub sort_by: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TaskInfo {
    pub uuid: String,
    pub description: String,
    pub status: String,
    pub project: Option<String>,
    pub tags: Vec<String>,
    pub priority: Option<String>,
    pub entry: Option<String>,
    pub modified: Option<String>,
    pub due: Option<String>,
    pub wait: Option<String>,
    pub start: Option<String>,
    pub recur: Option<String>,
    pub urgency: f64,
    pub is_active: bool,
    pub is_waiting: bool,
}

#[derive(Debug, Deserialize)]
pub struct ConfigureSyncRequest {
    pub server_url: String,
    pub client_id: String,
    pub encryption_secret: String,
}

#[derive(Debug, Serialize)]
pub struct WorkingSetResponse {
    pub tasks: Vec<String>,
}
