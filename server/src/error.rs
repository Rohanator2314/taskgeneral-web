use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use taskgeneral_core::error::TaskError;

pub enum AppError {
    Database(String),
    NotFound(String),
    BadRequest(String),
    Unauthorized,
    Internal(String),
}

impl From<TaskError> for AppError {
    fn from(e: TaskError) -> Self {
        match e {
            TaskError::TaskNotFound(_) => AppError::NotFound(e.to_string()),
            TaskError::InvalidDescription(_)
            | TaskError::InvalidDate(_)
            | TaskError::InvalidUuid(_)
            | TaskError::InvalidSyncUrl(_) => AppError::BadRequest(e.to_string()),
            _ => AppError::Internal(e.to_string()),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}
