use crate::types::ErrorResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use taskgeneral_core::error::TaskError;

pub enum AppError {
    Core(TaskError),
    BadRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Core(err) => match err {
                TaskError::TaskNotFound(msg) => (StatusCode::NOT_FOUND, msg),
                TaskError::InvalidUuid(msg)
                | TaskError::InvalidDescription(msg)
                | TaskError::InvalidPriority(msg)
                | TaskError::InvalidDate(msg)
                | TaskError::InvalidRecurrence(msg)
                | TaskError::InvalidStatus(msg)
                | TaskError::InvalidSyncUrl(msg) => (StatusCode::BAD_REQUEST, msg),
                TaskError::SyncNotConfigured => {
                    (StatusCode::CONFLICT, "Sync not configured".to_string())
                }
                TaskError::StorageError(msg)
                | TaskError::SyncError(msg)
                | TaskError::TaskChampionError(msg)
                | TaskError::IoError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            },
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        (status, Json(ErrorResponse { error: message })).into_response()
    }
}

impl From<TaskError> for AppError {
    fn from(e: TaskError) -> Self {
        AppError::Core(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::response::IntoResponse;

    #[test]
    fn test_task_not_found_maps_to_404() {
        let error = AppError::Core(TaskError::TaskNotFound("Task not found".to_string()));
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_invalid_uuid_maps_to_400() {
        let error = AppError::Core(TaskError::InvalidUuid("Invalid UUID".to_string()));
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_invalid_description_maps_to_400() {
        let error = AppError::Core(TaskError::InvalidDescription(
            "Empty description".to_string(),
        ));
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_invalid_priority_maps_to_400() {
        let error = AppError::Core(TaskError::InvalidPriority("Invalid priority".to_string()));
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_invalid_date_maps_to_400() {
        let error = AppError::Core(TaskError::InvalidDate("Invalid date".to_string()));
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_invalid_recurrence_maps_to_400() {
        let error = AppError::Core(TaskError::InvalidRecurrence(
            "Invalid recurrence".to_string(),
        ));
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_invalid_status_maps_to_400() {
        let error = AppError::Core(TaskError::InvalidStatus("Invalid status".to_string()));
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_invalid_sync_url_maps_to_400() {
        let error = AppError::Core(TaskError::InvalidSyncUrl("Invalid URL".to_string()));
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_sync_not_configured_maps_to_409() {
        let error = AppError::Core(TaskError::SyncNotConfigured);
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::CONFLICT);
    }

    #[test]
    fn test_storage_error_maps_to_500() {
        let error = AppError::Core(TaskError::StorageError("Storage error".to_string()));
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_sync_error_maps_to_500() {
        let error = AppError::Core(TaskError::SyncError("Sync error".to_string()));
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_task_champion_error_maps_to_500() {
        let error = AppError::Core(TaskError::TaskChampionError(
            "TaskChampion error".to_string(),
        ));
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_io_error_maps_to_500() {
        let error = AppError::Core(TaskError::IoError("IO error".to_string()));
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_bad_request_maps_to_400() {
        let error = AppError::BadRequest("Bad request".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
