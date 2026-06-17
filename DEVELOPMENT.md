# DEVELOPMENT.md

開発環境のセットアップと動作確認手順。

---

## 前提条件

| ツール | 用途 |
|---|---|
| [Docker Desktop](https://www.docker.com/products/docker-desktop/) | DevContainer の実行に必須 |
| VS Code または Zed | DevContainer サポートのある IDE |

> Docker Desktop が起動していないと DevContainer は起動しません。タスクバーのクジラアイコンが静止していることを確認してください。

---

## セットアップ

### 1. DevContainer を起動する

```
IDE で「Reopen in Container」を実行
```

初回は以下が自動で行われます（数分かかります）：

- `mcr.microsoft.com/devcontainers/base:ubuntu-22.04` のビルド
- Rust / Node 20 / GitHub CLI のインストール
- `cargo build`（依存クレートのダウンロード）
- `npm install`（フロントエンド依存）

### 2. 環境変数を設定する

コンテナ内で `devcontainer.json` の `remoteEnv` により `DATABASE_URL` は自動で設定済みです。

ローカル（コンテナ外）で動かす場合は `.env` を作成します：

```bash
cp .env.example .env
```

---

## 動作確認

### DevContainer 全体

コンテナ内のターミナルで以下を実行：

```bash
rustup --version    # Rust がインストールされているか
node --version      # Node 20 がインストールされているか
gh --version        # GitHub CLI がインストールされているか
```

### PostgreSQL（DB コンテナ）

```bash
psql -h db -U hoshika hoshika -c '\dt'
```

以下のテーブルが表示されれば OK：

```
 categories
 purchase_records
 wish_items
 budgets
```

> マイグレーションはサーバー起動時（`cargo run`）に自動実行されます。

### バックエンド（Rust / Axum）

```bash
# サーバー起動（ポート 3000）
cargo run

# 別ターミナルで確認
curl localhost:3000/health
# → {"status":"ok"}

curl localhost:3000/wish-items
# → []（空のリスト）
```

### ドメイン層のテスト（DB・Axum 不要）

```bash
cargo test
```

`WishItem` のステータス遷移テストなどが通ればOK。

### フロントエンド（React + TypeScript / Vite）

```bash
cd frontend
npm run dev
# → http://localhost:5173 でプレースホルダページが表示されれば OK
```

---

## ポート一覧

| ポート | 用途 |
|---|---|
| `3000` | Axum API サーバー |
| `5173` | Vite 開発サーバー（フロントエンド） |
| `5433` | PostgreSQL（ホストから接続する場合。コンテナ内では `db:5432`） |

---

## ファイル構成

```
.devcontainer/
├── devcontainer.json   # IDE 向けコンテナ設定（拡張機能・環境変数・ポートフォワード）
└── docker-compose.yml  # app + db の2コンテナ構成

.env.example            # 環境変数テンプレート（.env にコピーして使う）

Cargo.toml              # Rust 依存クレート（axum / sqlx / uuid / thiserror 等）
Cargo.lock              # バージョン固定ファイル（Git 管理対象）

migrations/
└── 20260615000001_initial_schema.sql  # DB スキーマ（cargo run 時に自動適用）

src/
├── main.rs             # エントリポイント：.env 読込 → DB 接続 → migrate → サーバー起動
├── domain/             # ビジネスロジック（Axum・SQLx に依存しない）
│   ├── entities/       # WishItem（集約ルート）・Budget・PurchaseRecord
│   ├── value_objects/  # Price / Category / WishItemStatus / Memo / YearMonth
│   ├── repositories/   # Repository trait（インターフェース定義のみ）
│   ├── services/       # BudgetService（複数集約をまたぐロジック）
│   └── events/         # DomainEvent 定義
├── application/        # ユースケース層（HTTP を知らない）
│   ├── use_cases/      # AddWishItem / ReviewWishItem / GetBudgetStatus
│   └── dto/            # 入出力データ構造
├── infrastructure/     # 外部依存の実装
│   └── db/             # Postgres 向け Repository impl
└── presentation/       # HTTP レイヤー
    ├── router.rs       # ルーティング定義
    └── handlers/       # health / wish_items / budgets ハンドラ

frontend/
├── src/App.tsx         # プレースホルダ（Phase 04 で UI 実装予定）
└── vite.config.ts      # Vite 設定（API プロキシ等）
```

---

## よく使うコマンド

```bash
# Rust
cargo run           # サーバー起動
cargo test          # テスト実行
cargo clippy        # Lint
cargo fmt           # フォーマット

# マイグレーション（sqlx-cli を使う場合）
sqlx migrate run    # 未適用のマイグレーションを実行
sqlx migrate info   # マイグレーション状態を確認

# フロントエンド
cd frontend
npm run dev         # 開発サーバー起動
npm run build       # 本番ビルド
npm run lint        # ESLint
```

---

## トラブルシューティング

### DevContainer が起動しない

```
open //./pipe/dockerDesktopLinuxEngine: The system cannot find the file specified
```

→ Docker Desktop が起動していません。起動後に「Reopen in Container」を再試行してください。

---

### ポート 5432 が already allocated

```
Bind for 0.0.0.0:5432 failed: port is already allocated
```

→ 古いコンテナが残っています。以下で削除してから再試行してください：

```bash
docker compose -f .devcontainer/docker-compose.yml down
```
