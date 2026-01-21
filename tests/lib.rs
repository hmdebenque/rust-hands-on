mod storage;
mod web;

#[cfg(not(feature = "postgres"))]
use axum::Router;
#[cfg(not(feature = "postgres"))]
use axum::body::Body;
#[cfg(not(feature = "postgres"))]
use axum::http::{Request, StatusCode};
#[cfg(not(feature = "postgres"))]
use axum::response::Response;
#[cfg(not(feature = "postgres"))]
use todo_api::app;
#[cfg(not(feature = "postgres"))]
use todo_api::storage::Todo;
#[cfg(not(feature = "postgres"))]
use tower::{Service, ServiceExt};

#[cfg(not(feature = "postgres"))]
async fn read_resp(response: Response) -> Todo {
    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
    serde_json::from_str(&body_str).unwrap()
}

#[cfg(not(feature = "postgres"))]
#[tokio::test]
async fn test_create_todo_returns_201() {
    // given
    let mut app = app().await;
    let req = Request::builder()
        .method("POST")
        .uri("/todos")
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"title": "Learn Rust"}"#))
        .unwrap();
    <Router as ServiceExt<Request<Body>>>::ready(&mut app)
        .await
        .unwrap();

    // when
    let response = app.call(req).await.unwrap();

    // then
    assert_eq!(response.status(), StatusCode::CREATED);
    let created_todo = read_resp(response).await;
    println!("Response received: {:?}", created_todo);

    // and when - fetch the created todo
    let get_todo_req = Request::builder()
        .method("GET")
        .uri(format!("/todos/{}", created_todo.id))
        .body(Body::empty())
        .unwrap();
    let get_response = app.call(get_todo_req).await.unwrap();

    // then
    assert_eq!(get_response.status(), StatusCode::OK);
    let fetched_todo = read_resp(get_response).await;
    assert_eq!(fetched_todo.id, created_todo.id);
    assert_eq!(fetched_todo.title, "Learn Rust");
}

#[cfg(not(feature = "postgres"))]
#[tokio::test]
async fn test_health_returns_200() {
    // given
    let mut app = app().await;
    let req = Request::builder()
        .method("GET")
        .uri("/health")
        .body(Body::empty())
        .unwrap();
    <Router as ServiceExt<Request<Body>>>::ready(&mut app)
        .await
        .unwrap();

    // when
    let response = app.call(req).await.unwrap();

    // then
    assert_eq!(response.status(), StatusCode::OK);
}
