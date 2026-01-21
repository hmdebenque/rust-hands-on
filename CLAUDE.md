# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Rust TODO API demonstrating production-ready web development with Axum. Uses feature flags to swap between in-memory storage (default) and PostgreSQL backends.

## Build Commands

```bash
# Building
make build              # Debug build (in_memory)
make build-postgres     # Build with PostgreSQL feature
make build-all          # Build with all features
make build-release      # Release build

# Testing
make test               # In-memory tests
make test-postgres      # PostgreSQL integration tests (uses testcontainers)
make test-all           # All tests

# Code quality
make fmt                # Format with rustfmt
make clippy             # Lint (warnings as errors)
make check              # cargo check with all features

# Database migrations (requires sqlx-cli)
make migrate            # Run pending migrations
make migrate-create     # Create new migration

# Docker
make compose-up         # Start app + PostgreSQL
make compose-down       # Stop services
make compose-logs       # Stream logs
```

## Architecture

```
src/
├── main.rs           # Entry point
├── lib.rs            # Core logic, app initialization, logging setup
├── storage/          # Storage abstraction layer
│   ├── mod.rs        # TodoStorage trait, DTOs (Todo, CreateTodo, UpdateTodo)
│   ├── memory.rs     # In-memory impl: Arc<RwLock<HashMap<Uuid, Todo>>>
│   └── postgres.rs   # PostgreSQL impl with SQLx and migrations
└── web/              # HTTP layer
    ├── routes.rs     # Axum router setup
    └── handlers.rs   # HTTP handlers, AppState definition
```

**Key patterns:**
- `TodoStorage` trait enables compile-time backend selection via features
- Handlers are generic over `S: TodoStorage + Clone + 'static`
- Error handling: `StorageError` → `AppError` → HTTP response
- SQLx compile-time checked queries with `query_as!` macro

## API Endpoints

```
GET    /health        # Health check (200)
POST   /todos         # Create todo (201)
GET    /todos         # List all
GET    /todos/{id}    # Get by ID
PATCH  /todos/{id}    # Partial update
DELETE /todos/{id}    # Delete (204)
```

## Environment Variables

- `DATABASE_URL` - PostgreSQL connection string (required for postgres feature)
- `MYAPP_LOG` - Log level filter (default: `info`)

## Running Locally

```bash
# In-memory (development)
cargo run

# With PostgreSQL
export DATABASE_URL=postgres://user:pass@localhost/todo_api
cargo run --features postgres

# Full stack via Docker Compose
make compose-up
```

Server runs on `http://localhost:3000`.
