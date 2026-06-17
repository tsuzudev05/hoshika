# Tasks

## Active

### Phase 01 · ドメインモデリング（〜6月末）

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
- [ ] **DB設計** - ドメインモデルからテーブル設計を導出する　due 6/25
  - wish_items / budgets / categories
  - 注意: テーブルの都合でエンティティを歪めない
- [ ] **開発環境構築** - DevContainer（Rust / React+TS / PostgreSQL / Fly.io）　due 6/28
- [ ] **GitHub repo作成・CI/CD基礎設定** - GitHub Actions　due 6/30

### 学習（並行）

- [ ] **「ドメイン駆動設計入門」読み進める** - Phase 01作業と同期して読む
  - 集約を設計するタイミングで集約の章を読む
- [ ] **「Clean Architecture」読み進める** - 原則・考え方を先に頭に入れる
  - 章ごとに「なぜそうするか」を自分の言葉でメモする

## Waiting On

## Someday

### Phase 02 · ドメイン層・アプリケーション層（7月〜）　※ DDD + Clean Architecture

- [ ] **Rustプロジェクト構造をレイヤーで切る** - domain / application / infrastructure / presentation
  - 依存ルール確認: `domain/` が `axum` / `sqlx` に依存していないこと
- [ ] **WishItemエンティティ実装** - IDによる同一性・不変条件をメソッドで保護
  - ステータス遷移は `WishItem::review()` メソッドに閉じ込める
- [ ] **値オブジェクト実装** - Price / Category / WishItemStatus / Memo / YearMonth
  - `WaitingPeriod` は見送り（レビュー行為で防止する設計のため）
- [ ] **WishItemRepository trait定義** - DBを知らないインターフェース（domain層に置く）
- [ ] **BudgetService ドメインサービス実装** - 予算超過チェック（複数集約をまたぐため）
- [ ] **ユースケース実装** - AddWishItem / ReviewWishItem / GetBudgetStatus
  - application層に置く。HTTPを知らない。引数・戻り値はDTO
- [ ] **InMemoryRepository実装** - テスト用。DBなしでドメイン・ユースケース層をテスト
- [ ] **ドメイン層のテスト整備** - cargo testだけで通るか確認（DBもAxumも不要）

## Done

- [x] **ユビキタス言語の定義** - 用語集（Glossary）をREADMEに書く　完了（2026-06-12）
- [x] **コンテキストマップのスケッチ** - 3コンテキストの境界と関係を図にする　完了（2026-06-12）
- [x] **ドメインモデル設計** - エンティティ・値オブジェクト・集約を識別する　完了（2026-06-13）
- [x] **ドメインイベントの洗い出し** - 「何が起きたか」を列挙する　完了（2026-06-13）
- [x] **設計思想をDDD + Clean Architectureに統一** - README / roadmap を更新　完了（2026-06-13）
