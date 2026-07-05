#!/usr/bin/env bash
# ローカル開発用に budgets / wish_items へサンプルデータを投入するスクリプト
# 使い方: bash scripts/seed-dev-data.sh
#
# 予算は「実行時点の年月」に対して投入する（budgets.year/month は UNIQUE のため再実行しても重複しない）。
# wish_items は name で重複チェックしているので、これも再実行して安全。
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

YEAR=$(date +%Y)
MONTH=$(date +%-m)
BUDGET_AMOUNT="${BUDGET_AMOUNT:-50000}"

info "投入対象: ${YEAR}年${MONTH}月 / 予算 ${BUDGET_AMOUNT}円"

psql "$DATABASE_URL" -v ON_ERROR_STOP=1 <<EOF
-- 予算（実行時点の年月。新規作成時は amount = balance）
INSERT INTO budgets (year, month, amount, balance)
VALUES (${YEAR}, ${MONTH}, ${BUDGET_AMOUNT}, ${BUDGET_AMOUNT})
ON CONFLICT (year, month) DO NOTHING;

-- 欲しいものリスト（name で重複チェックしているので再実行しても増えない）
INSERT INTO wish_items (name, price, category_id, memo, status)
SELECT 'リーダブルコード', 2400, id, 'チーム内で話題', 'Inbox'
FROM categories WHERE name = '書籍'
AND NOT EXISTS (SELECT 1 FROM wish_items WHERE name = 'リーダブルコード');

INSERT INTO wish_items (name, price, category_id, memo, status)
SELECT 'メカニカルキーボード', 15800, id, '', 'Inbox'
FROM categories WHERE name = 'ガジェット'
AND NOT EXISTS (SELECT 1 FROM wish_items WHERE name = 'メカニカルキーボード');

INSERT INTO wish_items (name, price, category_id, memo, status)
SELECT 'ノイズキャンセリングイヤホン', 32000, id, '出張用', 'NextToBuy'
FROM categories WHERE name = 'ガジェット'
AND NOT EXISTS (SELECT 1 FROM wish_items WHERE name = 'ノイズキャンセリングイヤホン');

INSERT INTO wish_items (name, price, category_id, memo, status)
SELECT '型落ちスニーカー', 8900, id, '', 'Archived'
FROM categories WHERE name = 'ファッション'
AND NOT EXISTS (SELECT 1 FROM wish_items WHERE name = '型落ちスニーカー');
EOF

ok "シードデータの投入が完了しました"
