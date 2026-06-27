#![allow(dead_code)]
//! PostgresWishItemRepository — WishItemRepository の sqlx 実装
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::entities::wish_item::WishItem;
use crate::domain::repositories::wish_item_repository::RepositoryError;
use crate::domain::repositories::WishItemRepository;
use crate::domain::value_objects::{Category, Memo, Price, WishItemName, WishItemStatus};

pub struct PostgresWishItemRepository {
    pool: Arc<PgPool>,
}

impl PostgresWishItemRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

fn to_repo_err(e: sqlx::Error) -> RepositoryError {
    RepositoryError::Unexpected(e.to_string())
}

fn status_to_str(status: &WishItemStatus) -> &'static str {
    match status {
        WishItemStatus::Inbox => "Inbox",
        WishItemStatus::NextToBuy => "NextToBuy",
        WishItemStatus::OnHold => "OnHold",
        WishItemStatus::Archived => "Archived",
        WishItemStatus::Purchased => "Purchased",
    }
}

fn parse_status(s: &str) -> Result<WishItemStatus, RepositoryError> {
    match s {
        "Inbox" => Ok(WishItemStatus::Inbox),
        "NextToBuy" => Ok(WishItemStatus::NextToBuy),
        "OnHold" => Ok(WishItemStatus::OnHold),
        "Archived" => Ok(WishItemStatus::Archived),
        "Purchased" => Ok(WishItemStatus::Purchased),
        _ => Err(RepositoryError::Unexpected(format!("unknown status: {s}"))),
    }
}

fn row_to_wish_item(row: &sqlx::postgres::PgRow) -> Result<WishItem, RepositoryError> {
    let id: Uuid = row.try_get("id").map_err(to_repo_err)?;
    let name: String = row.try_get("name").map_err(to_repo_err)?;
    let price_val: i64 = row.try_get("price").map_err(to_repo_err)?;
    let category_id: Uuid = row.try_get("category_id").map_err(to_repo_err)?;
    let category_name: String = row.try_get("category_name").map_err(to_repo_err)?;
    let status_str: String = row.try_get("status").map_err(to_repo_err)?;
    let memo: String = row.try_get("memo").map_err(to_repo_err)?;
    let added_at: DateTime<Utc> = row.try_get("added_at").map_err(to_repo_err)?;
    let updated_at: DateTime<Utc> = row.try_get("updated_at").map_err(to_repo_err)?;

    let name = WishItemName::new(name).map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
    let price =
        Price::new(price_val as u64).map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
    let status = parse_status(&status_str)?;
    let category = Category {
        id: category_id,
        name: category_name,
    };
    let memo = Memo::new(memo);

    Ok(WishItem::reconstitute(
        id, name, price, category, status, memo, added_at, updated_at,
    ))
}

const SELECT_WISH_ITEMS: &str = r#"
    SELECT
        wi.id,
        wi.name,
        wi.price,
        wi.status::TEXT AS status,
        wi.category_id,
        c.name AS category_name,
        wi.memo,
        wi.added_at,
        wi.updated_at
    FROM wish_items wi
    JOIN categories c ON wi.category_id = c.id
"#;

#[async_trait]
impl WishItemRepository for PostgresWishItemRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<WishItem>, RepositoryError> {
        let sql = format!("{SELECT_WISH_ITEMS} WHERE wi.id = $1");
        let row = sqlx::query(&sql)
            .bind(id)
            .fetch_optional(&*self.pool)
            .await
            .map_err(to_repo_err)?;

        row.map(|r| row_to_wish_item(&r)).transpose()
    }

    async fn find_all(&self) -> Result<Vec<WishItem>, RepositoryError> {
        let sql = format!("{SELECT_WISH_ITEMS} ORDER BY wi.added_at ASC");
        let rows = sqlx::query(&sql)
            .fetch_all(&*self.pool)
            .await
            .map_err(to_repo_err)?;

        rows.iter().map(row_to_wish_item).collect()
    }

    async fn save(&self, item: &WishItem) -> Result<(), RepositoryError> {
        sqlx::query(
            r#"
            INSERT INTO wish_items (id, name, price, category_id, status, memo, added_at, updated_at)
            VALUES ($1, $2, $3, $4, $5::wish_item_status, $6, $7, $8)
            ON CONFLICT (id) DO UPDATE SET
                name        = EXCLUDED.name,
                price       = EXCLUDED.price,
                category_id = EXCLUDED.category_id,
                status      = EXCLUDED.status,
                memo        = EXCLUDED.memo
            "#,
        )
        .bind(item.id())
        .bind(item.name())
        .bind(item.price().value() as i64)
        .bind(item.category().id)
        .bind(status_to_str(item.status()))
        .bind(item.memo().value())
        .bind(item.added_at())
        .bind(item.updated_at())
        .execute(&*self.pool)
        .await
        .map_err(to_repo_err)?;
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        let result = sqlx::query("DELETE FROM wish_items WHERE id = $1")
            .bind(id)
            .execute(&*self.pool)
            .await
            .map_err(to_repo_err)?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::NotFound);
        }
        Ok(())
    }
}
