use std::path::Path;
use taskgeneral_server::create_app;
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "taskgeneral_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv::dotenv().ok();

    let app = create_app().await;

    let static_dir = std::env::var("TASKGENERAL_STATIC_DIR").unwrap_or_else(|_| "./frontend/dist".to_string());

    let app = if Path::new(&static_dir).is_dir() {
        tracing::info!("Serving static files from {}", static_dir);
        let serve_dir = ServeDir::new(&static_dir)
            .fallback(ServeFile::new(format!("{}/index.html", static_dir)));
        app.fallback_service(serve_dir)
    } else {
        tracing::info!("Static dir '{}' not found — API-only mode", static_dir);
        app
    };

    let port = std::env::var("TASKGENERAL_PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);

    tracing::info!("Starting server on {}", addr);
    let listener = TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");
    axum::serve(listener, app).await.expect("Server failed");
}
