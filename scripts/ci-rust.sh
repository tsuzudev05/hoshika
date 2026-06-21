#!/usr/bin/env bash
# ローカルで Rust CI と同じ検査を実行するスクリプト
# 使い方: bash scripts/ci-rust.sh
set -euo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$WORKSPACE_ROOT"

# ── 色付きログ ──────────────────────────────────────────────
info()  { echo -e "\033[1;34m[INFO]\033[0m  $*"; }
ok()    { echo -e "\033[1;32m[OK]\033[0m    $*"; }
fail()  { echo -e "\033[1;31m[FAIL]\033[0m  $*" >&2; }

# ── DATABASE_URL の確認 ──────────────────────────────────────
if [ -z "${DATABASE_URL:-}" ]; then
  if [ -f .devcontainer/.env ]; then
    # shellcheck disable=SC1091
    set -a; source .devcontainer/.env; set +a
    info "DATABASE_URL を .devcontainer/.env から読み込みました"
  else
    fail "DATABASE_URL が未設定です。.devcontainer/.env を確認してください。"
    exit 1
  fi
fi

info "DATABASE_URL: $DATABASE_URL"

# ── DB 接続確認 ─────────────────────────────────────────────
info "DB 接続確認中..."
DB_HOST=$(echo "$DATABASE_URL" | sed -E 's|.*@([^:/]+).*|\1|')
DB_PORT=$(echo "$DATABASE_URL" | sed -E 's|.*:([0-9]+)/.*|\1|')
connected=false
for i in $(seq 1 15); do
  if pg_isready -h "$DB_HOST" -p "$DB_PORT" -q 2>/dev/null; then
    connected=true
    break
  fi
  info "DB 待機中... ($i/15)"
  sleep 2
done
if [ "$connected" = false ]; then
  fail "DB ($DB_HOST:$DB_PORT) に接続できません。"
  fail "IDE で「Rebuild Container」を実行して DevContainer を再起動してください。"
  exit 1
fi
ok "DB 接続: OK"

# ── マイグレーション ─────────────────────────────────────────
info "マイグレーション実行中..."
if command -v sqlx &>/dev/null; then
  sqlx migrate run
  ok "マイグレーション完了"
else
  info "sqlx-cli が見つかりません。インストールします..."
  cargo install sqlx-cli \
    --version 0.7.4 \
    --no-default-features \
    --features rustls,postgres \
    --locked
  sqlx migrate run
  ok "マイグレーション完了"
fi

# ── cargo fmt ───────────────────────────────────────────────
info "cargo fmt --check"
if cargo fmt --all -- --check; then
  ok "fmt: OK"
else
  fail "fmt: フォーマットが崩れています。'cargo fmt' を実行してください。"
  exit 1
fi

# ── cargo clippy ────────────────────────────────────────────
info "cargo clippy"
if cargo clippy --all-targets --all-features -- -D warnings; then
  ok "clippy: OK"
else
  fail "clippy: 警告があります。修正してください。"
  exit 1
fi

# ── cargo build ─────────────────────────────────────────────
info "cargo build"
if cargo build --locked; then
  ok "build: OK"
else
  fail "build: ビルドに失敗しました。"
  exit 1
fi

# ── cargo test ──────────────────────────────────────────────
info "cargo test"
if cargo test --locked; then
  ok "test: OK"
else
  fail "test: テストに失敗しました。"
  exit 1
fi

echo ""
ok "Rust CI: 全チェック通過 ✅"
