#![allow(dead_code)]
//! WishItem DTO（HTTP を知らない入出力型）
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct AddWishItemInput {
    pub name: String,
    pub price: u64,
    pub category_id: Uuid,
    pub memo: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReviewWishItemInput {
    pub still_want: bool,
}

#[derive(Debug, Serialize)]
pub struct WishItemOutput {
    pub id: Uuid,
    pub name: String,
    pub price: u64,
    pub category_name: String,
    pub status: String,
    pub memo: String,
    pub added_at: String,
}
