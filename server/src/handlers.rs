use axum::{
    extract::{Extension, Path, Query, State},
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

fn row_to_task_info(row: &tokio_postgres::Row) -> TaskInfo {
    let uuid: Uuid = row.get(0);
    TaskInfo {
        uuid: uuid.to_string(),
        description: row.get(1),
        status: row.get(2),
        project: row.get(3),
        tags: row.get::<_, Option<Vec<String>>>(4).unwrap_or_default(),
        priority: row.get(5),
        entry: row
            .get::<_, Option<chrono::DateTime<chrono::Utc>>>(6)
            .map(|t| t.to_rfc3339()),
        modified: row
            .get::<_, Option<chrono::DateTime<chrono::Utc>>>(7)
            .map(|t| t.to_rfc3339()),
        due: row
            .get::<_, Option<chrono::DateTime<chrono::Utc>>>(8)
            .map(|t| t.to_rfc3339()),
        wait: row
            .get::<_, Option<chrono::DateTime<chrono::Utc>>>(9)
            .map(|t| t.to_rfc3339()),
        start: row
            .get::<_, Option<chrono::DateTime<chrono::Utc>>>(10)
            .map(|t| t.to_rfc3339()),
        recur: row.get(11),
        urgency: row.get::<_, Option<f64>>(12).unwrap_or(0.0),
        is_active: row.get::<_, Option<bool>>(13).unwrap_or(false),
        is_waiting: row.get::<_, Option<bool>>(14).unwrap_or(false),
    }
}

const SELECT_COLS: &str = "uuid, description, status, project, tags, priority, entry, modified_at, due, wait, start, recur, urgency, is_active, is_waiting";

pub async fn list_tasks(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Query(filter): Query<TaskFilterQuery>,
) -> Result<Json<Vec<TaskInfo>>, AppError> {
    let user_id = user_id.0;

    let order = match filter.sort_by.as_deref() {
        Some("modified") => "modified_at DESC",
        Some("project") => "project ASC NULLS LAST",
        _ => "urgency DESC NULLS LAST",
    };

    let rows = match (&filter.status, &filter.project, &filter.tag) {
        (None, None, None) => state
            .db
            .query(
                &format!("SELECT {SELECT_COLS} FROM tasks WHERE user_id = $1 ORDER BY {order}"),
                &[&user_id],
            )
            .await
            .map_err(|e| AppError::Database(e.to_string()))?,
        (Some(s), None, None) => state
            .db
            .query(
                &format!("SELECT {SELECT_COLS} FROM tasks WHERE user_id = $1 AND status = $2 ORDER BY {order}"),
                &[&user_id, s],
            )
            .await
            .map_err(|e| AppError::Database(e.to_string()))?,
        (None, Some(p), None) => state
            .db
            .query(
                &format!("SELECT {SELECT_COLS} FROM tasks WHERE user_id = $1 AND project = $2 ORDER BY {order}"),
                &[&user_id, p],
            )
            .await
            .map_err(|e| AppError::Database(e.to_string()))?,
        (None, None, Some(t)) => state
            .db
            .query(
                &format!("SELECT {SELECT_COLS} FROM tasks WHERE user_id = $1 AND $2 = ANY(tags) ORDER BY {order}"),
                &[&user_id, t],
            )
            .await
            .map_err(|e| AppError::Database(e.to_string()))?,
        (Some(s), Some(p), None) => state
            .db
            .query(
                &format!("SELECT {SELECT_COLS} FROM tasks WHERE user_id = $1 AND status = $2 AND project = $3 ORDER BY {order}"),
                &[&user_id, s, p],
            )
            .await
            .map_err(|e| AppError::Database(e.to_string()))?,
        (Some(s), None, Some(t)) => state
            .db
            .query(
                &format!("SELECT {SELECT_COLS} FROM tasks WHERE user_id = $1 AND status = $2 AND $3 = ANY(tags) ORDER BY {order}"),
                &[&user_id, s, t],
            )
            .await
            .map_err(|e| AppError::Database(e.to_string()))?,
        (None, Some(p), Some(t)) => state
            .db
            .query(
                &format!("SELECT {SELECT_COLS} FROM tasks WHERE user_id = $1 AND project = $2 AND $3 = ANY(tags) ORDER BY {order}"),
                &[&user_id, p, t],
            )
            .await
            .map_err(|e| AppError::Database(e.to_string()))?,
        (Some(s), Some(p), Some(t)) => state
            .db
            .query(
                &format!("SELECT {SELECT_COLS} FROM tasks WHERE user_id = $1 AND status = $2 AND project = $3 AND $4 = ANY(tags) ORDER BY {order}"),
                &[&user_id, s, p, t],
            )
            .await
            .map_err(|e| AppError::Database(e.to_string()))?,
    };

    let tasks: Vec<TaskInfo> = rows.iter().map(row_to_task_info).collect();
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
            "INSERT INTO tasks (uuid, user_id, description, status, tags, urgency, is_active, is_waiting, entry, modified_at, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $9, $9)",
            &[
                &uuid,
                &user_id,
                &req.description,
                &"pending",
                &empty_tags,
                &0.0_f64,
                &false,
                &false,
                &now,
            ],
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
            &format!("SELECT {SELECT_COLS} FROM tasks WHERE uuid = $1 AND user_id = $2"),
            &[&uuid, &user_id],
        )
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(Json(row_to_task_info(&row)))
}

pub async fn update_task(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(uuid): Path<Uuid>,
    Json(req): Json<UpdateTaskRequest>,
) -> Result<Json<TaskInfo>, AppError> {
    let user_id = user_id.0;
    let now = chrono::Utc::now();

    let row = state
        .db
        .query_one(
            &format!(
                "UPDATE tasks SET \
                description = COALESCE($1, description), \
                project = COALESCE($2, project), \
                tags = COALESCE($3, tags), \
                priority = COALESCE($4, priority), \
                due = $5, wait = $6, recur = $7, \
                modified_at = $8 \
                WHERE uuid = $9 AND user_id = $10 \
                RETURNING {SELECT_COLS}"
            ),
            &[
                &req.description,
                &req.project,
                &req.tags,
                &req.priority,
                &req.due,
                &req.wait,
                &req.recur,
                &now,
                &uuid,
                &user_id,
            ],
        )
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(Json(row_to_task_info(&row)))
}

pub async fn delete_task(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(uuid): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let user_id = user_id.0;
    state
        .db
        .execute(
            "DELETE FROM tasks WHERE uuid = $1 AND user_id = $2",
            &[&uuid, &user_id],
        )
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
    let row = state
        .db
        .query_one(
            &format!(
                "UPDATE tasks SET status = 'completed', modified_at = $1, is_active = false \
                WHERE uuid = $2 AND user_id = $3 RETURNING {SELECT_COLS}"
            ),
            &[&now, &uuid, &user_id],
        )
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(Json(row_to_task_info(&row)))
}

pub async fn uncomplete_task(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<TaskInfo>, AppError> {
    let user_id = user_id.0;
    let now = chrono::Utc::now();
    let row = state
        .db
        .query_one(
            &format!(
                "UPDATE tasks SET status = 'pending', modified_at = $1 \
                WHERE uuid = $2 AND user_id = $3 RETURNING {SELECT_COLS}"
            ),
            &[&now, &uuid, &user_id],
        )
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(Json(row_to_task_info(&row)))
}

pub async fn start_task(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<TaskInfo>, AppError> {
    let user_id = user_id.0;
    let now = chrono::Utc::now();
    let row = state
        .db
        .query_one(
            &format!(
                "UPDATE tasks SET is_active = true, start = $1, modified_at = $1 \
                WHERE uuid = $2 AND user_id = $3 RETURNING {SELECT_COLS}"
            ),
            &[&now, &uuid, &user_id],
        )
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(Json(row_to_task_info(&row)))
}

pub async fn stop_task(
    State(state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<TaskInfo>, AppError> {
    let user_id = user_id.0;
    let now = chrono::Utc::now();
    let row = state
        .db
        .query_one(
            &format!(
                "UPDATE tasks SET is_active = false, modified_at = $1 \
                WHERE uuid = $2 AND user_id = $3 RETURNING {SELECT_COLS}"
            ),
            &[&now, &uuid, &user_id],
        )
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(Json(row_to_task_info(&row)))
}

pub async fn configure_sync(
    State(_state): State<AppState>,
    Json(_req): Json<ConfigureSyncRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(
        json!({ "status": "not implemented - use TaskChampion CLI for sync" }),
    ))
}

pub async fn sync_now(State(_state): State<AppState>) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(
        json!({ "status": "not implemented - use TaskChampion CLI for sync" }),
    ))
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

    let tasks: Vec<String> = rows
        .into_iter()
        .map(|row| {
            let uuid: Uuid = row.get(0);
            uuid.to_string()
        })
        .collect();

    Ok(Json(WorkingSetResponse { tasks }))
}
