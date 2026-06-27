#![allow(dead_code)]
use crate::domain::events::DomainEvent;
use crate::domain::value_objects::{Balance, Price, YearMonth};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// 予算
/// Budget エンティティ（集約ルート）
/// 同一性: id による
/// balance は負になりうる（予算超過 = ドメイン上ありえる状態）
#[derive(Debug, Clone)]
pub struct Budget {
    id: Uuid,
    year_month: YearMonth,
    amount: Price,
    balance: Balance,
    set_at: DateTime<Utc>,
}

impl Budget {
    pub fn new(year_month: YearMonth, amount: Price) -> (Self, Vec<DomainEvent>) {
        let id = Uuid::new_v4();
        let balance = Balance::from_price(&amount);
        let budget = Self {
            id,
            year_month,
            amount: amount.clone(),
            balance,
            set_at: Utc::now(),
        };
        let events = vec![DomainEvent::BudgetSet {
            budget_id: id,
            year_month,
            amount,
        }];
        (budget, events)
    }

    /// DBからの再構築。不変条件の適用・イベント発行は行わない。
    /// Infrastructure 層のリポジトリからのみ呼ばれる。
    pub fn reconstitute(
        id: Uuid,
        year_month: YearMonth,
        amount: Price,
        balance: Balance,
        set_at: DateTime<Utc>,
    ) -> Self {
        Self { id, year_month, amount, balance, set_at }
    }

    // --- getters ---

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn year_month(&self) -> YearMonth {
        self.year_month
    }

    pub fn amount(&self) -> &Price {
        &self.amount
    }

    pub fn balance(&self) -> Balance {
        self.balance
    }

    pub fn set_at(&self) -> DateTime<Utc> {
        self.set_at
    }

    // --- domain queries ---

    /// 指定金額を購入すると予算超過になるかチェックする
    pub fn is_exceed(&self, price: &Price) -> bool {
        !self.balance.is_sufficient_for(price)
    }

    // --- state transitions ---

    /// 購入を記録し、残高を減らす。超過した場合は BudgetExceeded イベントを追加する。
    pub fn record_purchase(&mut self, actual_price: Price) -> Vec<DomainEvent> {
        let prev_balance = self.balance;
        self.balance = self.balance.deduct(&actual_price);
        let mut events = vec![DomainEvent::PurchaseRecorded {
            budget_id: self.id,
            actual_price: actual_price.clone(),
        }];
        if !prev_balance.is_exceeded() && self.balance.is_exceeded() {
            events.push(DomainEvent::BudgetExceeded { budget_id: self.id });
        }
        events
    }
}

impl PartialEq for Budget {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Budget {}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_budget(amount: u64) -> Budget {
        let ym = YearMonth::new(2026, 6).unwrap();
        let (b, _) = Budget::new(ym, Price::new(amount).unwrap());
        b
    }

    #[test]
    fn new_balance_equals_amount() {
        let b = make_budget(10000);
        assert_eq!(b.balance().value(), 10000);
    }

    #[test]
    fn record_purchase_reduces_balance() {
        let mut b = make_budget(10000);
        b.record_purchase(Price::new(3000).unwrap());
        assert_eq!(b.balance().value(), 7000);
    }

    #[test]
    fn record_purchase_emits_exceeded_when_going_negative() {
        let mut b = make_budget(1000);
        let events = b.record_purchase(Price::new(1500).unwrap());
        assert!(b.balance().is_exceeded());
        assert!(events
            .iter()
            .any(|e| matches!(e, DomainEvent::BudgetExceeded { .. })));
    }

    #[test]
    fn no_exceeded_event_when_already_negative() {
        let mut b = make_budget(1000);
        b.record_purchase(Price::new(1500).unwrap()); // 超過イベント発生
        let events = b.record_purchase(Price::new(100).unwrap()); // 既に負
        assert!(!events
            .iter()
            .any(|e| matches!(e, DomainEvent::BudgetExceeded { .. })));
    }

    #[test]
    fn would_exceed_when_price_exceeds_balance() {
        let mut b = make_budget(10000);
        b.record_purchase(Price::new(9500).unwrap()); // balance = 500
        assert!(b.is_exceed(&Price::new(501).unwrap()));
        assert!(!b.is_exceed(&Price::new(500).unwrap()));
    }

    #[test]
    fn same_id_means_equal() {
        let b1 = make_budget(10000);
        let b2 = b1.clone();
        assert_eq!(b1, b2);
    }

    #[test]
    fn different_id_means_not_equal() {
        let b1 = make_budget(10000);
        let b2 = make_budget(10000);
        assert_ne!(b1, b2);
    }
}
