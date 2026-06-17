//! Budget エンティティ（集約ルート）
use crate::domain::events::DomainEvent;
use crate::domain::value_objects::{Price, YearMonth};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Budget {
    pub id: Uuid,
    pub year_month: YearMonth,
    pub amount: Price,
    pub balance: i64, // 負になりうる（BudgetExceeded）
    pub set_at: DateTime<Utc>,
}

impl Budget {
    pub fn new(year_month: YearMonth, amount: Price) -> (Self, Vec<DomainEvent>) {
        let id = Uuid::new_v4();
        let balance = amount.value() as i64;
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

    /// 購入を記録し、残高を減らす
    pub fn record_purchase(&mut self, actual_price: Price) -> Vec<DomainEvent> {
        let prev_balance = self.balance;
        self.balance -= actual_price.value() as i64;
        let mut events = vec![DomainEvent::PurchaseRecorded {
            budget_id: self.id,
            actual_price: actual_price.clone(),
        }];
        if prev_balance >= 0 && self.balance < 0 {
            events.push(DomainEvent::BudgetExceeded { budget_id: self.id });
        }
        events
    }
}
