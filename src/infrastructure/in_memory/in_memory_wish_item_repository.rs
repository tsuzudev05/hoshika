#![allow(dead_code)]
//! InMemoryWishItemRepository — WishItemRepository のインメモリ実装
//! テスト用。DBなしでドメイン・ユースケース層を検証する目的で使う。
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::wish_item::WishItem;
use crate::domain::repositories::wish_item_repository::RepositoryError;
use crate::domain::repositories::WishItemRepository;
use crate::infrastructure::in_memory::in_memory_store::InMemoryStore;

/// WishItem をメモリ上で管理するリポジトリ。
/// 内部の CRUD 操作は [`InMemoryStore`] に委譲する。
pub struct InMemoryWishItemRepository {
    store: InMemoryStore<WishItem>,
}

impl InMemoryWishItemRepository {
    /// 空のリポジトリを生成する。
    pub fn new() -> Self {
        Self {
            store: InMemoryStore::new(),
        }
    }
}

#[async_trait]
impl WishItemRepository for InMemoryWishItemRepository {
    /// 指定した ID の WishItem を返す。存在しない、または他ユーザーの所有物の場合は `Ok(None)`。
    async fn find_by_id(
        &self,
        user_id: &str,
        id: Uuid,
    ) -> Result<Option<WishItem>, RepositoryError> {
        Ok(self
            .store
            .find_by_id(id)
            .await
            .filter(|item| item.user_id() == user_id))
    }

    /// 指定したユーザーが保存している全 WishItem を返す。
    async fn find_all(&self, user_id: &str) -> Result<Vec<WishItem>, RepositoryError> {
        Ok(self
            .store
            .find_all()
            .await
            .into_iter()
            .filter(|item| item.user_id() == user_id)
            .collect())
    }

    /// WishItem を保存する。同じ ID が既に存在する場合は上書きする（upsert）。
    async fn save(&self, item: &WishItem) -> Result<(), RepositoryError> {
        self.store.save(item.id(), item.clone()).await;
        Ok(())
    }

    /// 指定した ID の WishItem を削除する。
    ///
    /// # Errors
    /// - `RepositoryError::NotFound` — 指定した ID が存在しない、または他ユーザーの所有物の場合
    async fn delete(&self, user_id: &str, id: Uuid) -> Result<(), RepositoryError> {
        match self.store.find_by_id(id).await {
            Some(item) if item.user_id() == user_id => {
                self.store.remove(id).await;
                Ok(())
            }
            _ => Err(RepositoryError::NotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::{Category, Memo, Price, WishItemName};

    const USER: &str = "user-1";
    const OTHER_USER: &str = "user-2";

    fn make_item_for(user_id: &str) -> WishItem {
        let (item, _) = WishItem::new(
            user_id.to_string(),
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

    fn make_item() -> WishItem {
        make_item_for(USER)
    }

    // --- find_all ---

    #[tokio::test]
    async fn find_all_when_empty_returns_empty_vec() {
        let repo = InMemoryWishItemRepository::new();
        let all = repo.find_all(USER).await.unwrap();
        assert!(all.is_empty());
    }

    #[tokio::test]
    async fn find_all_returns_all_saved_items() {
        let repo = InMemoryWishItemRepository::new();
        repo.save(&make_item()).await.unwrap();
        repo.save(&make_item()).await.unwrap();

        let all = repo.find_all(USER).await.unwrap();
        assert_eq!(all.len(), 2);
    }

    #[tokio::test]
    async fn find_all_does_not_return_other_users_items() {
        let repo = InMemoryWishItemRepository::new();
        repo.save(&make_item_for(USER)).await.unwrap();
        repo.save(&make_item_for(OTHER_USER)).await.unwrap();

        let all = repo.find_all(USER).await.unwrap();
        assert_eq!(all.len(), 1);
    }

    // --- find_by_id ---

    #[tokio::test]
    async fn find_by_id_returns_saved_item() {
        let repo = InMemoryWishItemRepository::new();
        let item = make_item();
        let id = item.id();
        repo.save(&item).await.unwrap();

        let found = repo.find_by_id(USER, id).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().id(), id);
    }

    #[tokio::test]
    async fn find_by_id_returns_none_when_missing() {
        let repo = InMemoryWishItemRepository::new();
        let result = repo.find_by_id(USER, Uuid::new_v4()).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn find_by_id_returns_none_for_other_users_item() {
        let repo = InMemoryWishItemRepository::new();
        let item = make_item_for(OTHER_USER);
        let id = item.id();
        repo.save(&item).await.unwrap();

        let result = repo.find_by_id(USER, id).await.unwrap();
        assert!(result.is_none());
    }

    // --- save ---

    #[tokio::test]
    async fn save_overwrites_existing() {
        let repo = InMemoryWishItemRepository::new();
        let mut item = make_item();
        repo.save(&item).await.unwrap();

        item.review(true).unwrap();
        repo.save(&item).await.unwrap();

        let found = repo.find_by_id(USER, item.id()).await.unwrap().unwrap();
        assert_eq!(found.status(), item.status());
    }

    #[tokio::test]
    async fn save_multiple_items_are_independent() {
        let repo = InMemoryWishItemRepository::new();
        let item1 = make_item();
        let item2 = make_item();
        let id1 = item1.id();
        let id2 = item2.id();

        repo.save(&item1).await.unwrap();
        repo.save(&item2).await.unwrap();

        assert!(repo.find_by_id(USER, id1).await.unwrap().is_some());
        assert!(repo.find_by_id(USER, id2).await.unwrap().is_some());
    }

    // --- delete ---

    #[tokio::test]
    async fn delete_removes_item() {
        let repo = InMemoryWishItemRepository::new();
        let item = make_item();
        let id = item.id();
        repo.save(&item).await.unwrap();

        repo.delete(USER, id).await.unwrap();

        assert!(repo.find_by_id(USER, id).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn delete_returns_not_found_for_missing_id() {
        let repo = InMemoryWishItemRepository::new();
        let result = repo.delete(USER, Uuid::new_v4()).await;
        assert!(matches!(result, Err(RepositoryError::NotFound)));
    }

    #[tokio::test]
    async fn delete_returns_not_found_for_other_users_item() {
        let repo = InMemoryWishItemRepository::new();
        let item = make_item_for(OTHER_USER);
        let id = item.id();
        repo.save(&item).await.unwrap();

        let result = repo.delete(USER, id).await;
        assert!(matches!(result, Err(RepositoryError::NotFound)));
        assert!(repo.find_by_id(OTHER_USER, id).await.unwrap().is_some());
    }

    #[tokio::test]
    async fn delete_does_not_affect_other_items() {
        let repo = InMemoryWishItemRepository::new();
        let item1 = make_item();
        let item2 = make_item();
        let id2 = item2.id();
        repo.save(&item1).await.unwrap();
        repo.save(&item2).await.unwrap();

        repo.delete(USER, item1.id()).await.unwrap();

        assert!(repo.find_by_id(USER, id2).await.unwrap().is_some());
        assert_eq!(repo.find_all(USER).await.unwrap().len(), 1);
    }
}
