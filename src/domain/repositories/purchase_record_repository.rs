#![allow(dead_code)]
//! PurchaseRecordRepository trait
use crate::domain::entities::purchase_record::PurchaseRecord;
use crate::domain::repositories::wish_item_repository::RepositoryError;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait PurchaseRecordRepository: Send + Sync {
    async fn find_by_id(
        &self,
        user_id: &str,
        id: Uuid,
    ) -> Result<Option<PurchaseRecord>, RepositoryError>;
    async fn save(&self, record: &PurchaseRecord) -> Result<(), RepositoryError>;
}
