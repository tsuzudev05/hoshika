#![allow(dead_code)]
//! SetBudget ユースケース
//! 指定した年月の予算が未設定なら新規作成し、既に設定済みなら金額を更新する。
use std::sync::Arc;

use crate::application::dto::{BudgetStatusOutput, SetBudgetInput};
use crate::domain::entities::budget::Budget;
use crate::domain::repositories::BudgetRepository;
use crate::domain::value_objects::{Price, YearMonth};

pub struct SetBudgetUseCase {
    budget_repo: Arc<dyn BudgetRepository>,
}

impl SetBudgetUseCase {
    pub fn new(budget_repo: Arc<dyn BudgetRepository>) -> Self {
        Self { budget_repo }
    }

    pub async fn execute(&self, input: SetBudgetInput) -> Result<BudgetStatusOutput, UseCaseError> {
        let year_month = YearMonth::new(input.year, input.month)
            .map_err(|e| UseCaseError::DomainError(e.to_string()))?;
        if input.amount == 0 {
            return Err(UseCaseError::InvalidAmount);
        }
        let amount = Price::new(input.amount).map_err(|_| UseCaseError::InvalidAmount)?;

        let existing = self.budget_repo.find_by_year_month(year_month).await?;

        let budget = match existing {
            Some(mut budget) => {
                budget.update_amount(amount);
                budget
            }
            None => {
                let (budget, _events) = Budget::new(year_month, amount);
                budget
            }
        };

        self.budget_repo.save(&budget).await?;

        Ok(BudgetStatusOutput {
            id: budget.id(),
            year: budget.year_month().year,
            month: budget.year_month().month,
            amount: budget.amount().value(),
            balance: budget.balance().value(),
            is_exceeded: budget.balance().is_exceeded(),
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UseCaseError {
    #[error("amount must be greater than 0")]
    InvalidAmount,
    #[error("domain error: {0}")]
    DomainError(String),
    #[error("repository error: {0}")]
    Repository(#[from] crate::domain::repositories::wish_item_repository::RepositoryError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    use crate::domain::entities::budget::Budget;
    use crate::domain::value_objects::{Price, YearMonth};
    use crate::infrastructure::in_memory::InMemoryBudgetRepository;

    fn make_input(year: u16, month: u8, amount: u64) -> SetBudgetInput {
        SetBudgetInput {
            year,
            month,
            amount,
        }
    }

    // --- 正常系 ---

    #[tokio::test]
    async fn execute_creates_new_budget_when_none_exists() {
        let repo = Arc::new(InMemoryBudgetRepository::new());
        let use_case = SetBudgetUseCase::new(repo.clone());

        let output = use_case.execute(make_input(2026, 7, 50000)).await.unwrap();

        assert_eq!(output.year, 2026);
        assert_eq!(output.month, 7);
        assert_eq!(output.amount, 50000);
        assert_eq!(output.balance, 50000);
        assert!(!output.is_exceeded);

        let saved = repo.find_by_id(output.id).await.unwrap();
        assert!(saved.is_some());
    }

    #[tokio::test]
    async fn execute_updates_amount_and_adjusts_balance_when_budget_exists() {
        let ym = YearMonth::new(2026, 7).unwrap();
        let (mut budget, _) = Budget::new(ym, Price::new(50000).unwrap());
        budget.record_purchase(Price::new(20000).unwrap()); // balance = 30000
        let repo = Arc::new(InMemoryBudgetRepository::with_budgets(vec![budget]));
        let use_case = SetBudgetUseCase::new(repo.clone());

        let output = use_case.execute(make_input(2026, 7, 60000)).await.unwrap();

        assert_eq!(output.amount, 60000);
        assert_eq!(output.balance, 40000); // 30000 + (60000 - 50000)
    }

    #[tokio::test]
    async fn execute_keeps_same_budget_id_when_updating() {
        let ym = YearMonth::new(2026, 7).unwrap();
        let (budget, _) = Budget::new(ym, Price::new(50000).unwrap());
        let original_id = budget.id();
        let repo = Arc::new(InMemoryBudgetRepository::with_budgets(vec![budget]));
        let use_case = SetBudgetUseCase::new(repo);

        let output = use_case.execute(make_input(2026, 7, 60000)).await.unwrap();

        assert_eq!(output.id, original_id);
    }

    // --- 異常系 ---

    #[tokio::test]
    async fn execute_returns_error_for_zero_amount() {
        let repo = Arc::new(InMemoryBudgetRepository::new());
        let use_case = SetBudgetUseCase::new(repo);

        let result = use_case.execute(make_input(2026, 7, 0)).await;

        assert!(matches!(result, Err(UseCaseError::InvalidAmount)));
    }

    #[tokio::test]
    async fn execute_returns_error_for_invalid_month() {
        let repo = Arc::new(InMemoryBudgetRepository::new());
        let use_case = SetBudgetUseCase::new(repo);

        let result = use_case.execute(make_input(2026, 13, 50000)).await;

        assert!(matches!(result, Err(UseCaseError::DomainError(_))));
    }
}
