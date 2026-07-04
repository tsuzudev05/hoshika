# Tasks · Phase 02 完了記録

## Phase 02 · ドメイン層・アプリケーション層（7月）　✅ 完了（2026-06-26）

- [x] **Rustプロジェクト構造をレイヤーで切る** - domain / application / infrastructure / presentation　完了（2026-06-23）
  - 依存ルール確認: `domain/` が `axum` / `sqlx` に依存していないこと ✅
  - `RepositoryError::Database(#[from] sqlx::Error)` を削除 → `Unexpected(String)` に置換
  - sqlx変換は infrastructure 層の `to_repo_err()` で行う
- [x] **WishItemエンティティ実装** - IDによる同一性・不変条件をメソッドで保護　完了（2026-06-24）
  - フィールドをすべて private 化 → getter 経由のみアクセス可（不変条件バイパス不可）
  - `PartialEq` を id のみで実装（属性が違っても id が同じなら同一エンティティ）
  - ステータス遷移は各メソッドに閉じ込め済み（review / move_to_next_to_buy / archive / purchase）
  - テスト: 全遷移パターン + 不正遷移 + エンティティ同一性を網羅
- [x] **値オブジェクト実装** - Price / Category / WishItemStatus / Memo / YearMonth　完了（2026-06-24）
  - `WaitingPeriod` は見送り（レビュー行為で防止する設計のため）
  - `Balance`（`i64` の代替）・`WishItemName`（`String` の代替）を追加
  - プリミティブ型の代わりにドメインモデルを使うことで、ルールを型で表現
  - `Balance::is_exceeded()` / `is_sufficient_for()` / `deduct()` でドメイン意図が読める
  - `WishItemName::new()` が空文字バリデーションを担い、`WishItem::new()` が非 `Result` 化
- [x] **WishItemRepository trait定義** - DBを知らないインターフェース（domain層に置く）　完了
  - `WishItemRepository` / `BudgetRepository` / `CategoryRepository` の3 trait 定義済み
  - `RepositoryError::Unexpected(String)` で sqlx 非依存を維持
- [x] **BudgetService ドメインサービス実装** - 予算超過チェック（複数集約をまたぐため）　完了
  - `Budget::would_exceed()` に委譲（ロジックはデータを持つ `Budget` 側に）
- [x] **ユースケース実装** - AddWishItem / ReviewWishItem / GetBudgetStatus　完了
  - application層に配置。HTTPを知らない。引数・戻り値はDTO
- [x] **InMemoryRepository実装** - テスト用。DBなしでドメイン・ユースケース層をテスト　完了（2026-06-25）
  - `InMemoryWishItemRepository` / `InMemoryBudgetRepository` / `InMemoryCategoryRepository` を実装
  - `src/infrastructure/in_memory/` に配置（PostgreSQL実装と並列）
  - `tokio::sync::Mutex<HashMap<Uuid, T>>` でスレッドセーフに管理
  - `InMemoryCategoryRepository::with_categories()` でテスト用シードデータを注入可能
  - 各リポジトリに `#[tokio::test]` 付きのユニットテストを内包
- [x] **ドメイン層のテスト整備** - cargo testだけで通るか確認（DBもAxumも不要）　完了（2026-06-26）
  - AddWishItem / ReviewWishItem / GetBudgetStatus の各ユースケースにテスト追加
  - InMemoryRepository を使ってDB・Axum不要でユースケースを検証
  - 正常系・異常系（CategoryNotFound / NotFound / DomainError）を網羅
