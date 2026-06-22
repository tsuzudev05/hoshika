# Tasks · Phase 01 完了記録

## Phase 01 · ドメインモデリング（6月）　✅ 完了（2026-06-22）

- [x] **ユビキタス言語の定義** - 用語集（Glossary）をREADMEに書く　完了（2026-06-12）
- [x] **コンテキストマップのスケッチ** - 3コンテキストの境界と関係を図にする　due 6/15　完了（2026-06-12）
  - 欲しいものコンテキスト / 予算管理コンテキスト / 衝動買い防止コンテキスト
  - READMEにMermaidダイアグラム + 統合パターン（Customer-Supplier / Published Language / ACL）を追記
- [x] **ドメインモデル設計** - エンティティ・値オブジェクト・集約を識別する　due 6/20　完了（2026-06-13）
  - エンティティ: WishItem, Budget, PurchaseRecord
  - 値オブジェクト: Price, Category, WishItemStatus, Memo, YearMonth
  - 集約境界: WishItemとCheckフローは同じ集約（WishItemが集約ルート）→ [domain-model.md](./domain-model.md) 参照
- [x] **ドメインイベントの洗い出し** - 「何が起きたか」を列挙する　due 6/20　完了（2026-06-13）
  - ItemAdded / ItemReviewed / ItemMovedToNextToBuy / ItemArchived / ItemPurchased
  - BudgetSet / PurchaseRecorded / BudgetExceeded
  - WaitingPeriodは見送り（タイマー強制ではなくレビュー行為で防止する設計のため）
- [x] **設計思想をDDD + Clean Architectureに統一** - README / roadmap を更新　完了（2026-06-13）
- [x] **DB設計** - ドメインモデルからテーブル設計を導出する　due 6/25　完了（2026-06-14）
  - wish_items / budgets / categories / purchase_records
  - 注意: テーブルの都合でエンティティを歪めない
  - 詳細: [db-design.md](./db-design.md) 参照
- [x] **DB設計の壁打ち** - db-design.mdをもとにトレードオフを議論する　due 6/16
  - `balance` のキャッシュ vs 都度集計
  - `category_id NOT NULL` のカテゴリ削除問題
  - `wish_item_status` をENUMにするトレードオフ
  - ステータス遷移履歴をどこで持つか
  - 購入済み `WishItem` の削除制約問題
- [x] **開発環境構築** - DevContainer（Rust / React+TS / PostgreSQL / Fly.io）　due 6/28　完了（2026-06-18）
  - `.devcontainer/devcontainer.json` + `docker-compose.yml`（PostgreSQL）
  - `Cargo.toml`（axum / sqlx / uuid / thiserror 等）
  - `src/` Clean Architectureレイヤー構造（domain / application / infrastructure / presentation）
  - `frontend/`（Vite + React + TypeScript + TanStack Query）
  - `migrations/20260615000001_initial_schema.sql`（db-design.mdのスキーマ）
- [x] **GitHub repo作成・CI/CD基礎設定** - GitHub Actions　due 6/30　完了（2026-06-22）
  - rust.yml / frontend.yml（型チェック・ビルド）
  - ai-review.yml / auto-pr.yml / create-issues.yml
  - Fly.ioデプロイパイプラインはPhase 02以降
