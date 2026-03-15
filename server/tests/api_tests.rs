use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::Value;
use taskgeneral_server::{create_app, state::AppState};
use tower::ServiceExt;

fn create_test_app_sync() -> (axum::Router, tempfile::TempDir) {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let data_dir = temp_dir.path().to_str().unwrap().to_string();
    
    let manager = taskgeneral_core::create_task_manager(data_dir)
        .expect("Failed to create task manager");
    
    let state = AppState { manager };
    let app = create_app(state);
    
    (app, temp_dir)
}

async fn create_test_app() -> (axum::Router, tempfile::TempDir) {
    tokio::task::spawn_blocking(|| create_test_app_sync())
        .await
        .unwrap()
}

/// Helper to parse JSON response body
async fn parse_json_body(body: Body) -> Value {
    let bytes = body.collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

async fn cleanup_test(app: axum::Router, _temp_dir: tempfile::TempDir) {
    tokio::task::spawn_blocking(move || {
        drop(app);
        drop(_temp_dir);
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn test_health_and_version() {
    let (app, temp_dir) = create_test_app().await;
    
    // Test health endpoint
    let request = Request::builder()
        .uri("/api/health")
        .body(Body::empty())
        .unwrap();
    
    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = parse_json_body(response.into_body()).await;
    assert_eq!(body["status"], "ok");
    
    // Test version endpoint
    let request = Request::builder()
        .uri("/api/version")
        .body(Body::empty())
        .unwrap();
    
    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = parse_json_body(response.into_body()).await;
    assert!(body.get("version").is_some());
    
    cleanup_test(app, temp_dir).await;
}

#[tokio::test]
async fn test_create_task() {
    let (app, temp_dir) = create_test_app().await;
    
    let request = Request::builder()
        .method("POST")
        .uri("/api/tasks")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"description":"Test task"}"#))
        .unwrap();
    
    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    
    let body = parse_json_body(response.into_body()).await;
    assert!(body["uuid"].is_string());
    assert_eq!(body["description"], "Test task");
    assert_eq!(body["status"], "pending");
    
    cleanup_test(app, temp_dir).await;
}

#[tokio::test]
async fn test_create_task_empty_description() {
    let (app, temp_dir) = create_test_app().await;
    
    let request = Request::builder()
        .method("POST")
        .uri("/api/tasks")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"description":""}"#))
        .unwrap();
    
    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    
    cleanup_test(app, temp_dir).await;
}

#[tokio::test]
async fn test_get_task() {
    let (app, temp_dir) = create_test_app().await;
    
    // Create a task first
    let create_request = Request::builder()
        .method("POST")
        .uri("/api/tasks")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"description":"Get test task"}"#))
        .unwrap();
    
    let create_response = app.clone().oneshot(create_request).await.unwrap();
    let create_body = parse_json_body(create_response.into_body()).await;
    let uuid = create_body["uuid"].as_str().unwrap();
    
    // Get the task
    let get_request = Request::builder()
        .uri(format!("/api/tasks/{}", uuid))
        .body(Body::empty())
        .unwrap();
    
    let response = app.clone().oneshot(get_request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = parse_json_body(response.into_body()).await;
    assert_eq!(body["uuid"], uuid);
    assert_eq!(body["description"], "Get test task");
    
    cleanup_test(app, temp_dir).await;
}

#[tokio::test]
async fn test_get_task_not_found() {
    let (app, temp_dir) = create_test_app().await;
    
    let request = Request::builder()
        .uri("/api/tasks/00000000-0000-0000-0000-000000000000")
        .body(Body::empty())
        .unwrap();
    
    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    
    cleanup_test(app, temp_dir).await;
}

#[tokio::test]
async fn test_get_task_invalid_uuid() {
    let (app, temp_dir) = create_test_app().await;
    
    let request = Request::builder()
        .uri("/api/tasks/not-a-uuid")
        .body(Body::empty())
        .unwrap();
    
    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    
    cleanup_test(app, temp_dir).await;
}

#[tokio::test]
async fn test_update_task() {
    let (app, temp_dir) = create_test_app().await;
    
    // Create a task first
    let create_request = Request::builder()
        .method("POST")
        .uri("/api/tasks")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"description":"Update test task"}"#))
        .unwrap();
    
    let create_response = app.clone().oneshot(create_request).await.unwrap();
    let create_body = parse_json_body(create_response.into_body()).await;
    let uuid = create_body["uuid"].as_str().unwrap();
    
    // Update the task
    let update_request = Request::builder()
        .method("PUT")
        .uri(format!("/api/tasks/{}", uuid))
        .header("content-type", "application/json")
        .body(Body::from(r#"{"priority":"H","project":"work"}"#))
        .unwrap();
    
    let response = app.clone().oneshot(update_request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = parse_json_body(response.into_body()).await;
    assert_eq!(body["priority"], "H");
    assert_eq!(body["project"], "work");
    
    cleanup_test(app, temp_dir).await;
}

#[tokio::test]
async fn test_delete_task() {
    let (app, temp_dir) = create_test_app().await;
    
    // Create a task first
    let create_request = Request::builder()
        .method("POST")
        .uri("/api/tasks")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"description":"Delete test task"}"#))
        .unwrap();
    
    let create_response = app.clone().oneshot(create_request).await.unwrap();
    let create_body = parse_json_body(create_response.into_body()).await;
    let uuid = create_body["uuid"].as_str().unwrap();
    
    // Delete the task
    let delete_request = Request::builder()
        .method("DELETE")
        .uri(format!("/api/tasks/{}", uuid))
        .body(Body::empty())
        .unwrap();
    
    let response = app.clone().oneshot(delete_request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    
    // After soft-delete, GET should return 404
    let get_request = Request::builder()
        .uri(format!("/api/tasks/{}", uuid))
        .body(Body::empty())
        .unwrap();
    
    let response = app.clone().oneshot(get_request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    
    cleanup_test(app, temp_dir).await;
}

#[tokio::test]
async fn test_list_tasks() {
    let (app, temp_dir) = create_test_app().await;
    
    // Create 3 tasks
    for i in 1..=3 {
        let request = Request::builder()
            .method("POST")
            .uri("/api/tasks")
            .header("content-type", "application/json")
            .body(Body::from(format!(r#"{{"description":"Task {i}"}}"#)))
            .unwrap();
        
        app.clone().oneshot(request).await.unwrap();
    }
    
    // List all tasks
    let list_request = Request::builder()
        .uri("/api/tasks")
        .body(Body::empty())
        .unwrap();
    
    let response = app.clone().oneshot(list_request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = parse_json_body(response.into_body()).await;
    let tasks = body.as_array().unwrap();
    assert!(tasks.len() >= 3);
    
    cleanup_test(app, temp_dir).await;
}

#[tokio::test]
async fn test_list_tasks_filter_status() {
    let (app, temp_dir) = create_test_app().await;
    
    // Create and complete a task
    let create_request = Request::builder()
        .method("POST")
        .uri("/api/tasks")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"description":"Complete me"}"#))
        .unwrap();
    
    let create_response = app.clone().oneshot(create_request).await.unwrap();
    let create_body = parse_json_body(create_response.into_body()).await;
    let uuid = create_body["uuid"].as_str().unwrap();
    
    // Complete the task
    let complete_request = Request::builder()
        .method("POST")
        .uri(format!("/api/tasks/{}/complete", uuid))
        .body(Body::empty())
        .unwrap();
    
    app.clone().oneshot(complete_request).await.unwrap();
    
    // Filter by completed status
    let list_request = Request::builder()
        .uri("/api/tasks?status=completed")
        .body(Body::empty())
        .unwrap();
    
    let response = app.clone().oneshot(list_request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = parse_json_body(response.into_body()).await;
    let tasks = body.as_array().unwrap();
    
    // All tasks should be completed
    for task in tasks {
        assert_eq!(task["status"], "completed");
    }
    
    cleanup_test(app, temp_dir).await;
}

#[tokio::test]
async fn test_list_tasks_sort() {
    let (app, temp_dir) = create_test_app().await;
    
    let request = Request::builder()
        .uri("/api/tasks?sort_by=description")
        .body(Body::empty())
        .unwrap();
    
    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    cleanup_test(app, temp_dir).await;
}

#[tokio::test]
async fn test_list_tasks_invalid_sort() {
    let (app, temp_dir) = create_test_app().await;
    
    let request = Request::builder()
        .uri("/api/tasks?sort_by=invalid")
        .body(Body::empty())
        .unwrap();
    
    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    
    cleanup_test(app, temp_dir).await;
}

#[tokio::test]
async fn test_complete_uncomplete() {
    let (app, temp_dir) = create_test_app().await;
    
    // Create a task
    let create_request = Request::builder()
        .method("POST")
        .uri("/api/tasks")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"description":"Complete cycle test"}"#))
        .unwrap();
    
    let create_response = app.clone().oneshot(create_request).await.unwrap();
    let create_body = parse_json_body(create_response.into_body()).await;
    let uuid = create_body["uuid"].as_str().unwrap();
    
    // Complete the task
    let complete_request = Request::builder()
        .method("POST")
        .uri(format!("/api/tasks/{}/complete", uuid))
        .body(Body::empty())
        .unwrap();
    
    let response = app.clone().oneshot(complete_request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = parse_json_body(response.into_body()).await;
    assert_eq!(body["status"], "completed");
    
    // Uncomplete the task
    let uncomplete_request = Request::builder()
        .method("POST")
        .uri(format!("/api/tasks/{}/uncomplete", uuid))
        .body(Body::empty())
        .unwrap();
    
    let response = app.clone().oneshot(uncomplete_request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = parse_json_body(response.into_body()).await;
    assert_eq!(body["status"], "pending");
    
    cleanup_test(app, temp_dir).await;
}

#[tokio::test]
async fn test_start_stop() {
    let (app, temp_dir) = create_test_app().await;
    
    // Create a task
    let create_request = Request::builder()
        .method("POST")
        .uri("/api/tasks")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"description":"Start/stop test"}"#))
        .unwrap();
    
    let create_response = app.clone().oneshot(create_request).await.unwrap();
    let create_body = parse_json_body(create_response.into_body()).await;
    let uuid = create_body["uuid"].as_str().unwrap();
    
    // Start the task
    let start_request = Request::builder()
        .method("POST")
        .uri(format!("/api/tasks/{}/start", uuid))
        .body(Body::empty())
        .unwrap();
    
    let response = app.clone().oneshot(start_request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = parse_json_body(response.into_body()).await;
    assert_eq!(body["is_active"], true);
    
    // Stop the task
    let stop_request = Request::builder()
        .method("POST")
        .uri(format!("/api/tasks/{}/stop", uuid))
        .body(Body::empty())
        .unwrap();
    
    let response = app.clone().oneshot(stop_request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    
    cleanup_test(app, temp_dir).await;
}

#[tokio::test]
async fn test_working_set() {
    let (app, temp_dir) = create_test_app().await;
    
    // Create a task
    let create_request = Request::builder()
        .method("POST")
        .uri("/api/tasks")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"description":"Working set test"}"#))
        .unwrap();
    
    app.clone().oneshot(create_request).await.unwrap();
    
    // Get working set
    let request = Request::builder()
        .uri("/api/tasks/working-set")
        .body(Body::empty())
        .unwrap();
    
    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = parse_json_body(response.into_body()).await;
    let working_set = body.as_array().unwrap();
    
    // Verify structure has id and task fields
    if !working_set.is_empty() {
        assert!(working_set[0].get("id").is_some());
        assert!(working_set[0].get("task").is_some());
    }
    
    cleanup_test(app, temp_dir).await;
}

#[tokio::test]
async fn test_sync_not_configured() {
    let (app, temp_dir) = create_test_app().await;
    
    let request = Request::builder()
        .method("POST")
        .uri("/api/sync")
        .body(Body::empty())
        .unwrap();
    
    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CONFLICT);
    
    cleanup_test(app, temp_dir).await;
}

#[tokio::test]
async fn test_configure_sync() {
    let (app, temp_dir) = create_test_app().await;
    
    let request = Request::builder()
        .method("POST")
        .uri("/api/sync/configure")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{"server_url":"http://localhost:8080","encryption_secret":"test-secret-key","client_id":"00000000-0000-0000-0000-000000000001"}"#
        ))
        .unwrap();
    
    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = parse_json_body(response.into_body()).await;
    assert_eq!(body["status"], "configured");
    
    cleanup_test(app, temp_dir).await;
}
