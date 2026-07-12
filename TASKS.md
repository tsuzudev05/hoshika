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
  - [ ] 予算メーターのE2Eフロー（予算設定〜超過表示）は下記「予算設定UI」の実装後に追加
  - [ ] CI（`.github/workflows/frontend.yml`）への組み込み — 現状`type-check`/`lint`/`test`/`build`のみでE2Eは含まれていない。Postgresサービスコンテナと`cargo run`起動をワークフローに追加する必要があり、未着手
- [ ] **予算設定UI** — 月次予算を登録・更新できるフォームをフロントエンドに追加。バックエンドに`POST /budgets`相当のエンドポイントがまだ無いため、そちらも合わせて実装が必要。上記「予算メーターのE2Eフロー」はこのタスクの完了に依存
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
