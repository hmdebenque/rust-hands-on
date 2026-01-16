use crate::storage::TodoStorage;
use crate::web::handlers::{AppState, create_todo, delete_todo, get_todo, list_todos, update_todo};
use axum::{
    Router,
    routing::{get, post},
};
use tower_http::cors::CorsLayer;

pub fn create_router<S: TodoStorage + Clone + 'static>(storage: S) -> Router {
    let state = AppState { storage };

    Router::new()
        .route("/todos", post(create_todo::<S>).get(list_todos::<S>))
        .route(
            "/todos/{id}",
            get(get_todo::<S>)
                .patch(update_todo::<S>)
                .delete(delete_todo::<S>),
        )
        .layer(CorsLayer::permissive())
        .with_state(state)
}
