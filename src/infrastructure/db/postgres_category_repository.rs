#![allow(dead_code)]
//! PostgresCategoryRepository — CategoryRepository の sqlx 実装
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use super::category_mapper::row_to_category;
use super::error::to_repo_err;
use crate::domain::repositories::wish_item_repository::RepositoryError;
use crate::domain::repositories::CategoryRepository;
use crate::domain::value_objects::Category;

pub struct PostgresCategoryRepository {
    pool: Arc<PgPool>,
}

impl PostgresCategoryRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CategoryRepository for PostgresCategoryRepository {
    async fn find_all(&self) -> Result<Vec<Category>, RepositoryError> {
        let rows = sqlx::query("SELECT id, name FROM categories ORDER BY name ASC")
            .fetch_all(&*self.pool)
            .await
            .map_err(to_repo_err)?;

        rows.iter().map(row_to_category).collect()
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Category>, RepositoryError> {
        let row = sqlx::query("SELECT id, name FROM categories WHERE id = $1")
            .bind(id)
            .fetch_optional(&*self.pool)
            .await
            .map_err(to_repo_err)?;

        row.map(|r| row_to_category(&r)).transpose()
    }
}
