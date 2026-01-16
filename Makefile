.PHONY: build build-postgres build-all test test-postgres test-all check clippy fmt clean \
        migrate migrate-create migrate-revert migrate-info sqlx-prepare \
        docker-build docker-build-postgres docker-run \
        compose-up compose-down compose-logs compose-build

# Build commands
build:
	cargo build

build-postgres:
	cargo build --features postgres

build-all:
	cargo build --all-features

build-release:
	cargo build --release --all-features

# Test commands
test:
	cargo test

test-postgres:
	cargo test --features postgres

test-all:
	cargo test --all-features

# Code quality
check:
	cargo check --all-features

clippy:
	cargo clippy --all-features -- -D warnings

fmt:
	cargo fmt

fmt-check:
	cargo fmt -- --check

# Database migrations (requires: cargo install sqlx-cli)
migrate:
	sqlx migrate run

migrate-create:
	@read -p "Migration name: " name; \
	sqlx migrate add -r $$name

migrate-revert:
	sqlx migrate revert

migrate-info:
	sqlx migrate info

sqlx-prepare:
	cargo sqlx prepare --workspace

# Docker
docker-build:
	docker build -t todo_api .

docker-build-postgres:
	docker build --build-arg FEATURES=postgres -t todo_api:postgres .

docker-run:
	docker run -p 3000:3000 todo_api

# Docker Compose
compose-up:
	docker compose up -d

compose-down:
	docker compose down

compose-logs:
	docker compose logs -f

compose-build:
	docker compose build

# Clean
clean:
	cargo clean