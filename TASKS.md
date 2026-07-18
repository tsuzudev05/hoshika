# Tasks

> Phase 01 完了記録 → [TASKS-phase01.md](./TASKS-phase01.md)
> Phase 02 完了記録 → [TASKS-phase02.md](./TASKS-phase02.md)
> Phase 03 完了記録 → [TASKS-phase03.md](./TASKS-phase03.md)
> Phase 04 完了記録 → [TASKS-phase04.md](./TASKS-phase04.md)

## Active

### Phase 05 · 品質・インフラ（9月〜）

> **このフェーズで学ぶこと**: 設計の良さはテストカバレッジと変更のしやすさで測る

- [x] **E2Eテスト（Playwright）** — 主要フローをカバー
  - [x] テスト基盤導入 — `frontend/playwright.config.ts`（Vite開発サーバー自動起動、baseURL `http://localhost:5173`）
  - [x] 追加 → 一覧表示のフロー — `wish-item-flow.spec.ts`
  - [x] レビュー「欲しい」→ステータス遷移（`Inbox` → `NextToBuy`）のフロー
  - [x] カテゴリフィルターの絞り込みフロー
  - [x] レビュー「やめておく」→ステータス遷移（`Inbox` → `Archived`）のフロー　完了（2026-07-10）
  - [x] `@playwright/test` の依存追加・`npm run test:e2e` スクリプト整備 — DevContainer内で `npm install` を実行し `package-lock.json` を同期。手順を[DEVELOPMENT.md](./DEVELOPMENT.md#フロントエンドのe2eテストplaywright)に明記　完了（2026-07-10）
  - [x] テストデータのクリーンアップ — `addWishItem`ヘルパーが追加完了を待たずに次の入力を始めていた競合バグを修正。また削除APIがまだ無いため、`e2e/global-teardown.ts` で`DATABASE_URL`に直接接続し`name LIKE 'E2E%'`の行をテスト終了後に削除するようにした　完了（2026-07-10）
  - [x] 予算メーターのE2Eフロー（予算設定〜超過表示）— `予算メーター`の`describe`ブロックを追加。予算未設定/設定済みの両パターンで金額を設定し、予算超過商品を購入して「予算超過」バッジと残高表示を確認する
    - 予算は年月ごとに1件のシングルトンで`E2E`接頭辞による隔離ができないため、`beforeAll`で当月予算行を退避し`afterAll`で復元（未設定だった場合は行ごと削除）することでDevContainerの開発用DBへの影響を残さないようにした
    - `global-teardown.ts`が`purchase_records`を残したまま`wish_items`を削除しようとして外部キー制約違反で失敗するバグを発見・修正（`purchase_records`を先に削除してから`wish_items`を削除するよう変更）
    - DB直結処理を`e2e/db.ts`（`runSql`/`querySql`）に共通化し、`global-teardown.ts`と予算テストの両方から利用。psqlへの接続情報もPG*環境変数5つの組み立てから、パスワードを除いた接続URL+`PGPASSWORD`のみに簡素化した
    - DevContainer内（`cargo run`でバックエンド起動 + `npx playwright install-deps chromium`でOS依存関係導入）で全5シナリオが通過し、連続2回実行してもDBが恒久的に変化しないことを確認　完了（2026-07-15）
  - [x] CI（`.github/workflows/frontend.yml`）への組み込み — 既存の`check`ジョブとは独立した`e2e`ジョブを追加。`postgres:16-alpine`のサービスコンテナ、`cargo build`でのバックエンドビルド、ヘルスチェック付きの起動待ち、`playwright install --with-deps`、`npm run test:e2e`を実行し、失敗時は`test-results/`とバックエンドログをアーティファクト/ログ出力する
    - バックエンドは`sqlx::migrate!`で起動時に自動マイグレーションするため（`src/main.rs`）、`rust.yml`と違い事前の`sqlx migrate run`は不要
    - 全く空の新規Postgresコンテナに対してビルド済みバイナリを起動し`npm run test:e2e`を実行する形でCI環境を再現して検証。その過程で、予算が未設定のまま`afterAll`が`budgets`行を削除しようとすると`purchase_records`の外部キー制約（`purchase_records_budget_id_fkey`）違反になるバグを発見し、`deleteE2EPurchaseRecords()`を`budgets`の復元・削除より先に呼ぶよう修正（`e2e/db.ts`に`deleteE2EPurchaseRecords`/`deleteE2EWishItems`として共通化し`global-teardown.ts`とテスト双方から利用）
    - 修正後、予算未設定/設定済みの両パターンで全5シナリオが通過し、DBが実行前の状態に正しく復元されることを確認　完了（2026-07-16）
- [x] **予算設定UI** — 月次予算を登録・更新できるフォームを追加
  - バックエンド: `POST /budgets`（`SetBudgetUseCase` / `budgets::set_budget` ハンドラー）を新規実装。年月の予算が未設定なら新規作成、既存なら金額を更新する（`Budget::update_amount` で残高を差分調整し、既に記録された購入の影響を失わないようにした）
  - フロントエンド: `SetBudgetForm` コンポーネントを追加し `BudgetMeter` と連携。予算未設定時（404）はその場でフォームを表示、設定済みのときは「予算を編集」から更新できる
  - ドメイン層テスト4件・ユースケーステスト5件・フロントエンドコンポーネントテスト9件を追加、全て通過（`cargo test` 81件 / フロントエンド33件）
  - 実機（ブラウザ）で新規作成・更新の両フローを確認。確認中にバックエンドのルート未反映（プロセス再起動漏れ）に気づき再起動して解消　完了（2026-07-12）
- [x] **購入記録機能**（`NextToBuy` → `Purchased`）
  - バックエンド: `PurchaseRecordRepository`（InMemory / Postgres 実装）を新規追加し、`PurchaseWishItemUseCase` で `WishItem::purchase()` → 当月の `Budget::record_purchase()` → `PurchaseRecord` 保存の一連の流れをつなぐ。`POST /wish-items/:id/purchase` を追加
    - 実支払額はフォームで入力させる方式を採用（希望価格をデフォルト値として表示しつつ編集可能。README記載の「希望価格と実支払額は独立」というドメインルールに合わせた）
    - 予算は購入時点の当月固定。当月の予算が未設定の場合は`BudgetNotFound`エラー（422）を返す
    - 予算超過は`Budget.record_purchase()`が許容する既存のドメイン仕様どおり許可し、`BudgetMeter`の「予算超過」バッジで事後的に可視化する方針（確認ダイアログなどのブロッキングは入れない）
  - フロントエンド: `WishItemCard`に`NextToBuy`ステータス時のみ「購入済みにする」ボタンを追加。クリックすると実支払額（希望価格をデフォルト値に）・メモの入力フォームが開く。送信成功時は`wish-items`・`budget-status`両方のクエリを無効化しBudgetMeterも連動して更新
  - ドメイン層テスト（`update_amount`系4件は既存）・ユースケーステスト6件・インメモリリポジトリテスト3件・フロントエンドコンポーネントテスト5件を追加、全て通過（`cargo test` 91件 / フロントエンド38件）
  - 実機（ブラウザ）で購入フロー（希望価格のデフォルト表示・実支払額での予算差し引き・`purchase_records`への記録・予算超過バッジ表示）を確認　完了（2026-07-12）
- [x] **セキュリティ確認** — CORS・SQLインジェクション・認証周りの確認
  - CORS: 未設定だがVite devプロキシ（`/api`→`localhost:3000`）によりブラウザからは同一オリジン扱いのため実害なし。本番で別オリジン配信する場合は要検討として保留
  - SQLインジェクション: 全リポジトリでsqlxのbind変数を使用しており問題なし。`postgres_wish_item_repository.rs`に`format!`でSQLを組み立てる箇所があるが埋め込むのは固定の定数文字列のみで安全
  - 認証・認可: `POST /auth/token`が無認証で任意`user_id`のJWTを発行でき、かつ発行したJWTを検証するミドルウェアがどの業務エンドポイントにも適用されておらず、`/wish-items`等が誰でも無認証で読み書き可能な状態だったバグを発見
    - `src/presentation/auth_middleware.rs`を新規追加し、`router.rs`で業務エンドポイント（`/wish-items`系・`/categories`・`/budgets`系）のみを`route_layer`でJWT検証必須にした（`/health`・`/auth/token`・`/auth/verify`は引き続き未認証でアクセス可）
    - ユーザーごとのデータ分離（`user_id`列の追加）は大規模変更になるため今回はスコープ外とし、「有効なJWTがなければアクセス不可」という最低限の閂のみを閉じた
    - フロントエンドが一切トークンを送信していなかったため`frontend/src/api/auth.ts`を新規追加し、起動時に固定`user_id`（`hoshika-app`）で`/auth/token`を取得・キャッシュして`apiClient`の全リクエストに`Authorization`ヘッダーを付与するよう変更。MSWの既定ハンドラーにもトークン取得のモックを追加
    - バックエンド単体テスト91件・フロントエンド38件が全て通過し、実際にバックエンドとフロントを起動して未トークン時401・トークン付き200を確認。`npm run test:e2e`（実ブラウザ）でも全5シナリオが新しい認証フローで通過することを確認　完了（2026-07-17）
- [ ] **Fly.ioデプロイ** — ステージング環境・自動デプロイ
  - 設定ファイル一式を作成済み: `Dockerfile`（フロントエンドビルド→Rustビルド→実行イメージの3段階）・`.dockerignore`・`fly.toml`・`.github/workflows/fly-deploy.yml`（`main`へのpushで自動デプロイ）
  - `src/main.rs`に`STATIC_DIR`環境変数による分岐を追加。設定時のみAxumバイナリが`frontend/dist`を静的配信し、APIを`/api`配下にネストする（未設定のローカル/CIでは従来通りAPIがルート直下のまま動作し、既存のE2E・CIには一切影響しない）
  - ローカルで`STATIC_DIR`未設定/設定済みの両方を実機確認（`/health`・`/api/health`・`/`・静的アセット配信）。`cargo test`101件も通過を確認
  - この環境にはFly.ioアカウント認証・`flyctl`がないため、実際の`fly launch`（アプリ作成）・`fly postgres create`・`fly secrets set`・GitHub Secretsへの`FLY_API_TOKEN`登録はユーザー自身が行う必要がある（手順は[DEVELOPMENT.md](./DEVELOPMENT.md#デプロイflyio)に記載）。`docker build`自体もこの環境にDockerがないため未実行・未検証
- [x] **Sentry導入** — エラートラッキング（アプリ層とインフラ層でのエラー分類も意識）
  - バックエンド: `sentry`クレート（`sentry-tracing`統合込み、`rustls`/`reqwest`トランスポート）を追加。`SENTRY_DSN`未設定時は`sentry::init`自体を呼ばずcapture系呼び出しが自動でno-opになる設計にし、ローカル/CIでは分岐なしで無効化される
  - エラー分類: 各ハンドラーの「どの業務エラーにも当てはまらない`Err(e)`」の受け皿（＝リポジトリ層の`Unexpected`など、インフラ層由来で分類しようがないエラー）だけを`src/presentation/handlers/mod.rs`の`internal_error()`に集約し、`tracing::error!`でログ。`sentry::integrations::tracing::layer()`がERRORレベルのログをSentryイベントとして送るため、404/422等のドメイン上想定済みのエラーはSentryに一切飛ばない
  - フロントエンド: `@sentry/react`を追加。`VITE_SENTRY_DSN`未設定時は`initSentry()`が何もしない。`Sentry.ErrorBoundary`で`App`全体を包み予期しないレンダリング例外を`ErrorFallback`で表示。`api/client.ts`では status 0（ネットワーク断）・5xx・レスポンス契約違反（`res.ok`なのにbody解析失敗）のみ`Sentry.captureException`し、4xx（バリデーション・未認証・未検出など）は送らない
  - パフォーマンス計測（`tracesSampleRate`等）は別タスクのスコープのため含めていない
  - 動作確認: DBコンテナを一時停止させて`/categories`を叩き、`internal_error()`のERRORログ（`unexpected error: ... database ...`）が出ることを実機で確認。ダミーDSNを設定してもバックエンド起動が壊れないことも確認。`cargo fmt`/`clippy`/`test`（101件）、フロントエンドの`type-check`/`lint`/`test`（38件）/`build`、`npm run test:e2e`（実ブラウザ・5シナリオ）が全て通過することを確認　完了（2026-07-18）
- [ ] **パフォーマンス計測** — Lighthouse・DBクエリ最適化
- [ ] ✅ チェックポイント: 「新機能を追加するとき、どのレイヤーを触るか迷わないか？」

### 学習（並行）

- [ ] **「ドメイン駆動設計入門」読み進める** - Phase 02作業と同期して読む
  - 集約を設計するタイミングで集約の章を読む
- [ ] **「Clean Architecture」読み進める** - 原則・考え方を先に頭に入れる
  - 章ごとに「なぜそうするか」を自分の言葉でメモする
