#![allow(dead_code)]
/// 欲しいものの状態
/// 値オブジェクト（ステータス列挙）
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WishItemStatus {
    Inbox,
    NextToBuy,
    OnHold,
    Archived,
    Purchased,
}
