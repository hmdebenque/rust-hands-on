pub mod handlers;

use axum::Router;
use axum::routing::post;

// pub mod handlers;
// pub mod models;
// pub mod repository;

pub fn app() -> Router {
    Router::new().route("/todos", post(handlers::create_todo))
}
