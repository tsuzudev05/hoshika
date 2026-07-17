#![allow(dead_code)]
//! PostgresPurchaseRecordRepository — PurchaseRecordRepository の sqlx 実装
use async_trait::async_trait;
use sqlx::{PgPool, Row};
use std::sync::Arc;
use uuid::Uuid;

use super::error::to_repo_err;
use crate::domain::entities::purchase_record::PurchaseRecord;
use crate::domain::repositories::wish_item_repository::RepositoryError;
use crate::domain::repositories::PurchaseRecordRepository;
use crate::domain::value_objects::{Memo, Price};

pub struct PostgresPurchaseRecordRepository {
    pool: Arc<PgPool>,
}

impl PostgresPurchaseRecordRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PurchaseRecordRepository for PostgresPurchaseRecordRepository {
    async fn find_by_id(
        &self,
        user_id: &str,
        id: Uuid,
    ) -> Result<Option<PurchaseRecord>, RepositoryError> {
        let row = sqlx::query(
            "SELECT id, user_id, budget_id, wish_item_id, actual_price, memo, purchased_at FROM purchase_records WHERE user_id = $1 AND id = $2",
        )
        .bind(user_id)
        .bind(id)
        .fetch_optional(&*self.pool)
        .await
        .map_err(to_repo_err)?;

        row.map(|r| row_to_purchase_record(&r)).transpose()
    }

    async fn save(&self, record: &PurchaseRecord) -> Result<(), RepositoryError> {
        sqlx::query(
            r#"
            INSERT INTO purchase_records (id, user_id, budget_id, wish_item_id, actual_price, memo, purchased_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(record.id())
        .bind(record.user_id())
        .bind(record.budget_id())
        .bind(record.wish_item_id())
        .bind(record.actual_price().value() as i64)
        .bind(record.memo().value())
        .bind(record.purchased_at())
        .execute(&*self.pool)
        .await
        .map_err(to_repo_err)?;
        Ok(())
    }
}

fn row_to_purchase_record(row: &sqlx::postgres::PgRow) -> Result<PurchaseRecord, RepositoryError> {
    let id: Uuid = row.try_get("id").map_err(to_repo_err)?;
    let user_id: String = row.try_get("user_id").map_err(to_repo_err)?;
    let budget_id: Uuid = row.try_get("budget_id").map_err(to_repo_err)?;
    let wish_item_id: Uuid = row.try_get("wish_item_id").map_err(to_repo_err)?;
    let actual_price_val: i64 = row.try_get("actual_price").map_err(to_repo_err)?;
    let memo: String = row.try_get("memo").map_err(to_repo_err)?;
    let purchased_at = row.try_get("purchased_at").map_err(to_repo_err)?;

    let actual_price = Price::new(actual_price_val as u64)
        .map_err(|e| RepositoryError::Unexpected(e.to_string()))?;

    Ok(PurchaseRecord::reconstitute(
        id,
        user_id,
        budget_id,
        wish_item_id,
        actual_price,
        Memo::new(memo),
        purchased_at,
    ))
}
