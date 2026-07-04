# Tasks · Phase 03 完了記録

## Phase 03 · インフラ層・プレゼンテーション層（Rust）　✅ 完了（2026-06-28）

- [x] **PostgresRepository 実装** — sqlx による `WishItemRepository` / `BudgetRepository` / `CategoryRepository` のimpl　完了（2026-06-26）
  - `PostgresWishItemRepository` — find_by_id / find_all / save（UPSERT） / delete（NotFound対応）
  - `PostgresBudgetRepository` — find_by_id / find_by_year_month / save（UPSERT）
  - `PostgresCategoryRepository` — find_all / find_by_id
  - `Budget::reconstitute()` / `WishItem::reconstitute()` をドメイン層に追加（DBからの復元用コンストラクタ）
  - sqlx Error → `RepositoryError::Unexpected` の変換は各ファイル内 `to_repo_err()` で行う
  - status は PostgreSQL enum を `::TEXT` キャストで読み込み、書込み時は `$n::wish_item_status` でキャスト
- [x] **インフラ層の残タスク**　完了（2026-06-28）
  - [x] `JwtAuthService` — `jsonwebtoken` crate で発行・検証を実装。`JWT_SECRET` 環境変数からキーを取得。`AuthError::Expired` / `InvalidToken` で検証エラーを分類。ユニットテスト3本（正常 / 改ざん / 期限切れ）
  - [x] DIコンテナ的な組み立て — `AppState` に `Arc<dyn Repo>` 3本を保持し、`create_router()` 内で Postgres 実装を注入、`with_state()` でハンドラーに渡す　完了（2026-06-28）
- [x] **プレゼンテーション層の実装**　完了（2026-06-28）
  - `GET /wish-items` — 全件取得、`WishItemOutput` の配列を返す
  - `POST /wish-items` — `AddWishItemUseCase` 呼び出し、201 Created
  - `POST /wish-items/:id/review` — `ReviewWishItemUseCase` 呼び出し
  - `GET /budgets/status?year=&month=` — `GetBudgetStatusUseCase` 呼び出し、未登録なら 404
  - エラーバリアントごとに HTTP ステータスをマッピング（422 / 404 / 500）
  - ビジネスロジックはハンドラーにゼロ、ユースケース層への委譲のみ
- [x] **変更容易性の検証（Clean Architectureの真価）**　完了（2026-06-25）
  - `InMemoryWishItemRepository` を使ってユースケース層のテストが DB なしで通ることを確認
  - 「データ永続化の詳細をドメインが知らない」設計を証明（依存逆転の原則の実証）

> 補足（2026-07-04）: 起動時に `JWT_SECRET` 未設定でも開発用固定シークレットへフォールバックしていた挙動を修正。
> `APP_ENV=development` の場合のみ許可し、それ以外は起動時 fail-fast するように変更（認証バイパスの脆弱性対応）。
