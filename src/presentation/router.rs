use axum::{routing::get, Router};
use sqlx::PgPool;

use crate::presentation::handlers::{budgets, health, wish_items};

pub fn create_router(_pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(health::health_check))
        .route(
            "/wish-items",
            get(wish_items::list_wish_items).post(wish_items::add_wish_item),
        )
        .route("/budgets/status", get(budgets::get_budget_status))
}
