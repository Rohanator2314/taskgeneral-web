use axum::{
    extract::State,
    http::{header, Request, StatusCode},
    routing::{get, post},
    Router,
};
use native_tls::TlsConnector;
use postgres_native_tls::MakeTlsConnector;
use std::sync::Arc;
use taskgeneral_core::storage::pg::PostgresTaskManager;
use uuid::Uuid;

pub mod auth;
pub mod config;
pub mod error;
pub mod handlers;
pub mod state;
pub mod types;

use auth::Auth;
use handlers::{
    clear_data, complete_task, configure_sync, create_task, delete_task, get_sync_config, get_task,
    get_working_set, list_tasks, start_task, stop_task, sync_now, uncomplete_task, update_task,
};
use state::AppState;

#[derive(Clone)]
pub struct UserId(pub Uuid);

pub async fn create_app() -> Router {
    let app_config = config::Config::from_env().expect("Failed to load config");

    let rt = Arc::new(
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed to build Postgres runtime"),
    );

    let pg_config: tokio_postgres::Config = app_config
        .database_url
        .parse()
        .expect("Failed to parse DATABASE_URL");
    let tls_connector = TlsConnector::builder()
        .build()
        .expect("Failed to build TLS connector");
    let tls = MakeTlsConnector::new(tls_connector);

    let (client, connection) = pg_config
        .connect(tls)
        .await
        .expect("Failed to connect to database");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    let client = Arc::new(client);

    let init_mgr =
        PostgresTaskManager::new_arc(client.clone(), &Uuid::nil().to_string(), rt.clone())
            .expect("Failed to create init manager");
    init_mgr
        .init_schema()
        .await
        .expect("Failed to initialise schema");

    let mut auth = Auth::from_env();
    auth.load_jwks()
        .await
        .expect("Failed to load Supabase JWKS");

    let state = AppState { client, rt, auth };

    let protected = Router::new()
        .route("/api/tasks", get(list_tasks).post(create_task))
        .route(
            "/api/tasks/{uuid}",
            get(get_task).put(update_task).delete(delete_task),
        )
        .route("/api/tasks/{uuid}/complete", post(complete_task))
        .route("/api/tasks/{uuid}/uncomplete", post(uncomplete_task))
        .route("/api/tasks/{uuid}/start", post(start_task))
        .route("/api/tasks/{uuid}/stop", post(stop_task))
        .route("/api/sync/configure", post(configure_sync))
        .route("/api/sync/config", get(get_sync_config))
        .route("/api/sync", post(sync_now))
        .route("/api/data", axum::routing::delete(clear_data))
        .route("/api/tasks/working-set", get(get_working_set))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    Router::new()
        .route("/api/health", get(health_handler))
        .route("/api/version", get(version_handler))
        .merge(protected)
        .with_state(state)
}

async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, StatusCode> {
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok());

    let token = match auth_header {
        Some(v) if v.starts_with("Bearer ") => &v[7..],
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    match state.auth.validate_token(token).await {
        Ok(user_id) => {
            request.extensions_mut().insert(UserId(user_id));
            Ok(next.run(request).await)
        }
        Err(e) => {
            tracing::warn!("JWT validation failed: {}", e);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

async fn health_handler() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({"status": "ok"}))
}

async fn version_handler() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({"version": env!("CARGO_PKG_VERSION")}))
}
