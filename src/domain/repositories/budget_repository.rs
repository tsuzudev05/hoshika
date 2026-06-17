//! BudgetRepository trait
use crate::domain::entities::budget::Budget;
use crate::domain::value_objects::YearMonth;
use crate::domain::repositories::wish_item_repository::RepositoryError;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait BudgetRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Budget>, RepositoryError>;
    async fn find_by_year_month(&self, ym: YearMonth) -> Result<Option<Budget>, RepositoryError>;
    async fn save(&self, budget: &Budget) -> Result<(), RepositoryError>;
}
