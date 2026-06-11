# Tasks

## Active

### Phase 01 · ドメインモデリング（〜6月末）

- [ ] **ユビキタス言語の定義** - 用語集（Glossary）をREADMEに書く　due 6/13
  - 「欲しいもの」の正式名称を決める（WishItem? DesiredItem?）→ `WishItem` に決定
  - 「衝動買い防止」のドメインルールを言語化する
- [ ] **コンテキストマップのスケッチ** - 3コンテキストの境界と関係を図にする　due 6/15
  - 欲しいものコンテキスト / 予算管理コンテキスト / 衝動買い防止コンテキスト
- [ ] **ドメインモデル設計** - エンティティ・値オブジェクト・集約を識別する　due 6/20
  - エンティティ: WishItem, Budget（IDで追跡されるもの）
  - 値オブジェクト: Price, Category, WaitingPeriod（値で比較されるもの）
  - 集約の境界を決める（WishItemとCheckフローは同じ集約か？）
- [ ] **ドメインイベントの洗い出し** - 「何が起きたか」を列挙する　due 6/20
  - ItemAdded / BudgetExceeded / ImpulseBuyPrevented など
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

### Phase 02 · ドメイン層・アプリケーション層（7月〜）

- [ ] **Rustプロジェクト構造をレイヤーで切る** - domain / application / infrastructure / presentation
- [ ] **WishItemエンティティ実装** - IDによる同一性・不変条件をメソッドで保護
- [ ] **値オブジェクト実装** - Price / Category / WaitingPeriod
- [ ] **WishItemRepository trait定義** - DBを知らないインターフェース
- [ ] **BudgetService ドメインサービス実装** - 予算超過チェック
- [ ] **ユースケース実装** - AddWishItem / CheckImpulseBuy / GetBudgetStatus
- [ ] **ドメイン層のテスト整備** - cargo testだけで通るか確認

## Done

- [ ] **ユビキタス言語の定義** - 用語集（Glossary）をREADMEに書く　完了
