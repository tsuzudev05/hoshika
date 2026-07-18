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

### 1. 環境変数ファイルを作成する

```bash
bash scripts/dev-setup.sh
```

`.env.example` → `.env` と `.devcontainer/.env.example` → `.devcontainer/.env` をコピーします。
`.devcontainer/.env` は Git 管理外（`.gitignore` に記載済み）です。パスワードを変更する場合はこのファイルを編集してください。

### 2. （任意）Claude Code の設定を DevContainer に共有する

ホストの Claude Code 設定（`~/.claude`）をコンテナ内で使いたい場合は、テンプレートをコピーして編集します。

```bash
cp .devcontainer/docker-compose.override.yml.example .devcontainer/docker-compose.override.yml
```

`.devcontainer/docker-compose.override.yml` を自分の環境に合わせて編集してください：

```yaml
services:
  app:
    volumes:
      - type: bind
        source: /mnt/c/Users/<あなたのユーザー名>/.claude  # Windows (WSL2)
        # source: /Users/<あなたのユーザー名>/.claude      # macOS
        target: /home/vscode/.claude
        read_only: true
```

> このファイルは `.gitignore` に登録済みのため、コミットされません。Docker Compose が自動的に読み込むため、設定後は DevContainer を再起動するだけで有効になります。

### 3. DevContainer を起動する

```
IDE で「Reopen in Container」を実行
```

初回は以下が自動で行われます（数分かかります）：

- `mcr.microsoft.com/devcontainers/base:ubuntu-22.04` のビルド
- Rust / Node 20 / GitHub CLI のインストール
- `cargo build`（依存クレートのダウンロード）
- `npm install`（フロントエンド依存）

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
```

全エンドポイントの一覧は [README.md](./README.md) の「API エンドポイント」を参照。

### ドメイン層のテスト（DB・Axum 不要）

```bash
cargo test
```

`WishItem` のステータス遷移テストなどが通ればOK。CIと同じ検査（fmt / clippy / build / test）をまとめて実行する場合は `bash scripts/ci-rust.sh`。

### フロントエンド（React + TypeScript / Vite）

```bash
cd frontend
npm run dev
# → http://localhost:5173 で画面が表示されれば OK
```

初回は `wish_items` が空のため「欲しいものはまだ登録されていません」と表示される。サンプルデータを投入する場合は [`scripts/seed-dev-data.sh`](./scripts/seed-dev-data.sh) を実行する。

### フロントエンドのテスト

```bash
cd frontend
npm test
```

`WishItemList` / `WishItemCard` / `BudgetMeter` / `api/client.ts` のテストが通ればOK。CIと同じ検査（type-check / lint / test / build）をまとめて実行する場合は `bash scripts/ci-frontend.sh`。

### フロントエンドのE2Eテスト（Playwright）

> ⚠️ **`npm install` は必ず DevContainer 内のターミナルで実行すること。** ホスト（Windows/macOS）側で `npm install` を実行すると、`node_modules` にホストOS向けのネイティブバイナリ（esbuild等）がインストールされ、DevContainer（Linux）内で動かなくなる。`package.json` の編集だけをホスト側エディタで行うのは問題ないが、インストールコマンドは必ずDevContainerのターミナルで実行する。

初回のみ、Playwrightが使うブラウザ（Chromium）をDevContainer内にダウンロードする：

```bash
cd frontend
npx playwright install chromium
```

E2Eテストの実行には、バックエンド（`cargo run`）とDBが別途起動している必要がある（Vite開発サーバーだけは `playwright.config.ts` の `webServer` 設定により自動起動する）：

```bash
# 別ターミナルでバックエンドを起動しておく
cargo run

# frontend ディレクトリで
cd frontend
npm run test:e2e
```

`e2e/wish-item-flow.spec.ts` のシナリオ（追加→一覧表示、レビュー操作によるステータス遷移、カテゴリフィルター）が通ればOK。

> テストは実行のたびに `E2E` プレフィックス付きの名前で `wish_items` にデータを作成する。後片付け用の削除APIがまだ存在しないため、`playwright.config.ts` の `globalTeardown`（`e2e/global-teardown.ts`）が `DATABASE_URL` に直接接続し、`name LIKE 'E2E%'` の行をテスト終了後に削除する。`DATABASE_URL` が環境変数として設定されていない場合はクリーンアップがスキップされ警告が出るので、その場合は手動で `psql "$DATABASE_URL" -c "DELETE FROM wish_items WHERE name LIKE 'E2E%';"` を実行すること。

---

## ポート一覧

| ポート | 用途 |
|---|---|
| `3000` | Axum API サーバー |
| `5173` | Vite 開発サーバー（フロントエンド） |
| `5433` | PostgreSQL（ホストから接続する場合。コンテナ内では `db:5432`） |

---

## デプロイ（Fly.io）

フロントエンド（Vite）とバックエンド（Axum）を単一の Fly app にまとめ、Axum バイナリが
ビルド済みフロントエンド（`frontend/dist`）を静的配信する構成。API は本番では `/api` 配下に
ネストされる（`STATIC_DIR` 環境変数が設定されている時だけ有効になり、ローカル/CI では未設定
のため従来通り API はルート直下で動く。`src/main.rs` 参照）。

### 初回セットアップ（アカウントに紐づく操作のため、Fly.io アカウントを持つ本人が手元で実行）

```bash
# flyctl のインストール（未導入の場合）
curl -L https://fly.io/install.sh | sh

# 1. アプリを作成する（fly.toml の app 名を実際のものに更新すること）
fly launch --no-deploy

# 2. Postgres を作成してアタッチする（DATABASE_URL が自動で secrets に設定される）
fly postgres create
fly postgres attach <postgres-app-name>

# 3. JWT_SECRET を設定する（本番用の値。開発用のデフォルト鍵は使わないこと）
fly secrets set JWT_SECRET=<本番用の秘密鍵>
```

GitHub リポジトリの **Settings → Secrets and variables → Actions** に `FLY_API_TOKEN` を登録する
（`fly tokens create deploy` で発行したトークンを使う）。

### 自動デプロイ

上記セットアップ後は `main` ブランチへの push で `.github/workflows/fly-deploy.yml` が
`flyctl deploy --remote-only` を実行し、自動的に再デプロイされる（Fly のリモートビルダーを
使うため GitHub Actions 側に Docker 環境は不要）。手動でデプロイしたい場合は
Actions タブから `Fly Deploy` ワークフローを `workflow_dispatch` で実行するか、
手元で `fly deploy` を実行する。

### ローカルで本番相当の配信を確認する

```bash
cd frontend && npm run build && cd ..
STATIC_DIR=$(pwd)/frontend/dist cargo run
# http://localhost:3000/       … フロントエンド
# http://localhost:3000/api/health … API（本番と同じ /api プレフィックス）
```

`docker build .` でイメージがビルドできることも変更のたびに確認するとよい。

---

## ファイル構成

```
.devcontainer/
├── devcontainer.json                    # IDE 向けコンテナ設定（拡張機能・環境変数・ポートフォワード）
├── docker-compose.yml                   # app + db の2コンテナ構成（Git 管理対象）
├── docker-compose.override.yml.example  # 個人設定オーバーライドのテンプレート
└── docker-compose.override.yml          # 個人設定オーバーライド（Git 管理外・各自作成）

.env.example            # 環境変数テンプレート（.env にコピーして使う）

Dockerfile              # Fly.io デプロイ用（フロントエンドビルド→Rustビルド→実行イメージの3段階）
fly.toml                # Fly.io アプリ設定（内部ポート・ヘルスチェック・環境変数）

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
│   └── dto/            # 入出力データ構造（WishItem / Budget / Category）
├── infrastructure/     # 外部依存の実装
│   ├── db/             # Postgres 向け Repository impl
│   ├── in_memory/      # テスト用 InMemory Repository impl
│   └── auth/           # JwtAuthService
└── presentation/       # HTTP レイヤー
    ├── router.rs       # ルーティング定義
    └── handlers/       # health / auth / wish_items / categories / budgets ハンドラ

frontend/
├── src/
│   ├── App.tsx         # ルートコンポーネント
│   ├── api/            # APIコール関数（client.ts / wishItems.ts / budgets.ts / categories.ts）
│   ├── components/     # WishItemList / WishItemCard / BudgetMeter / AddWishItemForm
│   ├── utils/          # date / errors / wishItemStatus
│   └── test/           # Vitest セットアップ・mswモックハンドラー
└── vite.config.ts      # Vite 設定（API プロキシ・Vitest設定）

scripts/
├── dev-setup.sh          # 初回セットアップ（.env コピー）
├── seed-dev-data.sh      # 開発用サンプルデータ投入（budgets / wish_items）
├── ci-rust.sh            # ローカルでRust CIと同じ検査を実行
├── ci-frontend.sh        # ローカルでFrontend CIと同じ検査を実行
└── check-layer-deps.sh   # レイヤー依存ルールの検証
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
npm test            # Vitest（ユニット・コンポーネントテスト）
npm run test:e2e    # Playwright（E2Eテスト。事前に cargo run でバックエンドを起動しておくこと）

# スクリプト（すべてリポジトリルートから実行）
bash scripts/ci-rust.sh        # Rust CI と同じ検査（fmt / clippy / build / test）
bash scripts/ci-frontend.sh    # Frontend CI と同じ検査（type-check / lint / test / build）
bash scripts/ci-e2e.sh         # E2E CI と同じ検査（ビルド済みバイナリを起動し Playwright を実行）
bash scripts/seed-dev-data.sh  # budgets / wish_items にサンプルデータを投入
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

---

### psql: command not found

DevContainer 起動直後に `postCreateCommand` がまだ実行中の場合があります。数秒待ってから再試行してください。それでも解決しない場合：

```bash
sudo apt-get install -y postgresql-client
```

---

### `Did not find any relations`（psql で \dt が空）

まだ `cargo run` でサーバーを起動していない可能性がある（マイグレーションはサーバー起動時に自動適用される。[「PostgreSQL」](#postgresqldb-コンテナ) 参照）。

```bash
cargo run
# 別ターミナルで
psql -h db -U hoshika hoshika -c '\dt'
```

---

### GitHub へ push できない（Password authentication is not supported）

DevContainer 内では HTTPS 認証に Personal Access Token が使えません。GitHub CLI で認証してください（一度だけ必要）：

```bash
gh auth login        # ブラウザでデバイスコードを認証
gh auth setup-git    # git の認証ヘルパーに登録
```

---

### E2Eテストが `browserType.launch: Executable doesn't exist` で失敗する

Playwrightのブラウザ本体（Chromium）がDevContainer内にダウンロードされていません。

```bash
cd frontend
npx playwright install chromium
```

---

### E2Eテストが軒並みタイムアウト・接続エラーになる

バックエンド（`cargo run`）が起動していないか、DBが未起動の可能性があります。`playwright.config.ts` の `webServer` はVite開発サーバーのみを自動起動するため、バックエンドは別ターミナルで手動起動しておく必要があります。

```bash
# 別ターミナルで
cargo run
curl localhost:3000/health   # {"status":"ok"} が返ることを確認
```

それでも失敗する場合は、`frontend/.env` や `vite.config.ts` の `server.proxy['/api'].target` が `http://localhost:3000` を指しているか確認してください。

---

### E2Eテスト実行後もテストデータ（`E2E`始まりの名前）が残っている

`e2e/global-teardown.ts` によるクリーンアップは `DATABASE_URL` 環境変数が読めることに依存している。DevContainer内のターミナルでは通常自動的に設定されているが、`echo $DATABASE_URL` で空の場合は手動で削除する：

```bash
psql "$DATABASE_URL" -c "DELETE FROM wish_items WHERE name LIKE 'E2E%';"
```

---

### E2EテストはCIでは実行されない

`.github/workflows/frontend.yml` は `type-check` / `lint` / `test`（Vitest）/ `build` のみを実行し、`npm run test:e2e` は含まれていない。E2EはバックエンドとDBの起動が前提のため、CIに組み込むにはPostgresサービスコンテナと `cargo run` の起動をワークフローに追加する必要がある（未着手）。現状はDevContainer内でのローカル実行のみを想定している。
