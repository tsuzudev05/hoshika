#!/usr/bin/env bash
# ローカルで Frontend CI と同じ検査を実行するスクリプト
# 使い方: bash scripts/ci-frontend.sh
set -euo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FRONTEND_DIR="$WORKSPACE_ROOT/frontend"

# ── 色付きログ ──────────────────────────────────────────────
info()  { echo -e "\033[1;34m[INFO]\033[0m  $*"; }
ok()    { echo -e "\033[1;32m[OK]\033[0m    $*"; }
fail()  { echo -e "\033[1;31m[FAIL]\033[0m  $*" >&2; }

cd "$FRONTEND_DIR"

# ── npm ci ──────────────────────────────────────────────────
info "npm ci"
if npm ci; then
  ok "install: OK"
else
  fail "install: npm ci に失敗しました。"
  exit 1
fi

# ── 型チェック ───────────────────────────────────────────────
info "npm run type-check"
if npm run type-check; then
  ok "type-check: OK"
else
  fail "type-check: 型エラーがあります。"
  exit 1
fi

# ── lint ────────────────────────────────────────────────────
info "eslint"
if npm run lint; then
  ok "lint: OK"
else
  fail "lint: ESLint エラーがあります。"
  exit 1
fi

# ── build ───────────────────────────────────────────────────
info "vite build"
if npm run build; then
  ok "build: OK"
else
  fail "build: ビルドに失敗しました。"
  exit 1
fi

echo ""
ok "Frontend CI: 全チェック通過 ✅"
