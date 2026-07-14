//! インメモリリポジトリ実装群
//!
//! テスト用。PostgreSQL なしでドメイン・ユースケース層の動作を検証できる。
//! 各リポジトリは共通の [`in_memory_store::InMemoryStore`] に CRUD 操作を委譲する。
pub mod in_memory_budget_repository;
pub mod in_memory_category_repository;
pub mod in_memory_purchase_record_repository;
pub mod in_memory_store;
pub mod in_memory_wish_item_repository;

#[allow(unused_imports)]
pub use in_memory_budget_repository::InMemoryBudgetRepository;

#[allow(unused_imports)]
pub use in_memory_category_repository::InMemoryCategoryRepository;

#[allow(unused_imports)]
pub use in_memory_purchase_record_repository::InMemoryPurchaseRecordRepository;

#[allow(unused_imports)]
pub use in_memory_wish_item_repository::InMemoryWishItemRepository;
