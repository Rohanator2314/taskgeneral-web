use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::json;
use taskgeneral_core::{
    models::{TaskFilter, TaskUpdate},
    storage::pg::PostgresTaskManager,
};
use uuid::Uuid;

use crate::{
    error::AppError,
    state::AppState,
    types::{
        ConfigureSyncRequest, CreateTaskRequest, TaskFilterQuery, TaskInfo, UpdateTaskRequest,
        WorkingSetResponse,
    },
    UserId,
};

fn make_mgr(state: &AppState, user_id: Uuid) -> Result<PostgresTaskManager, AppError> {
    PostgresTaskManager::new_arc(state.client.clone(), &user_id.to_string(), state.rt.clone())
        .map_err(|e| AppError::Internal(e.to_string()))
}

fn core_task_to_api(t: taskgeneral_core::models::TaskInfo) -> TaskInfo {
    TaskInfo {
        uuid: t.uuid,
        description: t.description,
        status: t.status,
        project: t.project,
        tags: t.tags,
        priority: t.priority,
        entry: t.entry,
        modified: t.modified,
        due: t.due,
        wait: t.wait,
        start: t.start,
        recur: t.recur,
        urgency: t.urgency,
        is_active: t.is_active,
        is_waiting: t.is_waiting,
    }
}

pub async fn list_tasks(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Query(filter): Query<TaskFilterQuery>,
) -> Result<Json<Vec<TaskInfo>>, AppError> {
    let mut mgr = make_mgr(&state, user_id.0)?;

    let sort_field = match filter.sort_by.as_deref() {
        Some("modified") => taskgeneral_core::models::SortField::Modified,
        Some("project") => taskgeneral_core::models::SortField::Description,
        _ => taskgeneral_core::models::SortField::Urgency,
    };

    let tf = TaskFilter {
        status: filter.status,
        project: filter.project,
        tag: filter.tag,
        sort_by: None,
    };

    let tasks = tokio::task::spawn_blocking(move || mgr.list_tasks_sorted(tf, sort_field))
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .map_err(AppError::from)?;

    Ok(Json(tasks.into_iter().map(core_task_to_api).collect()))
}

pub async fn create_task(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Json(req): Json<CreateTaskRequest>,
) -> Result<Json<TaskInfo>, AppError> {
    let mut mgr = make_mgr(&state, user_id.0)?;

    let task = tokio::task::spawn_blocking(move || mgr.create_task(&req.description))
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .map_err(AppError::from)?;

    Ok(Json(core_task_to_api(task)))
}

pub async fn get_task(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<TaskInfo>, AppError> {
    let mut mgr = make_mgr(&state, user_id.0)?;
    let uuid_str = uuid.to_string();

    let task = tokio::task::spawn_blocking(move || mgr.get_task(&uuid_str))
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::NotFound(format!("Task {} not found", uuid)))?;

    Ok(Json(core_task_to_api(task)))
}

pub async fn update_task(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(uuid): Path<Uuid>,
    Json(req): Json<UpdateTaskRequest>,
) -> Result<Json<TaskInfo>, AppError> {
    let mut mgr = make_mgr(&state, user_id.0)?;
    let uuid_str = uuid.to_string();

    let updates = TaskUpdate {
        description: req.description,
        project: req.project,
        tags: req.tags,
        priority: req.priority,
        due: req.due,
        wait: req.wait,
        recur: req.recur,
    };

    let task = tokio::task::spawn_blocking(move || mgr.update_task(&uuid_str, updates))
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .map_err(AppError::from)?;

    Ok(Json(core_task_to_api(task)))
}

pub async fn delete_task(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(uuid): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let mut mgr = make_mgr(&state, user_id.0)?;
    let uuid_str = uuid.to_string();

    tokio::task::spawn_blocking(move || mgr.delete_task(&uuid_str))
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .map_err(AppError::from)?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn complete_task(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<TaskInfo>, AppError> {
    let mut mgr = make_mgr(&state, user_id.0)?;
    let uuid_str = uuid.to_string();

    let task = tokio::task::spawn_blocking(move || mgr.complete_task(&uuid_str))
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .map_err(AppError::from)?;

    Ok(Json(core_task_to_api(task)))
}

pub async fn uncomplete_task(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<TaskInfo>, AppError> {
    let mut mgr = make_mgr(&state, user_id.0)?;
    let uuid_str = uuid.to_string();

    let task = tokio::task::spawn_blocking(move || mgr.uncomplete_task(&uuid_str))
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .map_err(AppError::from)?;

    Ok(Json(core_task_to_api(task)))
}

pub async fn start_task(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<TaskInfo>, AppError> {
    let mut mgr = make_mgr(&state, user_id.0)?;
    let uuid_str = uuid.to_string();

    let task = tokio::task::spawn_blocking(move || mgr.start_task(&uuid_str))
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .map_err(AppError::from)?;

    Ok(Json(core_task_to_api(task)))
}

pub async fn stop_task(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<TaskInfo>, AppError> {
    let mut mgr = make_mgr(&state, user_id.0)?;
    let uuid_str = uuid.to_string();

    let task = tokio::task::spawn_blocking(move || mgr.stop_task(&uuid_str))
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .map_err(AppError::from)?;

    Ok(Json(core_task_to_api(task)))
}

pub async fn configure_sync(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Json(req): Json<ConfigureSyncRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let mut mgr = make_mgr(&state, user_id.0)?;

    tokio::task::spawn_blocking(move || {
        mgr.configure_sync(&req.server_url, &req.encryption_secret, &req.client_id)
    })
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
    .map_err(AppError::from)?;

    Ok(Json(json!({ "status": "ok" })))
}

pub async fn get_sync_config(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
) -> Result<Json<serde_json::Value>, AppError> {
    let mut mgr = make_mgr(&state, user_id.0)?;

    let result = tokio::task::spawn_blocking(move || mgr.get_sync_config())
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .map_err(AppError::from)?;

    match result {
        None => Ok(Json(
            json!({ "configured": false, "server_url": null, "client_id": null }),
        )),
        Some((server_url, client_id)) => Ok(Json(json!({
            "configured": server_url.is_some(),
            "server_url": server_url,
            "client_id": client_id,
        }))),
    }
}

pub async fn sync_now(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
) -> Result<Json<serde_json::Value>, AppError> {
    let mut mgr = make_mgr(&state, user_id.0)?;

    let result = tokio::task::spawn_blocking(move || mgr.sync())
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .map_err(AppError::from)?;

    Ok(Json(
        json!({ "success": result.success, "message": result.message }),
    ))
}

pub async fn clear_data(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
) -> Result<StatusCode, AppError> {
    let mut mgr = make_mgr(&state, user_id.0)?;

    tokio::task::spawn_blocking(move || mgr.clear_local_data())
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .map_err(AppError::from)?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_working_set(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
) -> Result<Json<WorkingSetResponse>, AppError> {
    let mut mgr = make_mgr(&state, user_id.0)?;

    let items = tokio::task::spawn_blocking(move || mgr.get_working_set())
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .map_err(AppError::from)?;

    let tasks: Vec<String> = items.into_iter().map(|item| item.task.uuid).collect();
    Ok(Json(WorkingSetResponse { tasks }))
}
