use axum::http::StatusCode;
use axum::{
    Json,
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use serde_json::json;
use uuid::Uuid;
use crate::data_types::{CreateTodo, Todo};
use crate::storage::{StorageError, TodoStorage};
use crate::web::routes::{AppError, AppState};


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
            StorageError::NotFound => AppError::NotFound,
            StorageError::Database(msg) => AppError::Storage(msg),
        }
    }
}

pub async fn health() -> StatusCode {
    StatusCode::OK
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
