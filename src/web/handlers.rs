use crate::storage::StorageError::Database;
use crate::storage::{CreateTodo, StorageError, Todo, TodoStorage, UpdateTodo};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState<S: TodoStorage> {
    pub storage: S,
}

pub enum AppError {
    NotFound,
    Storage(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Todo not found"),
            AppError::Storage(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.leak() as &str),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

impl From<StorageError> for AppError {
    fn from(err: StorageError) -> Self {
        match err {
            crate::storage::StorageError::NotFound => AppError::NotFound,
            crate::storage::StorageError::Database(msg) => AppError::Storage(msg),
        }
    }
}

pub async fn create_todo<S: TodoStorage>(
    State(state): State<AppState<S>>,
    Json(create): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), AppError> {
    let todo = state.storage.create(create).await?;
    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn get_todo<S: TodoStorage>(
    State(state): State<AppState<S>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Todo>, AppError> {
    let todo = state.storage.get(id).await?;
    Ok(Json(todo))
}

pub async fn list_todos<S: TodoStorage>(
    State(state): State<AppState<S>>,
) -> Result<Json<Vec<Todo>>, AppError> {
    let todos = state.storage.list().await?;
    Ok(Json(todos))
}

pub async fn update_todo<S: TodoStorage>(
    State(state): State<AppState<S>>,
    Path(id): Path<Uuid>,
    Json(update): Json<UpdateTodo>,
) -> Result<Json<Todo>, AppError> {
    let todo = state.storage.update(id, update).await?;
    Ok(Json(todo))
}

pub async fn delete_todo<S: TodoStorage>(
    State(state): State<AppState<S>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    state.storage.delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
