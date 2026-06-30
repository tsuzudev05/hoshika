//! Axum ルーター — エンドポイントの定義と依存性の注入（DI）を行う。
//!
//! # やっていること
//!
//! 1. **DI の組み立て**: PgPool から各リポジトリを生成し、AppState にまとめる
//! 2. **ルーティング**: URL パスとハンドラー関数を対応付ける
//! 3. **状態の注入**: `with_state(state)` で全ハンドラーから AppState を参照できるようにする
//!
//! ハンドラーはリクエストの解析とレスポンスの変換のみを担う。
//! ビジネスロジックはユースケース層に委譲する。

use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

use crate::infrastructure::db::{
    postgres_budget_repository::PostgresBudgetRepository,
    postgres_category_repository::PostgresCategoryRepository,
    postgres_wish_item_repository::PostgresWishItemRepository,
};
use crate::presentation::{
    handlers::{budgets, health, wish_items},
    state::AppState,
};

/// アプリケーション全体のルーターを組み立てて返す。
///
/// `PgPool` を受け取り、各 Postgres リポジトリを生成して `AppState` に注入する。
/// ハンドラーは `State<AppState>` extractor で各リポジトリにアクセスする。
pub fn create_router(pool: PgPool) -> Router {
    // Arc で包んで複数のリポジトリで PgPool を共有する
    let pool = Arc::new(pool);

    // 各リポジトリに PgPool を渡して生成し、AppState に集約する
    // （ここが DI の組み立てポイント。テスト時は InMemory 実装に差し替え可能）
    let state = AppState {
        wish_item_repo: Arc::new(PostgresWishItemRepository::new(pool.clone())),
        category_repo: Arc::new(PostgresCategoryRepository::new(pool.clone())),
        budget_repo: Arc::new(PostgresBudgetRepository::new(pool)),
    };

    Router::new()
        // ヘルスチェック — サーバーが起動しているか確認するだけのエンドポイント
        .route("/health", get(health::health_check))
        // 欲しいものリスト — 一覧取得 / 新規追加
        .route(
            "/wish-items",
            get(wish_items::list_wish_items).post(wish_items::add_wish_item),
        )
        // 予算状況 — 指定年月の予算と残高を返す（クエリパラメータ: ?year=&month=）
        .route("/budgets/status", get(budgets::get_budget_status))
        // 欲しいものレビュー — 衝動買い防止チェック（still_want: true/false）
        .route("/wish-items/:id/review", post(wish_items::review_wish_item))
        // AppState を全ハンドラーに注入する（Axum の State extractor で取り出せるようになる）
        .with_state(state)
}
