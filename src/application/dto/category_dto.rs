//! Category DTO（HTTP を知らない入出力型）
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct CategoryOutput {
    pub id: Uuid,
    pub name: String,
}
