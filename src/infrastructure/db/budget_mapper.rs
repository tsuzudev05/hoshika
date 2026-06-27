use chrono::{DateTime, Utc};
use sqlx::Row;
use uuid::Uuid;

use super::error::to_repo_err;
use crate::domain::entities::budget::Budget;
use crate::domain::repositories::wish_item_repository::RepositoryError;
use crate::domain::value_objects::{Balance, Price, YearMonth};

pub(super) fn row_to_budget(row: &sqlx::postgres::PgRow) -> Result<Budget, RepositoryError> {
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
