#![allow(dead_code)]
use crate::domain::events::DomainEvent;
use crate::domain::value_objects::{Category, Memo, Price, WishItemName, WishItemStatus};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// 欲しいもの
/// WishItem エンティティ（集約ルート）
/// 不変条件:
/// - name は空文字列不可
/// - ステータス遷移は各メソッドを通じてのみ行う（フィールド直接書き換え不可）
///
/// 同一性: id による（属性が異なっても id が同じなら同一エンティティ）
#[derive(Debug, Clone)]
pub struct WishItem {
    id: Uuid,
    name: WishItemName,
    price: Price,
    category: Category,
    status: WishItemStatus,
    memo: Memo,
    added_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl WishItem {
    /// DBからの再構築。不変条件の適用・イベント発行は行わない。
    /// Infrastructure 層のリポジトリからのみ呼ばれる。
    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: Uuid,
        name: WishItemName,
        price: Price,
        category: Category,
        status: WishItemStatus,
        memo: Memo,
        added_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            name,
            price,
            category,
            status,
            memo,
            added_at,
            updated_at,
        }
    }

    /// 新規 WishItem を作成する。作成時のステータスは必ず `Inbox`。
    /// name のバリデーション（空文字列不可）は WishItemName::new() が担う。
    pub fn new(
        name: WishItemName,
        price: Price,
        category: Category,
        memo: Memo,
    ) -> (Self, Vec<DomainEvent>) {
        let now = Utc::now();
        let item = Self {
            id: Uuid::new_v4(),
            name,
            price,
            category,
            status: WishItemStatus::Inbox,
            memo,
            added_at: now,
            updated_at: now,
        };
        let events = vec![DomainEvent::ItemAdded {
            wish_item_id: item.id,
        }];
        (item, events)
    }

    // --- getters ---

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.value()
    }

    pub fn price(&self) -> &Price {
        &self.price
    }

    pub fn category(&self) -> &Category {
        &self.category
    }

    pub fn status(&self) -> &WishItemStatus {
        &self.status
    }

    pub fn memo(&self) -> &Memo {
        &self.memo
    }

    pub fn added_at(&self) -> DateTime<Utc> {
        self.added_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    // --- state transitions ---

    /// 衝動買い防止チェックを実施する（Inbox → NextToBuy or OnHold）
    pub fn review(&mut self, still_want: bool) -> Result<Vec<DomainEvent>, WishItemError> {
        if self.status != WishItemStatus::Inbox {
            return Err(WishItemError::InvalidTransition {
                from: self.status.clone(),
                to: if still_want {
                    WishItemStatus::NextToBuy
                } else {
                    WishItemStatus::OnHold
                },
            });
        }
        self.status = if still_want {
            WishItemStatus::NextToBuy
        } else {
            WishItemStatus::OnHold
        };
        self.updated_at = Utc::now();
        Ok(vec![DomainEvent::ItemReviewed {
            wish_item_id: self.id,
            still_want,
        }])
    }

    /// NextToBuy に昇格させる（OnHold → NextToBuy）
    pub fn move_to_next_to_buy(&mut self) -> Result<Vec<DomainEvent>, WishItemError> {
        if self.status != WishItemStatus::OnHold {
            return Err(WishItemError::InvalidTransition {
                from: self.status.clone(),
                to: WishItemStatus::NextToBuy,
            });
        }
        self.status = WishItemStatus::NextToBuy;
        self.updated_at = Utc::now();
        Ok(vec![DomainEvent::ItemMovedToNextToBuy {
            wish_item_id: self.id,
        }])
    }

    /// アーカイブする（Inbox / OnHold / NextToBuy → Archived）
    pub fn archive(&mut self) -> Result<Vec<DomainEvent>, WishItemError> {
        match self.status {
            WishItemStatus::Purchased | WishItemStatus::Archived => {
                return Err(WishItemError::InvalidTransition {
                    from: self.status.clone(),
                    to: WishItemStatus::Archived,
                })
            }
            _ => {}
        }
        self.status = WishItemStatus::Archived;
        self.updated_at = Utc::now();
        Ok(vec![DomainEvent::ItemArchived {
            wish_item_id: self.id,
        }])
    }

    /// 購入済みにする（NextToBuy → Purchased）
    pub fn purchase(&mut self) -> Result<Vec<DomainEvent>, WishItemError> {
        if self.status != WishItemStatus::NextToBuy {
            return Err(WishItemError::InvalidTransition {
                from: self.status.clone(),
                to: WishItemStatus::Purchased,
            });
        }
        self.status = WishItemStatus::Purchased;
        self.updated_at = Utc::now();
        Ok(vec![DomainEvent::ItemPurchased {
            wish_item_id: self.id,
        }])
    }
}

/// エンティティの同一性は id で決まる（属性が違っても id が同じなら同一）
impl PartialEq for WishItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for WishItem {}

#[derive(Debug, thiserror::Error)]
pub enum WishItemError {
    #[error("invalid status transition: {from:?} -> {to:?}")]
    InvalidTransition {
        from: WishItemStatus,
        to: WishItemStatus,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::{Category, Memo, Price, WishItemName};

    fn make_item() -> WishItem {
        let category = Category {
            id: Uuid::new_v4(),
            name: "書籍".to_string(),
        };
        let (item, _) = WishItem::new(
            WishItemName::new("テスト本").unwrap(),
            Price::new(2000).unwrap(),
            category,
            Memo::new(""),
        );
        item
    }

    // --- new ---

    #[test]
    fn new_item_status_is_inbox() {
        let item = make_item();
        assert_eq!(item.status(), &WishItemStatus::Inbox);
    }

    // --- review ---

    #[test]
    fn review_inbox_to_next_to_buy() {
        let mut item = make_item();
        let events = item.review(true).unwrap();
        assert_eq!(item.status(), &WishItemStatus::NextToBuy);
        assert!(matches!(
            events[0],
            DomainEvent::ItemReviewed {
                still_want: true,
                ..
            }
        ));
    }

    #[test]
    fn review_inbox_to_on_hold() {
        let mut item = make_item();
        let events = item.review(false).unwrap();
        assert_eq!(item.status(), &WishItemStatus::OnHold);
        assert!(matches!(
            events[0],
            DomainEvent::ItemReviewed {
                still_want: false,
                ..
            }
        ));
    }

    #[test]
    fn cannot_review_non_inbox() {
        let mut item = make_item();
        item.review(true).unwrap();
        assert!(item.review(true).is_err());
    }

    // --- move_to_next_to_buy ---

    #[test]
    fn move_to_next_to_buy_from_on_hold() {
        let mut item = make_item();
        item.review(false).unwrap(); // Inbox → OnHold
        let events = item.move_to_next_to_buy().unwrap();
        assert_eq!(item.status(), &WishItemStatus::NextToBuy);
        assert!(matches!(
            events[0],
            DomainEvent::ItemMovedToNextToBuy { .. }
        ));
    }

    #[test]
    fn cannot_move_to_next_to_buy_from_inbox() {
        let mut item = make_item();
        assert!(item.move_to_next_to_buy().is_err());
    }

    // --- archive ---

    #[test]
    fn archive_from_inbox() {
        let mut item = make_item();
        let events = item.archive().unwrap();
        assert_eq!(item.status(), &WishItemStatus::Archived);
        assert!(matches!(events[0], DomainEvent::ItemArchived { .. }));
    }

    #[test]
    fn archive_from_on_hold() {
        let mut item = make_item();
        item.review(false).unwrap();
        assert!(item.archive().is_ok());
        assert_eq!(item.status(), &WishItemStatus::Archived);
    }

    #[test]
    fn cannot_archive_purchased() {
        let mut item = make_item();
        item.review(true).unwrap(); // Inbox → NextToBuy
        item.purchase().unwrap(); // NextToBuy → Purchased
        assert!(item.archive().is_err());
    }

    // --- purchase ---

    #[test]
    fn purchase_from_next_to_buy() {
        let mut item = make_item();
        item.review(true).unwrap();
        let events = item.purchase().unwrap();
        assert_eq!(item.status(), &WishItemStatus::Purchased);
        assert!(matches!(events[0], DomainEvent::ItemPurchased { .. }));
    }

    #[test]
    fn cannot_purchase_from_inbox() {
        let mut item = make_item();
        assert!(item.purchase().is_err());
    }

    // --- entity identity ---

    #[test]
    fn same_id_means_equal() {
        let item1 = make_item();
        let mut item2 = item1.clone();
        // 名前を変えても id が同じなら同一エンティティ
        item2.name = WishItemName::new("別の名前").unwrap();
        assert_eq!(item1, item2);
    }

    #[test]
    fn different_id_means_not_equal() {
        let item1 = make_item();
        let item2 = make_item();
        assert_ne!(item1, item2);
    }
}
