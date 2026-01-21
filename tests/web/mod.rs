use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::response::Response;
use todo_api::create_router;
use todo_api::storage::memory::MemoryStorage;
use todo_api::storage::Todo;
use tower::{Service, ServiceExt};

async fn read_resp(response: Response) -> Todo {
    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
    serde_json::from_str(&body_str).unwrap()
}

#[tokio::test]
async fn test_create_todo() {
    // given
    let storage = MemoryStorage::new();
    let mut router = create_router(storage);

    let req = Request::builder()
        .method("POST")
        .uri("/todos")
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"title": "Web layer test"}"#))
        .unwrap();
    <Router as ServiceExt<Request<Body>>>::ready(&mut router)
        .await
        .unwrap();

    // when
    let response = router.call(req).await.unwrap();

    // then
    assert_eq!(response.status(), StatusCode::CREATED);
    let todo = read_resp(response).await;
    assert_eq!(todo.title, "Web layer test");
    assert!(!todo.completed);
}

#[tokio::test]
async fn test_get_nonexistent_todo_returns_404() {
    // given
    let storage = MemoryStorage::new();
    let mut router = create_router(storage);

    let req = Request::builder()
        .method("GET")
        .uri("/todos/00000000-0000-0000-0000-000000000000")
        .body(Body::empty())
        .unwrap();
    <Router as ServiceExt<Request<Body>>>::ready(&mut router)
        .await
        .unwrap();

    // when
    let response = router.call(req).await.unwrap();

    // then
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_health() {
    // given
    let storage = MemoryStorage::new();
    let mut router = create_router(storage);

    let req = Request::builder()
        .method("GET")
        .uri("/health")
        .body(Body::empty())
        .unwrap();
    <Router as ServiceExt<Request<Body>>>::ready(&mut router)
        .await
        .unwrap();

    // when
    let response = router.call(req).await.unwrap();

    // then
    assert_eq!(response.status(), StatusCode::OK);
}
