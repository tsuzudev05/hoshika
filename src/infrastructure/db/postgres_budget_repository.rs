#![allow(dead_code)]
//! PostgresBudgetRepository — BudgetRepository の sqlx 実装
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use super::budget_mapper::row_to_budget;
use super::error::to_repo_err;
use crate::domain::entities::budget::Budget;
use crate::domain::repositories::wish_item_repository::RepositoryError;
use crate::domain::repositories::BudgetRepository;
use crate::domain::value_objects::YearMonth;

pub struct PostgresBudgetRepository {
    pool: Arc<PgPool>,
}

impl PostgresBudgetRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BudgetRepository for PostgresBudgetRepository {
    async fn find_by_id(&self, user_id: &str, id: Uuid) -> Result<Option<Budget>, RepositoryError> {
        let row = sqlx::query(
            "SELECT id, user_id, year, month, amount, balance, set_at FROM budgets WHERE user_id = $1 AND id = $2",
        )
        .bind(user_id)
        .bind(id)
        .fetch_optional(&*self.pool)
        .await
        .map_err(to_repo_err)?;

        row.map(|r| row_to_budget(&r)).transpose()
    }

    async fn find_by_year_month(
        &self,
        user_id: &str,
        ym: YearMonth,
    ) -> Result<Option<Budget>, RepositoryError> {
        let row = sqlx::query(
            "SELECT id, user_id, year, month, amount, balance, set_at FROM budgets WHERE user_id = $1 AND year = $2 AND month = $3",
        )
        .bind(user_id)
        .bind(ym.year as i16)
        .bind(ym.month as i16)
        .fetch_optional(&*self.pool)
        .await
        .map_err(to_repo_err)?;

        row.map(|r| row_to_budget(&r)).transpose()
    }

    async fn save(&self, budget: &Budget) -> Result<(), RepositoryError> {
        sqlx::query(
            r#"
            INSERT INTO budgets (id, user_id, year, month, amount, balance, set_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO UPDATE SET
                amount  = EXCLUDED.amount,
                balance = EXCLUDED.balance
            "#,
        )
        .bind(budget.id())
        .bind(budget.user_id())
        .bind(budget.year_month().year as i16)
        .bind(budget.year_month().month as i16)
        .bind(budget.amount().value() as i64)
        .bind(budget.balance().value())
        .bind(budget.set_at())
        .execute(&*self.pool)
        .await
        .map_err(to_repo_err)?;
        Ok(())
    }
}
