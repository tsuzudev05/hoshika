#![allow(dead_code)]
//! PurchaseRecord エンティティ（Budget 集約の内部エンティティ）
use crate::domain::value_objects::{Memo, Price};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PurchaseRecord {
    pub id: Uuid,
    pub budget_id: Uuid,
    pub wish_item_id: Uuid,
    pub actual_price: Price,
    pub memo: Memo,
    pub purchased_at: DateTime<Utc>,
}
