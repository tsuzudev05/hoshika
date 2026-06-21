#![allow(dead_code)]
//! TODO: Phase 02 で実装
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

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
        todo!("Phase 02 で実装")
    }

    async fn find_by_id(&self, _id: Uuid) -> Result<Option<Category>, RepositoryError> {
        todo!("Phase 02 で実装")
    }
}
