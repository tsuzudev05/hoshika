#![allow(dead_code)]
use crate::domain::value_objects::{Memo, Price};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// 購入記録
/// PurchaseRecord エンティティ（Budget 集約の内部エンティティ）
/// 購入 1 件の記録。Budget.record_purchase() から生成される。
/// 同一性: id による
#[derive(Debug, Clone)]
pub struct PurchaseRecord {
    id: Uuid,
    user_id: String,
    budget_id: Uuid,
    wish_item_id: Uuid,
    actual_price: Price,
    memo: Memo,
    purchased_at: DateTime<Utc>,
}

impl PurchaseRecord {
    pub fn new(
        user_id: String,
        budget_id: Uuid,
        wish_item_id: Uuid,
        actual_price: Price,
        memo: Memo,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            budget_id,
            wish_item_id,
            actual_price,
            memo,
            purchased_at: Utc::now(),
        }
    }

    /// DBからの再構築。Infrastructure 層のリポジトリからのみ呼ばれる。
    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: Uuid,
        user_id: String,
        budget_id: Uuid,
        wish_item_id: Uuid,
        actual_price: Price,
        memo: Memo,
        purchased_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            user_id,
            budget_id,
            wish_item_id,
            actual_price,
            memo,
            purchased_at,
        }
    }

    // --- getters ---

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    pub fn budget_id(&self) -> Uuid {
        self.budget_id
    }

    pub fn wish_item_id(&self) -> Uuid {
        self.wish_item_id
    }

    pub fn actual_price(&self) -> &Price {
        &self.actual_price
    }

    pub fn memo(&self) -> &Memo {
        &self.memo
    }

    pub fn purchased_at(&self) -> DateTime<Utc> {
        self.purchased_at
    }
}

impl PartialEq for PurchaseRecord {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for PurchaseRecord {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::{Memo, Price};

    #[test]
    fn new_record_has_unique_id() {
        let r1 = PurchaseRecord::new(
            "test-user".to_string(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            Price::new(1000).unwrap(),
            Memo::new(""),
        );
        let r2 = PurchaseRecord::new(
            "test-user".to_string(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            Price::new(1000).unwrap(),
            Memo::new(""),
        );
        assert_ne!(r1, r2);
    }
}
