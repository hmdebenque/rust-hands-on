//! PostgreSQL integration tests using testcontainers
//!
//! Run with: cargo test --features postgres

#![cfg(feature = "postgres")]

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::response::Response;
use sqlx::PgPool;
use testcontainers::ContainerAsync;
use testcontainers_modules::{postgres::Postgres, testcontainers::runners::AsyncRunner};
use todo_api::create_router;
use todo_api::storage::Todo;
use todo_api::storage::postgres::PostgresStorage;
use tower::{Service, ServiceExt};

async fn read_todo_response(response: Response) -> Todo {
    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
    serde_json::from_str(&body_str).unwrap()
}

async fn read_todos_response(response: Response) -> Vec<Todo> {
    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
    serde_json::from_str(&body_str).unwrap()
}

async fn setup_postgres_storage() -> (PostgresStorage, ContainerAsync<Postgres>) {
    let container = Postgres::default()
        .start()
        .await
        .expect("Failed to start PostgreSQL container");

    let host = container.get_host().await.unwrap();
    let port = container.get_host_port_ipv4(5432).await.unwrap();
    let connection_string = format!("postgres://postgres:postgres@{}:{}/postgres", host, port);

    let pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to PostgreSQL");

    let storage = PostgresStorage::new(pool);
    storage
        .init_schema()
        .await
        .expect("Failed to initialize schema");

    (storage, container)
}

#[tokio::test]
async fn test_create_todo_with_postgres() {
    let (storage, _container) = setup_postgres_storage().await;
    let mut app = create_router(storage);

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
    let (storage, _container) = setup_postgres_storage().await;
    let mut app = create_router(storage);

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
async fn test_list_todos_with_postgres() {
    let (storage, _container) = setup_postgres_storage().await;
    let mut app = create_router(storage);

    // Create multiple todos
    for title in ["First", "Second", "Third"] {
        let req = Request::builder()
            .method("POST")
            .uri("/todos")
            .header("Content-Type", "application/json")
            .body(Body::from(format!(r#"{{"title": "{}"}}"#, title)))
            .unwrap();

        <Router as ServiceExt<Request<Body>>>::ready(&mut app)
            .await
            .unwrap();
        app.call(req).await.unwrap();
    }

    // List all todos
    let list_req = Request::builder()
        .method("GET")
        .uri("/todos")
        .body(Body::empty())
        .unwrap();

    let list_response = app.call(list_req).await.unwrap();

    assert_eq!(list_response.status(), StatusCode::OK);
    let todos = read_todos_response(list_response).await;
    assert_eq!(todos.len(), 3);
}

#[tokio::test]
async fn test_update_todo_with_postgres() {
    let (storage, _container) = setup_postgres_storage().await;
    let mut app = create_router(storage);

    // Create a todo
    let create_req = Request::builder()
        .method("POST")
        .uri("/todos")
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"title": "Original Title"}"#))
        .unwrap();

    <Router as ServiceExt<Request<Body>>>::ready(&mut app)
        .await
        .unwrap();
    let create_response = app.call(create_req).await.unwrap();
    let created_todo = read_todo_response(create_response).await;

    // Update the todo
    let update_req = Request::builder()
        .method("PATCH")
        .uri(format!("/todos/{}", created_todo.id))
        .header("Content-Type", "application/json")
        .body(Body::from(
            r#"{"title": "Updated Title", "completed": true}"#,
        ))
        .unwrap();

    let update_response = app.call(update_req).await.unwrap();

    assert_eq!(update_response.status(), StatusCode::OK);
    let updated_todo = read_todo_response(update_response).await;
    assert_eq!(updated_todo.title, "Updated Title");
    assert!(updated_todo.completed);
}

#[tokio::test]
async fn test_delete_todo_with_postgres() {
    let (storage, _container) = setup_postgres_storage().await;
    let mut app = create_router(storage);

    // Create a todo
    let create_req = Request::builder()
        .method("POST")
        .uri("/todos")
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"title": "To Be Deleted"}"#))
        .unwrap();

    <Router as ServiceExt<Request<Body>>>::ready(&mut app)
        .await
        .unwrap();
    let create_response = app.call(create_req).await.unwrap();
    let created_todo = read_todo_response(create_response).await;

    // Delete the todo
    let delete_req = Request::builder()
        .method("DELETE")
        .uri(format!("/todos/{}", created_todo.id))
        .body(Body::empty())
        .unwrap();

    let delete_response = app.call(delete_req).await.unwrap();
    assert_eq!(delete_response.status(), StatusCode::NO_CONTENT);

    // Verify it's gone
    let get_req = Request::builder()
        .method("GET")
        .uri(format!("/todos/{}", created_todo.id))
        .body(Body::empty())
        .unwrap();

    let get_response = app.call(get_req).await.unwrap();
    assert_eq!(get_response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_nonexistent_todo_returns_404() {
    let (storage, _container) = setup_postgres_storage().await;
    let mut app = create_router(storage);

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
