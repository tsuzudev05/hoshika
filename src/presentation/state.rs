use std::sync::Arc;

use crate::domain::repositories::{BudgetRepository, CategoryRepository, WishItemRepository};

#[derive(Clone)]
pub struct AppState {
    pub wish_item_repo: Arc<dyn WishItemRepository>,
    pub category_repo: Arc<dyn CategoryRepository>,
    pub budget_repo: Arc<dyn BudgetRepository>,
}
