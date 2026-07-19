#!/usr/bin/env bash
set -e

echo "🔧 hoshika 開発環境セットアップ"

# .env
if [ ! -f .env ]; then
    cp .env.example .env
    echo "✅ .env を作成しました（.env.example からコピー）"
else
    echo "ℹ️  .env は既に存在します"
fi

# .devcontainer/.env
if [ ! -f .devcontainer/.env ]; then
    cp .devcontainer/.env.example .devcontainer/.env
    echo "✅ .devcontainer/.env を作成しました（.env.example からコピー）"
else
    echo "ℹ️  .devcontainer/.env は既に存在します"
fi

# frontend/.env
if [ ! -f frontend/.env ]; then
    cp frontend/.env.example frontend/.env
    echo "✅ frontend/.env を作成しました（frontend/.env.example からコピー）"
else
    echo "ℹ️  frontend/.env は既に存在します"
fi

echo ""
echo "✅ セットアップ完了！IDE で「Reopen in Container」を実行してください。"
