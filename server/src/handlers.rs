use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use taskgeneral_core::{error::TaskError, models::TaskUpdate};

use crate::{
    error::AppError,
    state::AppState,
    types::{CreateTaskRequest, TaskFilterQuery, UpdateTaskRequest, parse_sort_field},
};

pub async fn create_task(
    State(state): State<AppState>,
    Json(req): Json<CreateTaskRequest>,
) -> Result<impl IntoResponse, AppError> {
    let manager = state.manager.clone();
    let result = tokio::task::spawn_blocking(move || {
        manager.create_task(req.description)
    })
    .await
    .unwrap()?;
    Ok((StatusCode::CREATED, Json(result)))
}

pub async fn get_task(
    State(state): State<AppState>,
    Path(uuid): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let manager = state.manager.clone();
    let uuid_clone = uuid.clone();
    let result = tokio::task::spawn_blocking(move || {
        manager.get_task(uuid_clone)
    })
    .await
    .unwrap()?;
    
    match result {
        Some(task) if task.status == "deleted" => Err(AppError::Core(TaskError::TaskNotFound(format!(
            "Task with UUID {} not found",
            uuid
        )))),
        Some(task) => Ok(Json(task)),
        None => Err(AppError::Core(TaskError::TaskNotFound(format!(
            "Task with UUID {} not found",
            uuid
        )))),
    }
}

pub async fn update_task(
    State(state): State<AppState>,
    Path(uuid): Path<String>,
    Json(req): Json<UpdateTaskRequest>,
) -> Result<impl IntoResponse, AppError> {
    let manager = state.manager.clone();
    let task_update: TaskUpdate = req.into();
    let result = tokio::task::spawn_blocking(move || {
        manager.update_task(uuid, task_update)
    })
    .await
    .unwrap()?;
    Ok(Json(result))
}

pub async fn delete_task(
    State(state): State<AppState>,
    Path(uuid): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let manager = state.manager.clone();
    tokio::task::spawn_blocking(move || {
        manager.delete_task(uuid)
    })
    .await
    .unwrap()?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_tasks(
    State(state): State<AppState>,
    Query(filter_query): Query<TaskFilterQuery>,
) -> Result<impl IntoResponse, AppError> {
    let manager = state.manager.clone();
    
    let has_any_param = filter_query.sort_by.is_some()
        || filter_query.status.is_some()
        || filter_query.project.is_some()
        || filter_query.tag.is_some();
    
    let result = if has_any_param {
        let sort_field = match filter_query.sort_by.as_deref() {
            Some(s) => parse_sort_field(s)?,
            None => taskgeneral_core::models::SortField::Urgency,
        };
        let filter = filter_query.into();
        tokio::task::spawn_blocking(move || {
            manager.list_tasks_sorted(filter, sort_field)
        })
        .await
        .unwrap()?
    } else {
        tokio::task::spawn_blocking(move || {
            manager.list_tasks()
        })
        .await
        .unwrap()?
    };
    
    Ok(Json(result))
}

pub async fn complete_task(
    State(state): State<AppState>,
    Path(uuid): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let manager = state.manager.clone();
    let result = tokio::task::spawn_blocking(move || {
        manager.complete_task(uuid)
    })
    .await
    .unwrap()?;
    Ok(Json(result))
}

pub async fn uncomplete_task(
    State(state): State<AppState>,
    Path(uuid): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let manager = state.manager.clone();
    let result = tokio::task::spawn_blocking(move || {
        manager.uncomplete_task(uuid)
    })
    .await
    .unwrap()?;
    Ok(Json(result))
}

pub async fn start_task(
    State(state): State<AppState>,
    Path(uuid): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let manager = state.manager.clone();
    let result = tokio::task::spawn_blocking(move || {
        manager.start_task(uuid)
    })
    .await
    .unwrap()?;
    Ok(Json(result))
}

pub async fn stop_task(
    State(state): State<AppState>,
    Path(uuid): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let manager = state.manager.clone();
    tokio::task::spawn_blocking(move || {
        manager.stop_task(uuid)
    })
    .await
    .unwrap()?;
    Ok(StatusCode::OK)
}

pub async fn configure_sync(
    State(state): State<AppState>,
    Json(req): Json<crate::types::SyncConfigRequest>,
) -> Result<impl IntoResponse, AppError> {
    let manager = state.manager.clone();
    tokio::task::spawn_blocking(move || {
        manager.configure_sync(req.server_url, req.encryption_secret, req.client_id)
    })
    .await
    .unwrap()?;
    Ok(Json(serde_json::json!({"status": "configured"})))
}

pub async fn sync_now(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let manager = state.manager.clone();
    let result = tokio::task::spawn_blocking(move || {
        manager.sync()
    })
    .await
    .unwrap()?;
    Ok(Json(serde_json::json!({
        "success": result.success,
        "message": result.message,
    })))
}

pub async fn clear_data(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let manager = state.manager.clone();
    tokio::task::spawn_blocking(move || {
        manager.clear_local_data()
    })
    .await
    .unwrap()?;
    tracing::warn!("clear_local_data called — destructive operation");
    Ok(Json(serde_json::json!({"status": "cleared"})))
}

pub async fn get_working_set(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let manager = state.manager.clone();
    let working_set = tokio::task::spawn_blocking(move || {
        manager.get_working_set()
    })
    .await
    .unwrap()?;
    let items: Vec<serde_json::Value> = working_set.into_iter().map(|item| {
        serde_json::json!({
            "id": item.id,
            "task": item.task,
        })
    }).collect();
    Ok(Json(items))
}
