use axum::{routing::{get, post}, Router};
use taskgeneral_core::version;

pub mod types;
pub mod error;
pub mod state;
pub mod handlers;

use state::AppState;
use handlers::{create_task, get_task, list_tasks, update_task, delete_task, complete_task, uncomplete_task, start_task, stop_task, configure_sync, sync_now, clear_data, get_working_set};

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/api/health", get(health_handler))
        .route("/api/version", get(version_handler))
        .route("/api/tasks", get(list_tasks).post(create_task))
        .route("/api/tasks/:uuid", get(get_task).put(update_task).delete(delete_task))
        .route("/api/tasks/:uuid/complete", post(complete_task))
        .route("/api/tasks/:uuid/uncomplete", post(uncomplete_task))
        .route("/api/tasks/:uuid/start", post(start_task))
        .route("/api/tasks/:uuid/stop", post(stop_task))
        .route("/api/sync/configure", post(configure_sync))
        .route("/api/sync", post(sync_now))
        .route("/api/data", axum::routing::delete(clear_data))
        .route("/api/tasks/working-set", get(get_working_set))
        .with_state(state)
}

async fn health_handler() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({"status": "ok"}))
}

async fn version_handler() -> axum::Json<serde_json::Value> {
    let v = tokio::task::spawn_blocking(version).await.unwrap();
    axum::Json(serde_json::json!({"version": v}))
}
