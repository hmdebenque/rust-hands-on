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
2. Read the instructions in the README or code comments
3. Implement the solution
4. Compare with the solution: `git diff handson/1-setup-project-solution`
5. Move to the next exercise: `git checkout handson/2-bootstrap-web`

### Running the project

```bash
# Build and run (in-memory storage)
cargo run

# Run tests
cargo test

# With PostgreSQL (requires Docker)
make compose-up
```
