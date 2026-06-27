#![allow(dead_code)]
//! PostgresBudgetRepository — BudgetRepository の sqlx 実装
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::entities::budget::Budget;
use crate::domain::repositories::wish_item_repository::RepositoryError;
use crate::domain::repositories::BudgetRepository;
use crate::domain::value_objects::{Balance, Price, YearMonth};

pub struct PostgresBudgetRepository {
    pool: Arc<PgPool>,
}

impl PostgresBudgetRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

fn to_repo_err(e: sqlx::Error) -> RepositoryError {
    RepositoryError::Unexpected(e.to_string())
}

fn row_to_budget(row: &sqlx::postgres::PgRow) -> Result<Budget, RepositoryError> {
    let id: Uuid = row.try_get("id").map_err(to_repo_err)?;
    let year: i16 = row.try_get("year").map_err(to_repo_err)?;
    let month: i16 = row.try_get("month").map_err(to_repo_err)?;
    let amount_val: i64 = row.try_get("amount").map_err(to_repo_err)?;
    let balance_val: i64 = row.try_get("balance").map_err(to_repo_err)?;
    let set_at: DateTime<Utc> = row.try_get("set_at").map_err(to_repo_err)?;

    let year_month = YearMonth::new(year as u16, month as u8)
        .map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
    let amount =
        Price::new(amount_val as u64).map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
    let balance = Balance::new(balance_val);

    Ok(Budget::reconstitute(
        id, year_month, amount, balance, set_at,
    ))
}

#[async_trait]
impl BudgetRepository for PostgresBudgetRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Budget>, RepositoryError> {
        let row = sqlx::query(
            "SELECT id, year, month, amount, balance, set_at FROM budgets WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&*self.pool)
        .await
        .map_err(to_repo_err)?;

        row.map(|r| row_to_budget(&r)).transpose()
    }

    async fn find_by_year_month(&self, ym: YearMonth) -> Result<Option<Budget>, RepositoryError> {
        let row = sqlx::query(
            "SELECT id, year, month, amount, balance, set_at FROM budgets WHERE year = $1 AND month = $2",
        )
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
            INSERT INTO budgets (id, year, month, amount, balance, set_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (id) DO UPDATE SET
                amount  = EXCLUDED.amount,
                balance = EXCLUDED.balance
            "#,
        )
        .bind(budget.id())
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
