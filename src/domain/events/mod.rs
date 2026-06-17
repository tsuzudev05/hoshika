//! ドメインイベント定義
//!
//! 「何が起きたか」を記録する。MVP ではアプリケーション層で処理し、
//! イベントストアへの永続化は Phase 02 以降の検討事項。
use crate::domain::value_objects::{Price, YearMonth};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum DomainEvent {
    // WishList コンテキスト
    ItemAdded { wish_item_id: Uuid },
    ItemReviewed { wish_item_id: Uuid, still_want: bool },
    ItemMovedToNextToBuy { wish_item_id: Uuid },
    ItemArchived { wish_item_id: Uuid },
    ItemPurchased { wish_item_id: Uuid },

    // Budget コンテキスト
    BudgetSet { budget_id: Uuid, year_month: YearMonth, amount: Price },
    PurchaseRecorded { budget_id: Uuid, actual_price: Price },
    BudgetExceeded { budget_id: Uuid },
}
