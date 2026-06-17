//! Price 値オブジェクト（円単位の非負整数）
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Price(u64);

impl Price {
    pub fn new(value: u64) -> Result<Self, PriceError> {
        Ok(Self(value))
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PriceError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_price_is_valid() {
        assert!(Price::new(0).is_ok());
    }
}
