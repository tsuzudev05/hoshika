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
  - [ ] CI（`.github/workflows/frontend.yml`）への組み込み — 現状`type-check`/`lint`/`test`/`build`のみでE2Eは含まれていない。Postgresサービスコンテナと`cargo run`起動をワークフローに追加する必要があり、未着手
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
- [ ] **Fly.ioデプロイ** — ステージング環境・自動デプロイ
- [ ] **Sentry導入** — エラートラッキング（アプリ層とインフラ層でのエラー分類も意識）
- [ ] **パフォーマンス計測** — Lighthouse・DBクエリ最適化
- [ ] **セキュリティ確認** — CORS・SQLインジェクション・認証周りの確認
- [ ] ✅ チェックポイント: 「新機能を追加するとき、どのレイヤーを触るか迷わないか？」

### 学習（並行）

- [ ] **「ドメイン駆動設計入門」読み進める** - Phase 02作業と同期して読む
  - 集約を設計するタイミングで集約の章を読む
- [ ] **「Clean Architecture」読み進める** - 原則・考え方を先に頭に入れる
  - 章ごとに「なぜそうするか」を自分の言葉でメモする
