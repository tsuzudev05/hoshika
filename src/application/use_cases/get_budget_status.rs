#![allow(dead_code)]
//! GetBudgetStatus ユースケース
use std::sync::Arc;

use crate::application::dto::BudgetStatusOutput;
use crate::domain::repositories::wish_item_repository::RepositoryError;
use crate::domain::repositories::BudgetRepository;
use crate::domain::value_objects::YearMonth;

pub struct GetBudgetStatusUseCase {
    budget_repo: Arc<dyn BudgetRepository>,
}

impl GetBudgetStatusUseCase {
    pub fn new(budget_repo: Arc<dyn BudgetRepository>) -> Self {
        Self { budget_repo }
    }

    pub async fn execute(
        &self,
        user_id: &str,
        year: u16,
        month: u8,
    ) -> Result<Option<BudgetStatusOutput>, RepositoryError> {
        let ym = YearMonth::new(year, month).map_err(|_| RepositoryError::NotFound)?;

        let budget = self.budget_repo.find_by_year_month(user_id, ym).await?;

        Ok(budget.map(|b| BudgetStatusOutput {
            id: b.id(),
            year: b.year_month().year,
            month: b.year_month().month,
            amount: b.amount().value(),
            balance: b.balance().value(),
            is_exceeded: b.balance().is_exceeded(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    use crate::domain::entities::budget::Budget;
    use crate::domain::value_objects::{Price, YearMonth};
    use crate::infrastructure::in_memory::InMemoryBudgetRepository;

    const USER: &str = "user-1";

    fn make_budget(year: u16, month: u8, amount: u64) -> Budget {
        let ym = YearMonth::new(year, month).unwrap();
        let (b, _) = Budget::new(USER.to_string(), ym, Price::new(amount).unwrap());
        b
    }

    // --- 正常系 ---

    #[tokio::test]
    async fn execute_returns_budget_status_for_existing_year_month() {
        let budget = make_budget(2026, 6, 50000);
        let repo = Arc::new(InMemoryBudgetRepository::with_budgets(vec![budget]));

        let status = GetBudgetStatusUseCase::new(repo)
            .execute(USER, 2026, 6)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(status.year, 2026);
        assert_eq!(status.month, 6);
        assert_eq!(status.amount, 50000);
        assert_eq!(status.balance, 50000);
        assert!(!status.is_exceeded);
    }

    #[tokio::test]
    async fn execute_returns_none_for_missing_year_month() {
        let repo = Arc::new(InMemoryBudgetRepository::new());
        let result = GetBudgetStatusUseCase::new(repo)
            .execute(USER, 2026, 6)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn execute_reflects_exceeded_balance() {
        let mut budget = make_budget(2026, 6, 1000);
        budget.record_purchase(Price::new(1500).unwrap()); // balance = -500
        let repo = Arc::new(InMemoryBudgetRepository::with_budgets(vec![budget]));

        let status = GetBudgetStatusUseCase::new(repo)
            .execute(USER, 2026, 6)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(status.balance, -500);
        assert!(status.is_exceeded);
    }

    #[tokio::test]
    async fn execute_returns_none_for_other_users_budget() {
        let budget = make_budget(2026, 6, 50000);
        let repo = Arc::new(InMemoryBudgetRepository::with_budgets(vec![budget]));

        let result = GetBudgetStatusUseCase::new(repo)
            .execute("other-user", 2026, 6)
            .await
            .unwrap();
        assert!(result.is_none());
    }
}
