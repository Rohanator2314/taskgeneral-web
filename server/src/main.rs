use std::env;
use std::path::Path;
use tokio::net::TcpListener;
use taskgeneral_core::create_task_manager;
use tower_http::services::{ServeDir, ServeFile};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use taskgeneral_server::{create_app, state::AppState};

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

    let mut app = create_app(state);

    let static_dir = env::var("TASKGENERAL_STATIC_DIR")
        .unwrap_or_else(|_| "./frontend/dist".to_string());

    if Path::new(&static_dir).is_dir() {
        tracing::info!("Serving static files from {}", static_dir);
        let serve_dir = ServeDir::new(&static_dir)
            .fallback(ServeFile::new(format!("{}/index.html", static_dir)));
        app = app.fallback_service(serve_dir);
    } else {
        tracing::info!("Static dir '{}' not found — API-only mode", static_dir);
    }

    let port = env::var("TASKGENERAL_PORT")
        .unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);

    tracing::info!("Starting server on {}", addr);
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind to address");
    axum::serve(listener, app).await.expect("Server failed");
}
