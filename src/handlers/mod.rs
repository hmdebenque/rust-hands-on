use axum::http::StatusCode;
use axum::{Json, Router};
use axum::routing::post;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Todo { id: Uuid, title: String, completed: bool }

#[derive(Deserialize)]
pub struct CreateTodo { title: String }

// 🎭 Mock implementation - just returns fake data
pub async fn create_todo(Json(input): Json<CreateTodo>) -> (StatusCode, Json<Todo>) {
    let todo = Todo { id: Uuid::new_v4(), title: input.title, completed: false };
    (StatusCode::CREATED, Json(todo))
}

pub fn app() -> Router { Router::new().route("/todos", post(create_todo)) }
