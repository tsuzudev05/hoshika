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
  - [ ] 予算メーターのE2Eフロー（予算設定〜超過表示）— バックエンド/フロントエンドとも実装済みのため着手可能　未着手
  - [ ] CI（`.github/workflows/frontend.yml`）への組み込み — 現状`type-check`/`lint`/`test`/`build`のみでE2Eは含まれていない。Postgresサービスコンテナと`cargo run`起動をワークフローに追加する必要があり、未着手
- [x] **予算設定UI** — 月次予算を登録・更新できるフォームを追加
  - バックエンド: `POST /budgets`（`SetBudgetUseCase` / `budgets::set_budget` ハンドラー）を新規実装。年月の予算が未設定なら新規作成、既存なら金額を更新する（`Budget::update_amount` で残高を差分調整し、既に記録された購入の影響を失わないようにした）
  - フロントエンド: `SetBudgetForm` コンポーネントを追加し `BudgetMeter` と連携。予算未設定時（404）はその場でフォームを表示、設定済みのときは「予算を編集」から更新できる
  - ドメイン層テスト4件・ユースケーステスト5件・フロントエンドコンポーネントテスト9件を追加、全て通過（`cargo test` 81件 / フロントエンド33件）
  - 実機（ブラウザ）で新規作成・更新の両フローを確認。確認中にバックエンドのルート未反映（プロセス再起動漏れ）に気づき再起動して解消　完了（2026-07-12）
- [ ] **購入記録機能**（`NextToBuy` → `Purchased`）— 実機確認中に「次に買う」から「購入済み」へのステータス変更ができないことが判明。ドメイン層（`WishItem::purchase()` / `Budget::record_purchase()` / `PurchaseRecord` エンティティ）は既に実装済みだが、これらをつなぐユースケース・APIエンドポイント・`PurchaseRecordRepository`・フロントエンドUIが一切無く、機能として未着手（バグではなく未実装）。実装時は以下を決める必要がある
  - 実際の支払額をどう入力させるか（希望価格をそのまま使うか、フォームで入力させるか — READMEのドメインモデルでは実支払額は希望価格と独立して持つ設計）
  - どの月の予算から差し引くか（購入日時点の当月固定 でよいか）
  - 予算超過（`BudgetService::will_exceed`）時の扱い（警告のみか、確認ダイアログを挟むか）
  発見（2026-07-12）
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
