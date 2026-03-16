use axum::{
    extract::{Path, Query, State, Extension},
    http::StatusCode,
    Json,
};
use serde_json::json;
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

pub async fn list_tasks(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Query(filter): Query<TaskFilterQuery>,
) -> Result<Json<Vec<TaskInfo>>, AppError> {
    let user_id = user_id.0;
    
    let query = match (&filter.status, &filter.project, &filter.tag, &filter.sort_by) {
        (None, None, None, None) => {
            "SELECT uuid, description, status, project, tags, priority, entry, modified, due, wait, start, recur, urgency, is_active, is_waiting FROM tasks WHERE user_id = $1 ORDER BY urgency DESC".to_string()
        }
        _ => {
            let mut q = "SELECT uuid, description, status, project, tags, priority, entry, modified, due, wait, start, recur, urgency, is_active, is_waiting FROM tasks WHERE user_id = $1".to_string();
            let mut idx = 2;
            if filter.status.is_some() { q.push_str(" AND status = $2"); }
            if filter.project.is_some() { q.push_str(" AND project = $3"); }
            if filter.tag.is_some() { q.push_str(" AND $4 = ANY(tags)"); }
            q.push_str(" ORDER BY ");
            q.push_str(match filter.sort_by.as_deref() {
                Some("modified") => "modified DESC",
                Some("project") => "project ASC",
                _ => "urgency DESC"
            });
            q
        }
    };

    let rows = state
        .db
        .query(&query, &[&user_id])
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    let tasks: Vec<TaskInfo> = rows
        .into_iter()
        .map(|row| TaskInfo {
            uuid: row.get(0),
            description: row.get(1),
            status: row.get(2),
            project: row.get(3),
            tags: row.get(4),
            priority: row.get(5),
            entry: row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(6).map(|t| t.to_rfc3339()),
            modified: row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(7).map(|t| t.to_rfc3339()),
            due: row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(8).map(|t| t.to_rfc3339()),
            wait: row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(9).map(|t| t.to_rfc3339()),
            start: row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(10).map(|t| t.to_rfc3339()),
            recur: row.get(11),
            urgency: row.get(12),
            is_active: row.get(13),
            is_waiting: row.get(14),
        })
        .collect();

    Ok(Json(tasks))
}

pub async fn create_task(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Json(req): Json<CreateTaskRequest>,
) -> Result<Json<TaskInfo>, AppError> {
    let user_id = user_id.0;
    let uuid = Uuid::new_v4();
    let now = chrono::Utc::now();

    let empty_tags: Vec<String> = vec![];
    state
        .db
        .execute(
            "INSERT INTO tasks (uuid, user_id, description, status, tags, urgency, is_active, is_waiting, entry, modified, created_at, modified_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
            &[&uuid, &user_id, &req.description, &"pending", &empty_tags, &0.0, &false, &false, &now, &now, &now, &now],
        )
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(Json(TaskInfo {
        uuid: uuid.to_string(),
        description: req.description,
        status: "pending".to_string(),
        project: None,
        tags: vec![],
        priority: None,
        entry: Some(now.to_rfc3339()),
        modified: Some(now.to_rfc3339()),
        due: None,
        wait: None,
        start: None,
        recur: None,
        urgency: 0.0,
        is_active: false,
        is_waiting: false,
    }))
}

pub async fn get_task(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<TaskInfo>, AppError> {
    let user_id = user_id.0;
    let row = state
        .db
        .query_one(
            "SELECT uuid, description, status, project, tags, priority, entry, modified, due, wait, start, recur, urgency, is_active, is_waiting FROM tasks WHERE uuid = $1 AND user_id = $2",
            &[&uuid, &user_id],
        )
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(Json(TaskInfo {
        uuid: row.get(0),
        description: row.get(1),
        status: row.get(2),
        project: row.get(3),
        tags: row.get(4),
        priority: row.get(5),
        entry: row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(6).map(|t| t.to_rfc3339()),
        modified: row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(7).map(|t| t.to_rfc3339()),
        due: row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(8).map(|t| t.to_rfc3339()),
        wait: row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(9).map(|t| t.to_rfc3339()),
        start: row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(10).map(|t| t.to_rfc3339()),
        recur: row.get(11),
        urgency: row.get(12),
        is_active: row.get(13),
        is_waiting: row.get(14),
    }))
}

pub async fn update_task(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(uuid): Path<Uuid>,
    Json(req): Json<UpdateTaskRequest>,
) -> Result<Json<TaskInfo>, AppError> {
    let user_id = user_id.0;
    let now = chrono::Utc::now();

    let row = state.db.query_one(
        "UPDATE tasks SET description = COALESCE($1, description), project = $2, tags = COALESCE($3, tags), priority = $4, due = $5, wait = $6, recur = $7, modified_at = $8 WHERE uuid = $9 AND user_id = $10 RETURNING uuid, description, status, project, tags, priority, entry, modified_at, due, wait, start, recur, urgency, is_active, is_waiting",
        &[&req.description, &req.project, &req.tags, &req.priority, &req.due, &req.wait, &req.recur, &now, &uuid, &user_id],
    ).await.map_err(|e| AppError::Database(e.to_string()))?;

    Ok(Json(TaskInfo {
        uuid: row.get(0),
        description: row.get(1),
        status: row.get(2),
        project: row.get(3),
        tags: row.get(4),
        priority: row.get(5),
        entry: row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(6).map(|t| t.to_rfc3339()),
        modified: row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(7).map(|t| t.to_rfc3339()),
        due: row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(8).map(|t| t.to_rfc3339()),
        wait: row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(9).map(|t| t.to_rfc3339()),
        start: row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(10).map(|t| t.to_rfc3339()),
        recur: row.get(11),
        urgency: row.get(12),
        is_active: row.get(13),
        is_waiting: row.get(14),
    }))
}

pub async fn delete_task(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(uuid): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let user_id = user_id.0;
    state
        .db
        .execute("DELETE FROM tasks WHERE uuid = $1 AND user_id = $2", &[&uuid, &user_id])
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn complete_task(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<TaskInfo>, AppError> {
    let user_id = user_id.0;
    let now = chrono::Utc::now();
    let row = state.db.query_one(
        "UPDATE tasks SET status = 'completed', modified_at = $1, is_active = false WHERE uuid = $2 AND user_id = $3 RETURNING uuid, description, status, project, tags, priority, entry, modified_at, due, wait, start, recur, urgency, is_active, is_waiting",
        &[&now, &uuid, &user_id],
    ).await.map_err(|e| AppError::Database(e.to_string()))?;

    Ok(Json(TaskInfo {
        uuid: row.get(0),
        description: row.get(1),
        status: row.get(2),
        project: row.get(3),
        tags: row.get(4),
        priority: row.get(5),
        entry: row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(6).map(|t| t.to_rfc3339()),
        modified: row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(7).map(|t| t.to_rfc3339()),
        due: row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(8).map(|t| t.to_rfc3339()),
        wait: row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(9).map(|t| t.to_rfc3339()),
        start: row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(10).map(|t| t.to_rfc3339()),
        recur: row.get(11),
        urgency: row.get(12),
        is_active: row.get(13),
        is_waiting: row.get(14),
    }))
}

pub async fn uncomplete_task(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<TaskInfo>, AppError> {
    let user_id = user_id.0;
    let now = chrono::Utc::now();
    let row = state.db.query_one(
        "UPDATE tasks SET status = 'pending', modified_at = $1 WHERE uuid = $2 AND user_id = $3 RETURNING uuid, description, status, project, tags, priority, entry, modified_at, due, wait, start, recur, urgency, is_active, is_waiting",
        &[&now, &uuid, &user_id],
    ).await.map_err(|e| AppError::Database(e.to_string()))?;

    Ok(Json(TaskInfo {
        uuid: row.get(0),
        description: row.get(1),
        status: row.get(2),
        project: row.get(3),
        tags: row.get(4),
        priority: row.get(5),
        entry: row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(6).map(|t| t.to_rfc3339()),
        modified: row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(7).map(|t| t.to_rfc3339()),
        due: row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(8).map(|t| t.to_rfc3339()),
        wait: row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(9).map(|t| t.to_rfc3339()),
        start: row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(10).map(|t| t.to_rfc3339()),
        recur: row.get(11),
        urgency: row.get(12),
        is_active: row.get(13),
        is_waiting: row.get(14),
    }))
}

pub async fn start_task(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<TaskInfo>, AppError> {
    let user_id = user_id.0;
    let now = chrono::Utc::now();
    let row = state.db.query_one(
        "UPDATE tasks SET is_active = true, start = $1, modified_at = $1 WHERE uuid = $2 AND user_id = $3 RETURNING uuid, description, status, project, tags, priority, entry, modified_at, due, wait, start, recur, urgency, is_active, is_waiting",
        &[&now, &uuid, &user_id],
    ).await.map_err(|e| AppError::Database(e.to_string()))?;

    Ok(Json(TaskInfo {
        uuid: row.get(0),
        description: row.get(1),
        status: row.get(2),
        project: row.get(3),
        tags: row.get(4),
        priority: row.get(5),
        entry: row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(6).map(|t| t.to_rfc3339()),
        modified: row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(7).map(|t| t.to_rfc3339()),
        due: row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(8).map(|t| t.to_rfc3339()),
        wait: row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(9).map(|t| t.to_rfc3339()),
        start: row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(10).map(|t| t.to_rfc3339()),
        recur: row.get(11),
        urgency: row.get(12),
        is_active: row.get(13),
        is_waiting: row.get(14),
    }))
}

pub async fn stop_task(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<TaskInfo>, AppError> {
    let user_id = user_id.0;
    let now = chrono::Utc::now();
    let row = state.db.query_one(
        "UPDATE tasks SET is_active = false, modified_at = $1 WHERE uuid = $2 AND user_id = $3 RETURNING uuid, description, status, project, tags, priority, entry, modified_at, due, wait, start, recur, urgency, is_active, is_waiting",
        &[&now, &uuid, &user_id],
    ).await.map_err(|e| AppError::Database(e.to_string()))?;

    Ok(Json(TaskInfo {
        uuid: row.get(0),
        description: row.get(1),
        status: row.get(2),
        project: row.get(3),
        tags: row.get(4),
        priority: row.get(5),
        entry: row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(6).map(|t| t.to_rfc3339()),
        modified: row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(7).map(|t| t.to_rfc3339()),
        due: row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(8).map(|t| t.to_rfc3339()),
        wait: row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(9).map(|t| t.to_rfc3339()),
        start: row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(10).map(|t| t.to_rfc3339()),
        recur: row.get(11),
        urgency: row.get(12),
        is_active: row.get(13),
        is_waiting: row.get(14),
    }))
}

pub async fn configure_sync(
    State(_state): State<AppState>,
    Json(_req): Json<ConfigureSyncRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(json!({ "status": "not implemented - use TaskChampion CLI for sync" })))
}

pub async fn sync_now(
    State(_state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(json!({ "status": "not implemented - use TaskChampion CLI for sync" })))
}

pub async fn clear_data(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
) -> Result<StatusCode, AppError> {
    let user_id = user_id.0;
    state
        .db
        .execute("DELETE FROM tasks WHERE user_id = $1", &[&user_id])
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_working_set(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
) -> Result<Json<WorkingSetResponse>, AppError> {
    let user_id = user_id.0;
    let rows = state
        .db
        .query(
            "SELECT task_uuid FROM working_set WHERE user_id = $1 ORDER BY position",
            &[&user_id],
        )
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    let tasks: Vec<String> = rows.into_iter().map(|row| {
        let uuid: String = row.get(0);
        uuid
    }).collect();

    Ok(Json(WorkingSetResponse { tasks }))
}
