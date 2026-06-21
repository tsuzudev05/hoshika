#![allow(dead_code)]
//! CategoryRepository trait
use crate::domain::repositories::wish_item_repository::RepositoryError;
use crate::domain::value_objects::Category;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait CategoryRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Category>, RepositoryError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Category>, RepositoryError>;
}
