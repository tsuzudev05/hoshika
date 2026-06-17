//! Budget DTO
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct SetBudgetInput {
    pub year: u16,
    pub month: u8,
    pub amount: u64,
}

#[derive(Debug, Serialize)]
pub struct BudgetStatusOutput {
    pub id: Uuid,
    pub year: u16,
    pub month: u8,
    pub amount: u64,
    pub balance: i64,
    pub is_exceeded: bool,
}
