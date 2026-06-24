use crate::domain::value_objects::Price;

/// 残高（Balance）値オブジェクト
///
/// 負になりうる（予算超過 = ドメイン上ありえる状態）。
/// i64 の生値ではなく Balance として扱うことで、
/// 「残高」という概念をコードで表現できる。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Balance(i64);

impl Balance {
    #[allow(dead_code)]
    pub fn new(value: i64) -> Self {
        Self(value)
    }

    /// Price から初期残高を生成する（予算設定時）
    pub fn from_price(price: &Price) -> Self {
        Self(price.value() as i64)
    }

    pub fn value(&self) -> i64 {
        self.0
    }

    /// 残高が負（予算超過）かどうか
    pub fn is_exceeded(&self) -> bool {
        self.0 < 0
    }

    /// 指定金額を支払える残高があるか
    pub fn is_sufficient_for(&self, price: &Price) -> bool {
        self.0 >= price.value() as i64
    }

    /// 指定金額を差し引いた新しい残高を返す（自身は変更しない）
    pub fn deduct(&self, price: &Price) -> Self {
        Self(self.0 - price.value() as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_price_sets_value() {
        let b = Balance::from_price(&Price::new(10000).unwrap());
        assert_eq!(b.value(), 10000);
        assert!(!b.is_exceeded());
    }

    #[test]
    fn is_exceeded_when_negative() {
        let b = Balance::new(-1);
        assert!(b.is_exceeded());
    }

    #[test]
    fn zero_balance_is_not_exceeded() {
        assert!(!Balance::new(0).is_exceeded());
    }

    #[test]
    fn is_sufficient_for_exact_amount() {
        let b = Balance::new(500);
        assert!(b.is_sufficient_for(&Price::new(500).unwrap()));
        assert!(!b.is_sufficient_for(&Price::new(501).unwrap()));
    }

    #[test]
    fn deduct_returns_new_balance() {
        let b = Balance::new(1000);
        let after = b.deduct(&Price::new(300).unwrap());
        assert_eq!(after.value(), 700);
        assert_eq!(b.value(), 1000); // 元の値は変わらない
    }

    #[test]
    fn deduct_can_go_negative() {
        let b = Balance::new(100);
        let after = b.deduct(&Price::new(200).unwrap());
        assert!(after.is_exceeded());
    }
}
