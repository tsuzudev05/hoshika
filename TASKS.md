# Tasks

> Phase 01 完了記録 → [TASKS-phase01.md](./TASKS-phase01.md)
> Phase 02 完了記録 → [TASKS-phase02.md](./TASKS-phase02.md)
> Phase 03 完了記録 → [TASKS-phase03.md](./TASKS-phase03.md)
> Phase 04 完了記録 → [TASKS-phase04.md](./TASKS-phase04.md)

## Active

### Phase 05 · 品質・インフラ（9月〜）

> **このフェーズで学ぶこと**: 設計の良さはテストカバレッジと変更のしやすさで測る

- [ ] **E2Eテスト（Playwright）** — 主要フローをカバー　着手中
  - [x] テスト基盤導入 — `frontend/playwright.config.ts`（Vite開発サーバー自動起動、baseURL `http://localhost:5173`）
  - [x] 追加 → 一覧表示のフロー — `wish-item-flow.spec.ts`
  - [x] レビュー「欲しい」→ステータス遷移（`Inbox` → `NextToBuy`）のフロー
  - [x] カテゴリフィルターの絞り込みフロー
  - [x] レビュー「やめておく」→ステータス遷移（`Inbox` → `Archived`）のフロー　完了（2026-07-10）
  - [ ] `@playwright/test` の依存追加・`npm run test:e2e` スクリプト整備 — **要DevContainer側で `npm install` 実行**（ホスト側npm installはLinuxネイティブバイナリを壊すため厳禁。package.jsonの編集のみホスト側で行い、インストールはDevContainerで）
  - [ ] 予算メーターのE2Eフロー（予算設定〜超過表示）は予算設定UIの実装後に追加
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
