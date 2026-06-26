#![allow(dead_code)]
//! ReviewWishItem ユースケース（衝動買い防止チェック）
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::repositories::wish_item_repository::RepositoryError;
use crate::domain::repositories::WishItemRepository;

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

        let _events = item
            .review(still_want)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use uuid::Uuid;

    use crate::domain::entities::wish_item::WishItem;
    use crate::domain::value_objects::{Category, Memo, Price, WishItemName, WishItemStatus};
    use crate::infrastructure::in_memory::InMemoryWishItemRepository;

    fn make_inbox_item() -> WishItem {
        let (item, _) = WishItem::new(
            WishItemName::new("テスト本").unwrap(),
            Price::new(2000).unwrap(),
            Category {
                id: Uuid::new_v4(),
                name: "書籍".to_string(),
            },
            Memo::new(""),
        );
        item
    }

    // --- 正常系 ---

    #[tokio::test]
    async fn execute_with_still_want_true_transitions_to_next_to_buy() {
        let repo = Arc::new(InMemoryWishItemRepository::new());
        let item = make_inbox_item();
        let id = item.id();
        repo.save(&item).await.unwrap();

        ReviewWishItemUseCase::new(repo.clone())
            .execute(id, true)
            .await
            .unwrap();

        let updated = repo.find_by_id(id).await.unwrap().unwrap();
        assert_eq!(updated.status(), &WishItemStatus::NextToBuy);
    }

    #[tokio::test]
    async fn execute_with_still_want_false_transitions_to_on_hold() {
        let repo = Arc::new(InMemoryWishItemRepository::new());
        let item = make_inbox_item();
        let id = item.id();
        repo.save(&item).await.unwrap();

        ReviewWishItemUseCase::new(repo.clone())
            .execute(id, false)
            .await
            .unwrap();

        let updated = repo.find_by_id(id).await.unwrap().unwrap();
        assert_eq!(updated.status(), &WishItemStatus::OnHold);
    }

    // --- 異常系 ---

    #[tokio::test]
    async fn execute_returns_not_found_for_unknown_id() {
        let repo = Arc::new(InMemoryWishItemRepository::new());
        let result = ReviewWishItemUseCase::new(repo)
            .execute(Uuid::new_v4(), true)
            .await;
        assert!(matches!(result, Err(ReviewError::NotFound(_))));
    }

    #[tokio::test]
    async fn execute_returns_domain_error_when_reviewing_non_inbox_item() {
        let repo = Arc::new(InMemoryWishItemRepository::new());
        let mut item = make_inbox_item();
        item.review(true).unwrap(); // Inbox → NextToBuy
        let id = item.id();
        repo.save(&item).await.unwrap();

        // NextToBuy から review するのは無効なステータス遷移
        let result = ReviewWishItemUseCase::new(repo)
            .execute(id, true)
            .await;
        assert!(matches!(result, Err(ReviewError::DomainError(_))));
    }
}
