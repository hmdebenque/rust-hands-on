# rust-hands-on

Demo Rust web server project for a presentation

## How to use

This project aims at helping understand using Rust to build a web server.
This is divided in steps with each time an exercise and a solution.

### Getting started

```bash
git checkout handson/1-setup-project
```

### Exercises

Each exercise has a starting branch and a solution branch. Try to complete the exercise on your own, then check the solution if needed.

| Step | Exercise                                                              | Solution                                                     |
|------|-----------------------------------------------------------------------|--------------------------------------------------------------|
| 1    | [`handson/1-setup-project`](../../tree/handson/1-setup-project)       | [`solution`](../../tree/handson/1-setup-project-solution)    |
| 2    | [`handson/2-bootstrap-web`](../../tree/handson/2-bootstrap-web)       | [`solution`](../../tree/handson/2-bootstrap-web-solution)    |
| 3    | [`handson/3-bootstrap-todo`](../../tree/handson/3-bootstrap-todo)     | [`solution`](../../tree/handson/3-bootstrap-todo-solution)   |
| 4    | [`handson/4-postgres-storage`](../../tree/handson/4-postgres-storage) | [`solution`](../../tree/handson/4-postgres-storage-solution) |

### Workflow

1. Checkout the exercise branch: `git checkout handson/1-setup-project`
2. Read the instructions in this README (each step is documented below)
3. Look for `todo!()` macros in the code - these are the parts you need to implement
4. Run `cargo test` to validate your implementation
5. If stuck, compare with the solution: `git diff handson/1-setup-project-solution`
6. Move to the next exercise: `git checkout handson/2-bootstrap-web`

### Tips for students

**Finding what to implement:**
```bash
# Search for todo!() macros in the code
grep -r "todo!()" src/

# See what files changed between exercise and solution
git diff handson/2-bootstrap-web..handson/2-bootstrap-web-solution --stat

# See the actual code diff
git diff handson/2-bootstrap-web..handson/2-bootstrap-web-solution
```

**Validating your work:**
```bash
cargo check          # Fast compile check
cargo test           # Run tests
cargo clippy         # Lint your code
```

**If you're stuck:**
- Check the solution diff: `git diff <exercise-branch>..<solution-branch>`
- Look at specific file: `git show <solution-branch>:src/path/to/file.rs`
- Reset and start fresh: `git checkout -- .`

---

## Step 1 - Scaffolding

**Goal:** Set up the basic Rust project structure.

```bash
git checkout handson/1-setup-project
```

**Tasks:**
- Run `cargo init` to create the project
- Create the module structure (`src/lib.rs`, `src/web/`, `src/storage/`)
- Set up the test directory structure

**Docs:** [cargo init](https://doc.rust-lang.org/cargo/commands/cargo-init.html)

---

## Step 2 - Bootstrap web server

**Goal:** Create a basic Axum web server with a health endpoint.

```bash
git checkout handson/2-bootstrap-web
```

**Tasks:**
1. Create the Axum router in `src/web/routes.rs`
2. Implement the health endpoint handler in `src/web/handlers.rs`
3. Wire up `start_server()` in `src/lib.rs`
4. Make the tests pass

**Docs:** [Axum](https://docs.rs/axum/latest/axum/)

---

## Step 3 - Bootstrap the Todo app

**Goal:** Implement the Todo CRUD operations with in-memory storage.

```bash
git checkout handson/3-bootstrap-todo
```

**Tasks:**
1. Implement `create()` and `get()` in `src/storage/memory.rs`
2. Add the routes to the router
3. Implement the handler logic

**Hint:** Look at the `TodoStorage` trait to understand the expected behavior.

---

## Step 4 - PostgreSQL storage

**Goal:** Implement the same storage interface using PostgreSQL.

```bash
git checkout handson/4-postgres-storage
```

**Tasks:**
1. Implement `create()` and `get()` in `src/storage/postgres.rs`
2. Use SQLx queries to interact with the database
3. Make the integration tests pass

**Running with PostgreSQL:**
```bash
make compose-up      # Start PostgreSQL in Docker
cargo test --features postgres
```

**Docs:** [SQLx](https://docs.rs/sqlx/latest/sqlx/)

---

## Running the project

```bash
# Build and run (in-memory storage)
cargo run

# Run tests
cargo test

# With PostgreSQL (requires Docker)
make compose-up
cargo test --features postgres
```
