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

pub fn create_router(pool: PgPool) -> Router {
    let pool = Arc::new(pool);
    let state = AppState {
        wish_item_repo: Arc::new(PostgresWishItemRepository::new(pool.clone())),
        category_repo: Arc::new(PostgresCategoryRepository::new(pool.clone())),
        budget_repo: Arc::new(PostgresBudgetRepository::new(pool)),
    };

    Router::new()
        .route("/health", get(health::health_check))
        .route(
            "/wish-items",
            get(wish_items::list_wish_items).post(wish_items::add_wish_item),
        )
        .route("/budgets/status", get(budgets::get_budget_status))
        .route("/wish-items/:id/review", post(wish_items::review_wish_item))
        .with_state(state)
}
