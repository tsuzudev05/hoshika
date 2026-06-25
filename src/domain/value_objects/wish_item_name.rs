/// 欲しいものの名前
/// 値オブジェクト
/// String の生値ではなく WishItemName として扱うことで、
/// 「欲しいものの名前は空文字列不可」というドメインルールを型で表現する。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WishItemName(String);

#[derive(Debug, thiserror::Error)]
pub enum WishItemNameError {
    #[error("name must not be empty")]
    Empty,
}

impl WishItemName {
    pub fn new(value: impl Into<String>) -> Result<Self, WishItemNameError> {
        let value = value.into();
        if value.is_empty() {
            return Err(WishItemNameError::Empty);
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_name() {
        assert!(WishItemName::new("テスト").is_ok());
    }

    #[test]
    fn empty_name_is_error() {
        assert!(matches!(
            WishItemName::new(""),
            Err(WishItemNameError::Empty)
        ));
    }
}
