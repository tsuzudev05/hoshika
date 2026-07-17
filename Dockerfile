# syntax=docker/dockerfile:1

# --- stage 1: フロントエンドのビルド ---
FROM node:20-alpine AS frontend-build
WORKDIR /app/frontend
COPY frontend/package.json frontend/package-lock.json ./
RUN npm ci
COPY frontend/ ./
RUN npm run build

# --- stage 2: バックエンドのビルド ---
# sqlx はここで sqlx::query()（実行時チェック）のみを使用しており、
# sqlx::query! 系のコンパイル時チェックマクロは使っていないため、
# ビルド時に DATABASE_URL や稼働中の DB は不要。
FROM rust:1-slim AS rust-build
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/
COPY migrations/ ./migrations/
RUN cargo build --release --locked

# --- stage 3: 実行用イメージ ---
FROM debian:bookworm-slim
RUN apt-get update -qq \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=rust-build /app/target/release/hoshika ./hoshika
COPY --from=frontend-build /app/frontend/dist ./frontend/dist

ENV STATIC_DIR=/app/frontend/dist
EXPOSE 3000

CMD ["./hoshika"]
