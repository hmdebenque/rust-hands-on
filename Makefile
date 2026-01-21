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

# Clean
clean:
	cargo clean
