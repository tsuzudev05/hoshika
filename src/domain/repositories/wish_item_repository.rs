#![allow(dead_code)]
//! WishItemRepository trait（DBを知らないインターフェース）
//! impl は infrastructure/db/ に置く。
use crate::domain::entities::wish_item::WishItem;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait WishItemRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<WishItem>, RepositoryError>;
    async fn find_all(&self) -> Result<Vec<WishItem>, RepositoryError>;
    async fn save(&self, item: &WishItem) -> Result<(), RepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError>;
}

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    /// 指定した ID に対応するレコードが存在しない
    #[error("not found")]
    NotFound,
    /// ユニーク制約違反（SQLSTATE 23505）など、重複・競合によって操作が拒否された
    #[error("conflict: {0}")]
    Conflict(String),
    /// DB 接続失敗やクエリ構築エラーなど、上記に分類できない予期しないエラー
    #[error("unexpected error: {0}")]
    Unexpected(String),
}
