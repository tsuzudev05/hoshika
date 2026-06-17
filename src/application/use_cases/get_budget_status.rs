//! GetBudgetStatus ユースケース
use std::sync::Arc;

use crate::domain::repositories::BudgetRepository;
use crate::domain::repositories::wish_item_repository::RepositoryError;
use crate::domain::value_objects::YearMonth;
use crate::application::dto::BudgetStatusOutput;

pub struct GetBudgetStatusUseCase {
    budget_repo: Arc<dyn BudgetRepository>,
}

impl GetBudgetStatusUseCase {
    pub fn new(budget_repo: Arc<dyn BudgetRepository>) -> Self {
        Self { budget_repo }
    }

    pub async fn execute(&self, year: u16, month: u8) -> Result<Option<BudgetStatusOutput>, RepositoryError> {
        let ym = YearMonth::new(year, month)
            .map_err(|_| RepositoryError::NotFound)?;

        let budget = self.budget_repo.find_by_year_month(ym).await?;

        Ok(budget.map(|b| BudgetStatusOutput {
            id: b.id,
            year: b.year_month.year,
            month: b.year_month.month,
            amount: b.amount.value(),
            balance: b.balance,
            is_exceeded: b.balance < 0,
        }))
    }
}
