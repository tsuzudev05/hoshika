#![allow(dead_code)]
use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::domain::repositories::wish_item_repository::RepositoryError;
use crate::domain::repositories::CategoryRepository;
use crate::domain::value_objects::Category;

pub struct InMemoryCategoryRepository {
    store: Arc<Mutex<HashMap<Uuid, Category>>>,
}

impl InMemoryCategoryRepository {
    pub fn new() -> Self {
        Self {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// テスト用にカテゴリを初期データとして投入する
    pub fn with_categories(categories: Vec<Category>) -> Self {
        let map = categories.into_iter().map(|c| (c.id, c)).collect();
        Self {
            store: Arc::new(Mutex::new(map)),
        }
    }
}

#[async_trait]
impl CategoryRepository for InMemoryCategoryRepository {
    async fn find_all(&self) -> Result<Vec<Category>, RepositoryError> {
        let store = self.store.lock().await;
        Ok(store.values().cloned().collect())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Category>, RepositoryError> {
        let store = self.store.lock().await;
        Ok(store.get(&id).cloned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_category(name: &str) -> Category {
        Category {
            id: Uuid::new_v4(),
            name: name.to_string(),
        }
    }

    #[tokio::test]
    async fn with_categories_seeds_data() {
        let cat = make_category("書籍");
        let id = cat.id;
        let repo = InMemoryCategoryRepository::with_categories(vec![cat]);

        let found = repo.find_by_id(id).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "書籍");
    }

    #[tokio::test]
    async fn find_all_returns_all() {
        let repo = InMemoryCategoryRepository::with_categories(vec![
            make_category("書籍"),
            make_category("ガジェット"),
        ]);

        let all = repo.find_all().await.unwrap();
        assert_eq!(all.len(), 2);
    }

    #[tokio::test]
    async fn find_by_id_returns_none_when_missing() {
        let repo = InMemoryCategoryRepository::new();
        let found = repo.find_by_id(Uuid::new_v4()).await.unwrap();
        assert!(found.is_none());
    }
}
