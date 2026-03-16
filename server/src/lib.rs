use axum::{
    extract::State,
    http::{header, Request, StatusCode},
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio_postgres::{NoTls, Config};
use uuid::Uuid;

pub mod config;
pub mod error;
pub mod handlers;
pub mod state;
pub mod types;
pub mod auth;

use auth::Auth;
use handlers::{
    clear_data, complete_task, configure_sync, create_task, delete_task, get_task, get_working_set,
    list_tasks, start_task, stop_task, sync_now, uncomplete_task, update_task,
};
use state::AppState;

#[derive(Clone)]
pub struct UserId(pub Uuid);

fn parse_pg_url(url: &str) -> Config {
    let mut config = Config::new();
    
    let url_part = url.strip_prefix("postgres://").or_else(|| url.strip_prefix("postgresql://")).unwrap_or(url);
    
    let (user_info, host_part) = if let Some(at) = url_part.find('@') {
        (&url_part[..at], &url_part[at+1..])
    } else {
        ("", url_part)
    };
    
    if let Some((user, pass)) = user_info.split_once(':') {
        config.user(user);
        config.password(pass);
    }
    
    let (host_port, db_part) = host_part.split_once('/').unwrap_or((host_part, "postgres"));
    config.dbname(db_part);
    
    if let Some((host, port)) = host_port.rsplit_once(':') {
        config.host(host);
        config.port(port.parse().unwrap_or(5432));
    } else {
        config.host(host_port);
        config.port(5432);
    }
    
    config
}

pub async fn create_app() -> Router {
    let app_config = config::Config::from_env().expect("Failed to load config");
    
    let pg_config = parse_pg_url(&app_config.database_url);
    let (client, connection) = pg_config.connect(NoTls).await.expect("Failed to connect to database");
    
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });
    
    let auth = Auth::from_env();
    let state = AppState { 
        db: Arc::new(client),
        auth,
    };

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
        .route("/api/sync", post(sync_now))
        .route("/api/data", axum::routing::delete(clear_data))
        .route("/api/tasks/working-set", get(get_working_set))
        .layer(axum::middleware::from_fn_with_state(state.clone(), auth_middleware));

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
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

async fn health_handler() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({"status": "ok"}))
}

async fn version_handler() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({"version": env!("CARGO_PKG_VERSION")}))
}
