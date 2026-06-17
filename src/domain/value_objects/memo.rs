//! Memo 値オブジェクト（空文字列を許容するメモ）
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Memo(String);

impl Memo {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
