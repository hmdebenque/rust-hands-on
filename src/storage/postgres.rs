use super::{CreateTodo, Result, StorageError, Todo, TodoStorage};
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PostgresStorage {
    pool: PgPool,
}

impl PostgresStorage {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Run all pending database migrations.
    /// Migrations are embedded at compile time from the `migrations/` directory.
    pub async fn run_migrations(&self) -> std::result::Result<(), sqlx::migrate::MigrateError> {
        sqlx::migrate!("./migrations").run(&self.pool).await
    }
}

#[async_trait]
impl TodoStorage for PostgresStorage {
    async fn create(&self, create: CreateTodo) -> Result<Todo> {
        let id = Uuid::new_v4();

        sqlx::query_as::<_, Todo>(
            "INSERT INTO todos (id, title, completed) VALUES ($1, $2, false) RETURNING *",
        )
        .bind(id)
        .bind(create.title)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| StorageError::Database(e.to_string()))
    }

    async fn get(&self, id: Uuid) -> Result<Todo> {
        sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?
            .ok_or(StorageError::NotFound)
    }
}
