//! PostgreSQL integration tests using testcontainers
//!
//! Run with: cargo test --features postgres

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::response::Response;
use testcontainers::ContainerAsync;
use testcontainers_modules::{postgres::Postgres, testcontainers::runners::AsyncRunner};
use todo_api::app_postgres;
use todo_api::data_types::Todo;
use tower::{Service, ServiceExt};

async fn read_todo_response(response: Response) -> Todo {
    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
    serde_json::from_str(&body_str).unwrap()
}

async fn setup_app() -> (Router, ContainerAsync<Postgres>) {
    let container = Postgres::default()
        .start()
        .await
        .expect("Failed to start PostgreSQL container");

    let host = container.get_host().await.unwrap();
    let port = container.get_host_port_ipv4(5432).await.unwrap();
    let database_url = format!("postgres://postgres:postgres@{}:{}/postgres", host, port);

    let app = app_postgres(&database_url).await;

    (app, container)
}

#[tokio::test]
async fn test_create_todo_with_postgres() {
    let (mut app, _container) = setup_app().await;

    let req = Request::builder()
        .method("POST")
        .uri("/todos")
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"title": "Learn Rust with Postgres"}"#))
        .unwrap();

    <Router as ServiceExt<Request<Body>>>::ready(&mut app)
        .await
        .unwrap();
    let response = app.call(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let created_todo = read_todo_response(response).await;
    assert_eq!(created_todo.title, "Learn Rust with Postgres");
    assert!(!created_todo.completed);
}

#[tokio::test]
async fn test_get_todo_with_postgres() {
    let (mut app, _container) = setup_app().await;

    // Create a todo first
    let create_req = Request::builder()
        .method("POST")
        .uri("/todos")
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"title": "Test Get"}"#))
        .unwrap();

    <Router as ServiceExt<Request<Body>>>::ready(&mut app)
        .await
        .unwrap();
    let create_response = app.call(create_req).await.unwrap();
    let created_todo = read_todo_response(create_response).await;

    // Get the todo
    let get_req = Request::builder()
        .method("GET")
        .uri(format!("/todos/{}", created_todo.id))
        .body(Body::empty())
        .unwrap();

    let get_response = app.call(get_req).await.unwrap();

    assert_eq!(get_response.status(), StatusCode::OK);
    let fetched_todo = read_todo_response(get_response).await;
    assert_eq!(fetched_todo.id, created_todo.id);
    assert_eq!(fetched_todo.title, "Test Get");
}

#[tokio::test]
async fn test_get_nonexistent_todo_returns_404() {
    let (mut app, _container) = setup_app().await;

    let req = Request::builder()
        .method("GET")
        .uri("/todos/00000000-0000-0000-0000-000000000000")
        .body(Body::empty())
        .unwrap();

    <Router as ServiceExt<Request<Body>>>::ready(&mut app)
        .await
        .unwrap();
    let response = app.call(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
