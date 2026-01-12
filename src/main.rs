use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = todo_api::app(); // ← from lib.rs
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}