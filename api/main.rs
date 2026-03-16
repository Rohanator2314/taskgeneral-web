use std::env;
use taskgeneral_core::create_task_manager;
use taskgeneral_server::{create_app, state::AppState};
use tower::ServiceBuilder;
use vercel_runtime::Error;
use vercel_runtime::axum::VercelLayer;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "taskgeneral_server=info".into()),
        )
        .init();

    let data_dir = env::var("TASKGENERAL_DATA_DIR").unwrap_or_else(|_| "/tmp/taskgeneral".to_string());

    std::fs::create_dir_all(&data_dir).expect("Failed to create data directory");

    let manager = tokio::task::spawn_blocking(move || {
        create_task_manager(data_dir).expect("Failed to initialize task manager")
    })
    .await
    .expect("Failed to join task");

    let state = AppState { manager };
    let router = create_app(state);

    let app = ServiceBuilder::new()
        .layer(VercelLayer::new())
        .service(router);

    vercel_runtime::run(app).await
}
