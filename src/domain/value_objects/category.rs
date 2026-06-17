//! Category 値オブジェクト
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
}
