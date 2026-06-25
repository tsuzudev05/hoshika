/// カテゴリ
/// 値オブジェクト アイテムの分類
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
}
