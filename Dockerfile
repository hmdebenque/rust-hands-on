# Build stage
FROM rust:1-alpine3.23 AS builder

RUN apk add --no-cache musl-dev openssl-dev pkgconfig

WORKDIR /usr/src/app
COPY . .

ARG FEATURES="in_memory"
RUN cargo install --path . --features "${FEATURES}"

# Runtime stage
FROM alpine:3.23

RUN apk add --no-cache ca-certificates libgcc

COPY --from=builder /usr/local/cargo/bin/todo_api /usr/local/bin/todo_api
COPY --from=builder /usr/src/app/migrations /app/migrations

WORKDIR /app

ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

EXPOSE 3000

CMD ["todo_api"]
