use axum::{routing::get, Router};
use std::env;
use tokio::net::TcpListener;
use taskgeneral_core::{create_task_manager, version};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod types;
mod error;
mod state;

use state::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "taskgeneral_server=debug,tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let data_dir = env::var("TASKGENERAL_DATA_DIR")
        .unwrap_or_else(|_| {
            let home = env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
            format!("{}/.local/share/taskgeneral", home)
        });

    std::fs::create_dir_all(&data_dir).expect("Failed to create data directory");

    let manager = tokio::task::spawn_blocking(move || {
        create_task_manager(data_dir).expect("Failed to initialize task manager")
    })
    .await
    .expect("Failed to join task");
    let state = AppState { manager };

    let app = Router::new()
        .route("/api/health", get(health_handler))
        .route("/api/version", get(version_handler))
        .with_state(state);

    let port = env::var("TASKGENERAL_PORT")
        .unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);

    tracing::info!("Starting server on {}", addr);
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind to address");
    axum::serve(listener, app).await.expect("Server failed");
}

async fn health_handler() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({"status": "ok"}))
}

async fn version_handler() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({"version": version()}))
}
