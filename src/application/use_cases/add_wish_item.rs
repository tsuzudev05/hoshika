#![allow(dead_code)]
//! AddWishItem ユースケース
//! HTTP を知らない。引数は DTO、戻り値も DTO。
use std::sync::Arc;
use uuid::Uuid;

use crate::application::dto::{AddWishItemInput, WishItemOutput};
use crate::domain::{
    entities::wish_item::WishItem,
    repositories::{CategoryRepository, WishItemRepository},
    value_objects::{Memo, Price},
};

pub struct AddWishItemUseCase {
    wish_item_repo: Arc<dyn WishItemRepository>,
    category_repo: Arc<dyn CategoryRepository>,
}

impl AddWishItemUseCase {
    pub fn new(
        wish_item_repo: Arc<dyn WishItemRepository>,
        category_repo: Arc<dyn CategoryRepository>,
    ) -> Self {
        Self {
            wish_item_repo,
            category_repo,
        }
    }

    pub async fn execute(&self, input: AddWishItemInput) -> Result<WishItemOutput, UseCaseError> {
        let category = self
            .category_repo
            .find_by_id(input.category_id)
            .await?
            .ok_or(UseCaseError::CategoryNotFound(input.category_id))?;

        let price = Price::new(input.price).map_err(|_| UseCaseError::InvalidPrice)?;
        let memo = Memo::new(input.memo.unwrap_or_default());

        let (item, _events) = WishItem::new(input.name, price, category.clone(), memo)
            .map_err(|e| UseCaseError::DomainError(e.to_string()))?;

        self.wish_item_repo.save(&item).await?;

        Ok(WishItemOutput {
            id: item.id(),
            name: item.name().to_string(),
            price: item.price().value(),
            category_name: category.name,
            status: format!("{:?}", item.status()),
            memo: item.memo().value().to_string(),
            added_at: item.added_at().to_rfc3339(),
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UseCaseError {
    #[error("category not found: {0}")]
    CategoryNotFound(Uuid),
    #[error("invalid price")]
    InvalidPrice,
    #[error("domain error: {0}")]
    DomainError(String),
    #[error("repository error: {0}")]
    Repository(#[from] crate::domain::repositories::wish_item_repository::RepositoryError),
}
