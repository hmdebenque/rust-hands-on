//! PostgresStorage unit tests
//!
//! Tests the storage layer directly without HTTP.
//! Run with: cargo test --features postgres

#![cfg(feature = "postgres")]

use sqlx::PgPool;
use testcontainers::ContainerAsync;
use testcontainers_modules::{postgres::Postgres, testcontainers::runners::AsyncRunner};
use todo_api::storage::postgres::PostgresStorage;
use todo_api::storage::{CreateTodo, TodoStorage, UpdateTodo};
use uuid::Uuid;

async fn setup_storage() -> (PostgresStorage, ContainerAsync<Postgres>) {
    let container = Postgres::default()
        .start()
        .await
        .expect("Failed to start PostgreSQL container");

    let host = container.get_host().await.unwrap();
    let port = container.get_host_port_ipv4(5432).await.unwrap();
    let connection_string = format!("postgres://postgres:postgres@{}:{}/postgres", host, port);

    let pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to PostgreSQL");

    let storage = PostgresStorage::new(pool);
    storage
        .run_migrations()
        .await
        .expect("Failed to run migrations");

    (storage, container)
}

#[tokio::test]
async fn test_create_todo() {
    let (storage, _container) = setup_storage().await;

    let todo = storage
        .create(CreateTodo {
            title: "Test todo".to_string(),
        })
        .await
        .unwrap();

    assert_eq!(todo.title, "Test todo");
    assert!(!todo.completed);
}

#[tokio::test]
async fn test_get_todo() {
    let (storage, _container) = setup_storage().await;

    let created = storage
        .create(CreateTodo {
            title: "Get me".to_string(),
        })
        .await
        .unwrap();

    let fetched = storage.get(created.id).await.unwrap();

    assert_eq!(fetched.id, created.id);
    assert_eq!(fetched.title, "Get me");
}

#[tokio::test]
async fn test_get_nonexistent_todo_returns_not_found() {
    let (storage, _container) = setup_storage().await;

    let result = storage.get(Uuid::nil()).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_todos() {
    let (storage, _container) = setup_storage().await;

    storage
        .create(CreateTodo {
            title: "First".to_string(),
        })
        .await
        .unwrap();
    storage
        .create(CreateTodo {
            title: "Second".to_string(),
        })
        .await
        .unwrap();

    let todos = storage.list().await.unwrap();

    assert_eq!(todos.len(), 2);
}

#[tokio::test]
async fn test_list_todos_ordered_by_title() {
    let (storage, _container) = setup_storage().await;

    storage
        .create(CreateTodo {
            title: "Zebra".to_string(),
        })
        .await
        .unwrap();
    storage
        .create(CreateTodo {
            title: "Apple".to_string(),
        })
        .await
        .unwrap();

    let todos = storage.list().await.unwrap();

    assert_eq!(todos[0].title, "Apple");
    assert_eq!(todos[1].title, "Zebra");
}

#[tokio::test]
async fn test_update_title_only() {
    let (storage, _container) = setup_storage().await;

    let created = storage
        .create(CreateTodo {
            title: "Original".to_string(),
        })
        .await
        .unwrap();

    let updated = storage
        .update(
            created.id,
            UpdateTodo {
                title: Some("Updated".to_string()),
                completed: None,
            },
        )
        .await
        .unwrap();

    assert_eq!(updated.title, "Updated");
    assert!(!updated.completed);
}

#[tokio::test]
async fn test_update_completed_only() {
    let (storage, _container) = setup_storage().await;

    let created = storage
        .create(CreateTodo {
            title: "Task".to_string(),
        })
        .await
        .unwrap();

    let updated = storage
        .update(
            created.id,
            UpdateTodo {
                title: None,
                completed: Some(true),
            },
        )
        .await
        .unwrap();

    assert_eq!(updated.title, "Task");
    assert!(updated.completed);
}

#[tokio::test]
async fn test_update_both_fields() {
    let (storage, _container) = setup_storage().await;

    let created = storage
        .create(CreateTodo {
            title: "Original".to_string(),
        })
        .await
        .unwrap();

    let updated = storage
        .update(
            created.id,
            UpdateTodo {
                title: Some("New title".to_string()),
                completed: Some(true),
            },
        )
        .await
        .unwrap();

    assert_eq!(updated.title, "New title");
    assert!(updated.completed);
}

#[tokio::test]
async fn test_update_with_no_fields_returns_unchanged() {
    let (storage, _container) = setup_storage().await;

    let created = storage
        .create(CreateTodo {
            title: "Unchanged".to_string(),
        })
        .await
        .unwrap();

    let updated = storage
        .update(
            created.id,
            UpdateTodo {
                title: None,
                completed: None,
            },
        )
        .await
        .unwrap();

    assert_eq!(updated.title, "Unchanged");
    assert!(!updated.completed);
}

#[tokio::test]
async fn test_update_nonexistent_todo_returns_not_found() {
    let (storage, _container) = setup_storage().await;

    let result = storage
        .update(
            Uuid::nil(),
            UpdateTodo {
                title: Some("Doesn't matter".to_string()),
                completed: None,
            },
        )
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_todo() {
    let (storage, _container) = setup_storage().await;

    let created = storage
        .create(CreateTodo {
            title: "Delete me".to_string(),
        })
        .await
        .unwrap();

    storage.delete(created.id).await.unwrap();

    let result = storage.get(created.id).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_todo_returns_not_found() {
    let (storage, _container) = setup_storage().await;

    let result = storage.delete(Uuid::nil()).await;

    assert!(result.is_err());
}
