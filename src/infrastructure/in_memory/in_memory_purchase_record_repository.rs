#![allow(dead_code)]
//! InMemoryPurchaseRecordRepository — PurchaseRecordRepository のインメモリ実装
//! テスト用。DBなしでドメイン・ユースケース層の動作を検証できる。
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::purchase_record::PurchaseRecord;
use crate::domain::repositories::wish_item_repository::RepositoryError;
use crate::domain::repositories::PurchaseRecordRepository;
use crate::infrastructure::in_memory::in_memory_store::InMemoryStore;

/// PurchaseRecord をメモリ上で管理するリポジトリ。
/// 内部の CRUD 操作は [`InMemoryStore`] に委譲する。
pub struct InMemoryPurchaseRecordRepository {
    store: InMemoryStore<PurchaseRecord>,
}

impl InMemoryPurchaseRecordRepository {
    /// 空のリポジトリを生成する。
    pub fn new() -> Self {
        Self {
            store: InMemoryStore::new(),
        }
    }

    /// テスト用の初期データを持つリポジトリを生成する。
    pub fn with_records(records: Vec<PurchaseRecord>) -> Self {
        Self {
            store: InMemoryStore::with_items(records.into_iter().map(|r| (r.id(), r))),
        }
    }

    /// 保存されている全レコードを返す（テストでの検証用。トレイトには含めない）。
    pub async fn find_all(&self) -> Vec<PurchaseRecord> {
        self.store.find_all().await
    }
}

#[async_trait]
impl PurchaseRecordRepository for InMemoryPurchaseRecordRepository {
    async fn find_by_id(
        &self,
        user_id: &str,
        id: Uuid,
    ) -> Result<Option<PurchaseRecord>, RepositoryError> {
        Ok(self
            .store
            .find_by_id(id)
            .await
            .filter(|r| r.user_id() == user_id))
    }

    async fn save(&self, record: &PurchaseRecord) -> Result<(), RepositoryError> {
        self.store.save(record.id(), record.clone()).await;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::{Memo, Price};

    const USER: &str = "user-1";
    const OTHER_USER: &str = "user-2";

    fn make_record_for(user_id: &str) -> PurchaseRecord {
        PurchaseRecord::new(
            user_id.to_string(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            Price::new(1000).unwrap(),
            Memo::new(""),
        )
    }

    fn make_record() -> PurchaseRecord {
        make_record_for(USER)
    }

    #[tokio::test]
    async fn find_by_id_returns_saved_record() {
        let repo = InMemoryPurchaseRecordRepository::new();
        let record = make_record();
        let id = record.id();
        repo.save(&record).await.unwrap();

        let found = repo.find_by_id(USER, id).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().id(), id);
    }

    #[tokio::test]
    async fn find_by_id_returns_none_when_missing() {
        let repo = InMemoryPurchaseRecordRepository::new();
        let result = repo.find_by_id(USER, Uuid::new_v4()).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn find_by_id_returns_none_for_other_users_record() {
        let repo = InMemoryPurchaseRecordRepository::new();
        let record = make_record_for(OTHER_USER);
        let id = record.id();
        repo.save(&record).await.unwrap();

        let result = repo.find_by_id(USER, id).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn with_records_seeds_data() {
        let record = make_record();
        let id = record.id();
        let repo = InMemoryPurchaseRecordRepository::with_records(vec![record]);

        let found = repo.find_by_id(USER, id).await.unwrap();
        assert!(found.is_some());
    }
}
