#![allow(dead_code)]
//! AddWishItem ユースケース
//! HTTP を知らない。引数は DTO、戻り値も DTO。
use std::sync::Arc;
use uuid::Uuid;

use crate::application::dto::{AddWishItemInput, WishItemOutput};
use crate::domain::{
    entities::wish_item::WishItem,
    repositories::{CategoryRepository, WishItemRepository},
    value_objects::{Memo, Price, WishItemName},
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

    pub async fn execute(
        &self,
        user_id: &str,
        input: AddWishItemInput,
    ) -> Result<WishItemOutput, UseCaseError> {
        let category = self
            .category_repo
            .find_by_id(input.category_id)
            .await?
            .ok_or(UseCaseError::CategoryNotFound(input.category_id))?;

        let name =
            WishItemName::new(input.name).map_err(|e| UseCaseError::DomainError(e.to_string()))?;
        let price = Price::new(input.price).map_err(|_| UseCaseError::InvalidPrice)?;
        let memo = Memo::new(input.memo.unwrap_or_default());

        let (item, _events) =
            WishItem::new(user_id.to_string(), name, price, category.clone(), memo);

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use uuid::Uuid;

    use crate::application::dto::AddWishItemInput;
    use crate::domain::value_objects::Category;
    use crate::infrastructure::in_memory::{
        InMemoryCategoryRepository, InMemoryWishItemRepository,
    };

    fn make_category() -> Category {
        Category {
            id: Uuid::new_v4(),
            name: "書籍".to_string(),
        }
    }

    fn make_use_case(category: Category) -> (Arc<InMemoryWishItemRepository>, AddWishItemUseCase) {
        let wish_item_repo = Arc::new(InMemoryWishItemRepository::new());
        let category_repo = Arc::new(InMemoryCategoryRepository::with_categories(vec![category]));
        let use_case = AddWishItemUseCase::new(wish_item_repo.clone(), category_repo);
        (wish_item_repo, use_case)
    }

    // --- 正常系 ---

    #[tokio::test]
    async fn execute_returns_correct_output() {
        let category = make_category();
        let category_id = category.id;
        let (_, use_case) = make_use_case(category);

        let output = use_case
            .execute(
                "user-1",
                AddWishItemInput {
                    name: "Rustプログラミング入門".to_string(),
                    price: 3000,
                    category_id,
                    memo: None,
                },
            )
            .await
            .unwrap();

        assert_eq!(output.name, "Rustプログラミング入門");
        assert_eq!(output.price, 3000);
        assert_eq!(output.category_name, "書籍");
        assert_eq!(output.status, "Inbox");
        assert!(output.memo.is_empty());
    }

    #[tokio::test]
    async fn execute_saves_item_to_repository() {
        let category = make_category();
        let category_id = category.id;
        let (wish_item_repo, use_case) = make_use_case(category);

        let output = use_case
            .execute(
                "user-1",
                AddWishItemInput {
                    name: "テスト商品".to_string(),
                    price: 1000,
                    category_id,
                    memo: Some("メモあり".to_string()),
                },
            )
            .await
            .unwrap();

        let saved = wish_item_repo
            .find_by_id("user-1", output.id)
            .await
            .unwrap();
        assert!(saved.is_some());
        assert_eq!(output.memo, "メモあり");
    }

    // --- 異常系 ---

    #[tokio::test]
    async fn execute_returns_error_when_category_not_found() {
        let wish_item_repo = Arc::new(InMemoryWishItemRepository::new());
        let category_repo = Arc::new(InMemoryCategoryRepository::new());
        let use_case = AddWishItemUseCase::new(wish_item_repo, category_repo);

        let result = use_case
            .execute(
                "user-1",
                AddWishItemInput {
                    name: "テスト".to_string(),
                    price: 1000,
                    category_id: Uuid::new_v4(),
                    memo: None,
                },
            )
            .await;

        assert!(matches!(result, Err(UseCaseError::CategoryNotFound(_))));
    }

    #[tokio::test]
    async fn execute_returns_domain_error_for_empty_name() {
        let category = make_category();
        let category_id = category.id;
        let (_, use_case) = make_use_case(category);

        let result = use_case
            .execute(
                "user-1",
                AddWishItemInput {
                    name: "".to_string(),
                    price: 1000,
                    category_id,
                    memo: None,
                },
            )
            .await;

        assert!(matches!(result, Err(UseCaseError::DomainError(_))));
    }
}
