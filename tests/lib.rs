mod storage;
mod web;

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use todo_api::app;
use tower::{Service, ServiceExt};

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
