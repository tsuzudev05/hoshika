#![allow(dead_code)]
//! PostgresWishItemRepository — WishItemRepository の sqlx 実装
//! TODO: Phase 02 で実装する
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

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

#[async_trait]
impl WishItemRepository for PostgresWishItemRepository {
    async fn find_by_id(&self, _id: Uuid) -> Result<Option<WishItem>, RepositoryError> {
        todo!("Phase 02 で実装")
    }

    async fn find_all(&self) -> Result<Vec<WishItem>, RepositoryError> {
        todo!("Phase 02 で実装")
    }

    async fn save(&self, _item: &WishItem) -> Result<(), RepositoryError> {
        todo!("Phase 02 で実装")
    }

    async fn delete(&self, _id: Uuid) -> Result<(), RepositoryError> {
        todo!("Phase 02 で実装")
    }
}
