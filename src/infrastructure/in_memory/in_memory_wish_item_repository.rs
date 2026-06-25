#![allow(dead_code)]
//! InMemoryWishItemRepository — WishItemRepository のインメモリ実装
//! テスト用。DBなしでドメイン・ユースケース層を検証する目的で使う。
use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::domain::entities::wish_item::WishItem;
use crate::domain::repositories::wish_item_repository::RepositoryError;
use crate::domain::repositories::WishItemRepository;

/// WishItem をメモリ上の HashMap で管理するリポジトリ。
/// 複数の非同期タスクから安全にアクセスできるよう `Arc<Mutex<...>>` で保護する。
pub struct InMemoryWishItemRepository {
    store: Arc<Mutex<HashMap<Uuid, WishItem>>>,
}

impl InMemoryWishItemRepository {
    /// 空のリポジトリを生成する。
    pub fn new() -> Self {
        Self {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl WishItemRepository for InMemoryWishItemRepository {
    /// 指定した ID の WishItem を返す。存在しない場合は `Ok(None)`。
    async fn find_by_id(&self, id: Uuid) -> Result<Option<WishItem>, RepositoryError> {
        let store = self.store.lock().await;
        Ok(store.get(&id).cloned())
    }

    /// 保存されている全 WishItem を返す。
    async fn find_all(&self) -> Result<Vec<WishItem>, RepositoryError> {
        let store = self.store.lock().await;
        Ok(store.values().cloned().collect())
    }

    /// WishItem を保存する。同じ ID が既に存在する場合は上書きする（upsert）。
    async fn save(&self, item: &WishItem) -> Result<(), RepositoryError> {
        let mut store = self.store.lock().await;
        store.insert(item.id(), item.clone());
        Ok(())
    }

    /// 指定した ID の WishItem を削除する。
    ///
    /// # Errors
    /// - `RepositoryError::NotFound` — 指定した ID が存在しない場合
    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        let mut store = self.store.lock().await;
        if store.remove(&id).is_none() {
            return Err(RepositoryError::NotFound);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::wish_item::WishItem;
    use crate::domain::value_objects::{Category, Memo, Price, WishItemName};

    fn make_item() -> WishItem {
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

    #[tokio::test]
    async fn save_and_find_by_id() {
        let repo = InMemoryWishItemRepository::new();
        let item = make_item();
        let id = item.id();

        repo.save(&item).await.unwrap();

        let found = repo.find_by_id(id).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().id(), id);
    }

    #[tokio::test]
    async fn find_by_id_returns_none_when_missing() {
        let repo = InMemoryWishItemRepository::new();
        let result = repo.find_by_id(Uuid::new_v4()).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn find_all_returns_all_saved_items() {
        let repo = InMemoryWishItemRepository::new();
        repo.save(&make_item()).await.unwrap();
        repo.save(&make_item()).await.unwrap();

        let all = repo.find_all().await.unwrap();
        assert_eq!(all.len(), 2);
    }

    #[tokio::test]
    async fn delete_removes_item() {
        let repo = InMemoryWishItemRepository::new();
        let item = make_item();
        let id = item.id();
        repo.save(&item).await.unwrap();

        repo.delete(id).await.unwrap();

        assert!(repo.find_by_id(id).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn delete_returns_not_found_for_missing_id() {
        let repo = InMemoryWishItemRepository::new();
        let result = repo.delete(Uuid::new_v4()).await;
        assert!(matches!(result, Err(RepositoryError::NotFound)));
    }

    #[tokio::test]
    async fn save_overwrites_existing() {
        let repo = InMemoryWishItemRepository::new();
        let mut item = make_item();
        repo.save(&item).await.unwrap();

        item.review(true).unwrap();
        repo.save(&item).await.unwrap();

        let found = repo.find_by_id(item.id()).await.unwrap().unwrap();
        assert_eq!(found.status(), item.status());
    }
}
