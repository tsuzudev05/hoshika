#![allow(dead_code)]
//! PostgresWishItemRepository — WishItemRepository の sqlx 実装
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use super::error::to_repo_err;
use super::wish_item_mapper::{row_to_wish_item, status_to_str};
use crate::domain::entities::wish_item::WishItem;
use crate::domain::repositories::wish_item_repository::RepositoryError;
use crate::domain::repositories::WishItemRepository;

pub struct PostgresWishItemRepository {
    pool: Arc<PgPool>,
}

impl PostgresWishItemRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

const SELECT_WISH_ITEMS: &str = r#"
    SELECT
        wi.id,
        wi.user_id,
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
    async fn find_by_id(
        &self,
        user_id: &str,
        id: Uuid,
    ) -> Result<Option<WishItem>, RepositoryError> {
        let sql = format!("{SELECT_WISH_ITEMS} WHERE wi.user_id = $1 AND wi.id = $2");
        let row = sqlx::query(&sql)
            .bind(user_id)
            .bind(id)
            .fetch_optional(&*self.pool)
            .await
            .map_err(to_repo_err)?;

        row.map(|r| row_to_wish_item(&r)).transpose()
    }

    async fn find_all(&self, user_id: &str) -> Result<Vec<WishItem>, RepositoryError> {
        let sql = format!("{SELECT_WISH_ITEMS} WHERE wi.user_id = $1 ORDER BY wi.added_at ASC");
        let rows = sqlx::query(&sql)
            .bind(user_id)
            .fetch_all(&*self.pool)
            .await
            .map_err(to_repo_err)?;

        rows.iter().map(row_to_wish_item).collect()
    }

    async fn save(&self, item: &WishItem) -> Result<(), RepositoryError> {
        sqlx::query(
            r#"
            INSERT INTO wish_items (id, user_id, name, price, category_id, status, memo, added_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6::wish_item_status, $7, $8, $9)
            ON CONFLICT (id) DO UPDATE SET
                name        = EXCLUDED.name,
                price       = EXCLUDED.price,
                category_id = EXCLUDED.category_id,
                status      = EXCLUDED.status,
                memo        = EXCLUDED.memo
            "#,
        )
        .bind(item.id())
        .bind(item.user_id())
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

    async fn delete(&self, user_id: &str, id: Uuid) -> Result<(), RepositoryError> {
        let result = sqlx::query("DELETE FROM wish_items WHERE user_id = $1 AND id = $2")
            .bind(user_id)
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
