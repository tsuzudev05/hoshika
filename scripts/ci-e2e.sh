#!/usr/bin/env bash
# ローカルで E2E (Playwright) CI ジョブと同じ検査を実行するスクリプト
# .github/workflows/frontend.yml の `e2e` ジョブ（ビルド済みバイナリを起動して
# Playwright を走らせる構成）をそのまま再現する。`cargo run` を使う通常の開発フロー
# （DEVELOPMENT.md 参照）とは別に、CI と同じ条件で事前に確認したいときに使う。
# 使い方: bash scripts/ci-e2e.sh
set -euo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FRONTEND_DIR="$WORKSPACE_ROOT/frontend"
BACKEND_LOG="$WORKSPACE_ROOT/backend.log"

# ── 色付きログ ──────────────────────────────────────────────
info()  { echo -e "\033[1;34m[INFO]\033[0m  $*"; }
ok()    { echo -e "\033[1;32m[OK]\033[0m    $*"; }
fail()  { echo -e "\033[1;31m[FAIL]\033[0m  $*" >&2; }

cd "$WORKSPACE_ROOT"

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

# CI の e2e ジョブは APP_ENV を設定せず JWT_SECRET を直接渡している。
# 同じ条件（development フォールバックに頼らない）でローカルでも確認する。
JWT_SECRET="${JWT_SECRET:-ci-e2e-test-secret-do-not-use-in-production}"

# ── バックエンドのビルド ─────────────────────────────────────
info "cargo build --locked"
if cargo build --locked; then
  ok "build: OK"
else
  fail "build: ビルドに失敗しました。"
  exit 1
fi

# ── バックエンドの起動 ───────────────────────────────────────
BACKEND_PID=""
cleanup() {
  if [ -n "$BACKEND_PID" ] && kill -0 "$BACKEND_PID" 2>/dev/null; then
    info "バックエンド (PID $BACKEND_PID) を停止します"
    kill "$BACKEND_PID" 2>/dev/null || true
    wait "$BACKEND_PID" 2>/dev/null || true
  fi
}
trap cleanup EXIT

info "バックエンドを起動 (JWT_SECRET を明示的に設定し、CI と同じ条件で起動)"
DATABASE_URL="$DATABASE_URL" JWT_SECRET="$JWT_SECRET" \
  "$WORKSPACE_ROOT/target/debug/hoshika" > "$BACKEND_LOG" 2>&1 &
BACKEND_PID=$!

backend_ready=false
for i in $(seq 1 60); do
  if curl -sf http://localhost:3000/health > /dev/null; then
    backend_ready=true
    break
  fi
  sleep 2
done

if [ "$backend_ready" = false ]; then
  fail "backend did not become ready in time"
  cat "$BACKEND_LOG"
  exit 1
fi
ok "backend is ready"

# ── フロントエンドの依存関係 ───────────────────────────────────
cd "$FRONTEND_DIR"

info "npm ci"
if npm ci; then
  ok "install: OK"
else
  fail "install: npm ci に失敗しました。"
  exit 1
fi

info "npx playwright install --with-deps chromium"
if npx playwright install --with-deps chromium; then
  ok "playwright install: OK"
else
  fail "playwright install: Chromium のセットアップに失敗しました。"
  exit 1
fi

# ── E2E テスト ─────────────────────────────────────────────
info "npm run test:e2e"
if CI=true npm run test:e2e; then
  ok "e2e: OK"
else
  fail "e2e: テストが失敗しました。"
  fail "バックエンドログ ($BACKEND_LOG):"
  cat "$BACKEND_LOG"
  exit 1
fi

echo ""
ok "E2E (Playwright) CI: 全チェック通過 ✅"
