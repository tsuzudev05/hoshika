#![allow(dead_code)]
//! BudgetRepository trait
use crate::domain::entities::budget::Budget;
use crate::domain::repositories::wish_item_repository::RepositoryError;
use crate::domain::value_objects::YearMonth;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait BudgetRepository: Send + Sync {
    async fn find_by_id(&self, user_id: &str, id: Uuid) -> Result<Option<Budget>, RepositoryError>;
    async fn find_by_year_month(
        &self,
        user_id: &str,
        ym: YearMonth,
    ) -> Result<Option<Budget>, RepositoryError>;
    async fn save(&self, budget: &Budget) -> Result<(), RepositoryError>;
}
