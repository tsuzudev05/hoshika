//! InMemoryStore — インメモリリポジトリが共通で使う汎用ストア
//!
//! 各 InMemoryRepository の `Arc<Mutex<HashMap<Uuid, T>>>` 操作を一箇所に集約し、
//! コードの重複を排除する。
use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;
use uuid::Uuid;

/// UUID をキーとするスレッドセーフなインメモリストア。
///
/// `Arc<Mutex<HashMap<Uuid, T>>>` のラッパーであり、
/// 各インメモリリポジトリが共通で必要とする CRUD 操作を提供する。
pub struct InMemoryStore<T> {
    data: Arc<Mutex<HashMap<Uuid, T>>>,
}

impl<T: Clone + Send + 'static> InMemoryStore<T> {
    /// 空のストアを生成する。
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 初期データを持つストアを生成する。
    ///
    /// # Parameters
    /// - `items` — `(Uuid, T)` のイテレータ。ID とデータのペアで投入する
    pub fn with_items(items: impl IntoIterator<Item = (Uuid, T)>) -> Self {
        Self {
            data: Arc::new(Mutex::new(items.into_iter().collect())),
        }
    }

    /// 指定した ID のアイテムを返す。存在しない場合は `None`。
    pub async fn find_by_id(&self, id: Uuid) -> Option<T> {
        let data = self.data.lock().await;
        data.get(&id).cloned()
    }

    /// 保存されている全アイテムを返す。
    pub async fn find_all(&self) -> Vec<T> {
        let data = self.data.lock().await;
        data.values().cloned().collect()
    }

    /// アイテムを保存する。同じ ID が存在する場合は上書きする（upsert）。
    ///
    /// # Parameters
    /// - `id` — ストアのキーとなる UUID
    /// - `item` — 保存するアイテム
    pub async fn save(&self, id: Uuid, item: T) {
        let mut data = self.data.lock().await;
        data.insert(id, item);
    }

    /// 指定した ID のアイテムを削除する。
    ///
    /// # Returns
    /// - `true` — 削除に成功した
    /// - `false` — 指定した ID が存在しなかった
    pub async fn remove(&self, id: Uuid) -> bool {
        let mut data = self.data.lock().await;
        data.remove(&id).is_some()
    }
}
