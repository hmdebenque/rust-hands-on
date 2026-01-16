use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::response::Response;
use todo_api::app;
use todo_api::storage::Todo;
use tower::{Service, ServiceExt};

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
    // wait for app to be ready
    <Router as ServiceExt<Request<Body>>>::ready(&mut app)
        .await
        .unwrap();

    // when
    let response = app.call(req).await.unwrap();

    // then
    assert_eq!(response.status(), StatusCode::CREATED);
    let created_todo = read_resp(response).await;
    println!("Response received: {:?}", created_todo);

    // and when
    // Fetch the created todo
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

async fn read_resp(get_response: Response) -> Todo {
    let get_body_bytes = axum::body::to_bytes(get_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let get_body_str = String::from_utf8(get_body_bytes.to_vec()).unwrap();
    let fetched_todo: Todo = serde_json::from_str(&get_body_str).unwrap();
    fetched_todo
}
