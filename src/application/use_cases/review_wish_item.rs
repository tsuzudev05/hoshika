//! ReviewWishItem ユースケース（衝動買い防止チェック）
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::repositories::WishItemRepository;
use crate::domain::repositories::wish_item_repository::RepositoryError;

pub struct ReviewWishItemUseCase {
    wish_item_repo: Arc<dyn WishItemRepository>,
}

impl ReviewWishItemUseCase {
    pub fn new(wish_item_repo: Arc<dyn WishItemRepository>) -> Self {
        Self { wish_item_repo }
    }

    pub async fn execute(&self, id: Uuid, still_want: bool) -> Result<(), ReviewError> {
        let mut item = self
            .wish_item_repo
            .find_by_id(id)
            .await?
            .ok_or(ReviewError::NotFound(id))?;

        let _events = item.review(still_want)
            .map_err(|e| ReviewError::DomainError(e.to_string()))?;

        self.wish_item_repo.save(&item).await?;
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ReviewError {
    #[error("wish item not found: {0}")]
    NotFound(Uuid),
    #[error("domain error: {0}")]
    DomainError(String),
    #[error("repository error: {0}")]
    Repository(#[from] RepositoryError),
}
