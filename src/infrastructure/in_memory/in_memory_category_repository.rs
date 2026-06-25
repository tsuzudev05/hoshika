#![allow(dead_code)]
//! InMemoryCategoryRepository — CategoryRepository のインメモリ実装
//! テスト用。DBなしでドメイン・ユースケース層を検証する目的で使う。
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::repositories::wish_item_repository::RepositoryError;
use crate::domain::repositories::CategoryRepository;
use crate::domain::value_objects::Category;
use crate::infrastructure::in_memory::in_memory_store::InMemoryStore;

/// Category をメモリ上で管理するリポジトリ。
/// 内部の CRUD 操作は [`InMemoryStore`] に委譲する。
pub struct InMemoryCategoryRepository {
    store: InMemoryStore<Category>,
}

impl InMemoryCategoryRepository {
    /// 空のリポジトリを生成する。
    pub fn new() -> Self {
        Self {
            store: InMemoryStore::new(),
        }
    }

    /// テスト用の初期データを持つリポジトリを生成する。
    ///
    /// # Parameters
    /// - `categories` — 初期状態として投入する Category のリスト
    pub fn with_categories(categories: Vec<Category>) -> Self {
        Self {
            store: InMemoryStore::with_items(categories.into_iter().map(|c| (c.id, c))),
        }
    }
}

#[async_trait]
impl CategoryRepository for InMemoryCategoryRepository {
    /// 保存されている全 Category を返す。
    async fn find_all(&self) -> Result<Vec<Category>, RepositoryError> {
        Ok(self.store.find_all().await)
    }

    /// 指定した ID の Category を返す。存在しない場合は `Ok(None)`。
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Category>, RepositoryError> {
        Ok(self.store.find_by_id(id).await)
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

    // --- find_all ---

    #[tokio::test]
    async fn find_all_when_empty_returns_empty_vec() {
        let repo = InMemoryCategoryRepository::new();
        let all = repo.find_all().await.unwrap();
        assert!(all.is_empty());
    }

    #[tokio::test]
    async fn find_all_returns_all_seeded_categories() {
        let repo = InMemoryCategoryRepository::with_categories(vec![
            make_category("書籍"),
            make_category("ガジェット"),
        ]);

        let all = repo.find_all().await.unwrap();
        assert_eq!(all.len(), 2);
    }

    // --- find_by_id ---

    #[tokio::test]
    async fn find_by_id_returns_correct_category() {
        let cat1 = make_category("書籍");
        let cat2 = make_category("ガジェット");
        let target_id = cat1.id;
        let repo = InMemoryCategoryRepository::with_categories(vec![cat1, cat2]);

        let found = repo.find_by_id(target_id).await.unwrap().unwrap();
        assert_eq!(found.name, "書籍");
    }

    #[tokio::test]
    async fn find_by_id_returns_none_when_missing() {
        let repo = InMemoryCategoryRepository::new();
        let found = repo.find_by_id(Uuid::new_v4()).await.unwrap();
        assert!(found.is_none());
    }

    // --- with_categories ---

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
    async fn with_categories_empty_vec_creates_empty_repo() {
        let repo = InMemoryCategoryRepository::with_categories(vec![]);
        let all = repo.find_all().await.unwrap();
        assert!(all.is_empty());
    }
}
