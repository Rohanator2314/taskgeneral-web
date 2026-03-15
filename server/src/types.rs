use crate::error::AppError;
use serde::{Deserialize, Serialize};
use taskgeneral_core::models::{SortField, TaskFilter, TaskUpdate};

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub description: String,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct SyncConfigRequest {
    pub server_url: String,
    pub encryption_secret: String,
    pub client_id: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, Serialize)]
pub struct VersionResponse {
    pub version: String,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
}

impl From<UpdateTaskRequest> for TaskUpdate {
    fn from(req: UpdateTaskRequest) -> Self {
        TaskUpdate {
            description: req.description,
            project: req.project,
            tags: req.tags,
            priority: req.priority,
            due: req.due,
            wait: req.wait,
            recur: req.recur,
        }
    }
}

impl From<TaskFilterQuery> for TaskFilter {
    fn from(query: TaskFilterQuery) -> Self {
        TaskFilter {
            status: query.status,
            project: query.project,
            tag: query.tag,
            sort_by: query.sort_by,
        }
    }
}

pub fn parse_sort_field(s: &str) -> Result<SortField, AppError> {
    match s {
        "urgency" => Ok(SortField::Urgency),
        "due" => Ok(SortField::DueDate),
        "priority" => Ok(SortField::Priority),
        "entry" => Ok(SortField::EntryDate),
        "modified" => Ok(SortField::Modified),
        "description" => Ok(SortField::Description),
        _ => Err(AppError::BadRequest(format!(
            "Invalid sort field: {}. Must be one of: urgency, due, priority, entry, modified, description",
            s
        ))),
    }
}
