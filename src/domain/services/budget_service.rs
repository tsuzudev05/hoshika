#![allow(dead_code)]
/// 予算サービス
/// ドメインサービス
/// 複数の集約をまたぐドメインロジック（予算超過チェック）を担う。
/// WishItem と Budget は別集約のため、ドメインサービスで橋渡しする。
use crate::domain::entities::budget::Budget;
use crate::domain::value_objects::Price;

pub struct BudgetService;

impl BudgetService {
    /// 指定金額の購入が予算超過になるかチェックする
    pub fn will_exceed(budget: &Budget, price: &Price) -> bool {
        budget.is_exceed(price)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::YearMonth;

    fn make_budget(amount: u64, balance: i64) -> Budget {
        let ym = YearMonth::new(2026, 6).unwrap();
        let (mut b, _) = Budget::new("test-user".to_string(), ym, Price::new(amount).unwrap());
        let spend = amount as i64 - balance;
        if spend > 0 {
            b.record_purchase(Price::new(spend as u64).unwrap());
        }
        b
    }

    #[test]
    fn will_exceed_when_over_budget() {
        let budget = make_budget(10000, 500);
        assert!(BudgetService::will_exceed(
            &budget,
            &Price::new(501).unwrap()
        ));
    }

    #[test]
    fn will_not_exceed_when_within_budget() {
        let budget = make_budget(10000, 500);
        assert!(!BudgetService::will_exceed(
            &budget,
            &Price::new(500).unwrap()
        ));
    }
}
