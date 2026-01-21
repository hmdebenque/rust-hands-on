use crate::web::handlers::{create_todo, get_todo, health};
use axum::{
    Router,
    routing::{get, post},
};
use crate::storage::TodoStorage;

#[derive(Clone)]
pub struct AppState<S: TodoStorage> {
    pub storage: S,
}

pub enum AppError {
    NotFound,
    Storage(String),
}

pub fn create_router<S: TodoStorage + Clone + 'static>(storage: S) -> Router {
    // todo: add state and routes
    Router::new()
        .route("/health", get(health))
}
