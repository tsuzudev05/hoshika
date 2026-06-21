#![allow(dead_code)]
//! WishItem エンティティ（集約ルート）
//!
//! # 不変条件
//! - name は空文字列不可
//! - price は 0 以上
//! - ステータス遷移は `review()`, `move_to_next_to_buy()` 等のメソッドを通じてのみ行う
use crate::domain::events::DomainEvent;
use crate::domain::value_objects::{Category, Memo, Price, WishItemStatus};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct WishItem {
    pub id: Uuid,
    pub name: String,
    pub price: Price,
    pub category: Category,
    pub status: WishItemStatus,
    pub memo: Memo,
    pub added_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl WishItem {
    /// 新規 WishItem を作成する。作成時のステータスは必ず `Inbox`。
    pub fn new(
        name: impl Into<String>,
        price: Price,
        category: Category,
        memo: Memo,
    ) -> Result<(Self, Vec<DomainEvent>), WishItemError> {
        let name = name.into();
        if name.is_empty() {
            return Err(WishItemError::EmptyName);
        }
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
        Ok((item, events))
    }

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
        let event = DomainEvent::ItemReviewed {
            wish_item_id: self.id,
            still_want,
        };
        Ok(vec![event])
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
        let event = DomainEvent::ItemMovedToNextToBuy {
            wish_item_id: self.id,
        };
        Ok(vec![event])
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

#[derive(Debug, thiserror::Error)]
pub enum WishItemError {
    #[error("name must not be empty")]
    EmptyName,
    #[error("invalid status transition: {from:?} -> {to:?}")]
    InvalidTransition {
        from: WishItemStatus,
        to: WishItemStatus,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::{Category, Memo, Price};

    fn make_item() -> WishItem {
        let category = Category {
            id: Uuid::new_v4(),
            name: "書籍".to_string(),
        };
        let (item, _) = WishItem::new(
            "テスト本",
            Price::new(2000).unwrap(),
            category,
            Memo::new(""),
        )
        .unwrap();
        item
    }

    #[test]
    fn new_item_status_is_inbox() {
        let item = make_item();
        assert_eq!(item.status, WishItemStatus::Inbox);
    }

    #[test]
    fn empty_name_returns_error() {
        let category = Category {
            id: Uuid::new_v4(),
            name: "書籍".to_string(),
        };
        let result = WishItem::new("", Price::new(1000).unwrap(), category, Memo::new(""));
        assert!(matches!(result, Err(WishItemError::EmptyName)));
    }

    #[test]
    fn review_inbox_to_next_to_buy() {
        let mut item = make_item();
        let events = item.review(true).unwrap();
        assert_eq!(item.status, WishItemStatus::NextToBuy);
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
        assert_eq!(item.status, WishItemStatus::OnHold);
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
        let result = item.review(true);
        assert!(result.is_err());
    }
}
