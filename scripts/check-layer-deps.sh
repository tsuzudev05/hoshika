#!/usr/bin/env bash
# Clean Architecture のレイヤー依存ルールを検証するスクリプト
#
# ルール:
#   domain/      → axum / sqlx を import してはいけない
#   application/ → axum を import してはいけない
#
# 使い方:
#   ./scripts/check-layer-deps.sh
#
# CI での使い方（違反があれば exit 1）:
#   bash scripts/check-layer-deps.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SRC_DIR="$(cd "$SCRIPT_DIR/../src" && pwd)"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

violations=0

check() {
    local layer="$1"      # domain / application
    local forbidden="$2"  # axum|sqlx など
    local label="$3"      # 表示用ラベル

    local target="$SRC_DIR/$layer"
    if [ ! -d "$target" ]; then
        echo -e "${YELLOW}SKIP${NC}  $layer/ ディレクトリが見つかりません"
        return
    fi

    local matches
    matches=$(grep -rn --include="*.rs" -E "$forbidden" "$target" 2>/dev/null || true)

    if [ -n "$matches" ]; then
        echo -e "${RED}FAIL${NC}  $layer/ が $label に依存しています"
        echo "$matches" | sed 's/^/       /'
        violations=$((violations + 1))
    else
        echo -e "${GREEN}OK${NC}    $layer/ は $label に依存していません"
    fi
}

echo "=== Layer dependency check ==="
echo ""

check "domain"      "axum|sqlx"  "axum / sqlx"
check "application" "axum"       "axum"

echo ""

if [ "$violations" -eq 0 ]; then
    echo -e "${GREEN}All checks passed.${NC}"
    exit 0
else
    echo -e "${RED}$violations violation(s) found.${NC}"
    exit 1
fi
