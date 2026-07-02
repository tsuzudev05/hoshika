use std::sync::Arc;

use crate::domain::repositories::{BudgetRepository, CategoryRepository, WishItemRepository};
use crate::infrastructure::auth::JwtAuthService;

#[derive(Clone)]
pub struct AppState {
    pub wish_item_repo: Arc<dyn WishItemRepository>,
    pub category_repo: Arc<dyn CategoryRepository>,
    pub budget_repo: Arc<dyn BudgetRepository>,
    /// JWT の発行・検証サービス。Arc で包むことで Clone を実現する。
    pub auth_service: Arc<JwtAuthService>,
}
