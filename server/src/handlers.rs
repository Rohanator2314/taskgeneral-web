use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use taskgeneral_core::{error::TaskError, models::TaskUpdate};

use crate::{
    error::AppError,
    state::AppState,
    types::{CreateTaskRequest, UpdateTaskRequest},
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
