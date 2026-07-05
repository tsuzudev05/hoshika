//! Axum ハンドラー（parse → usecase → serialize のみ）
//! ビジネスロジックをここに書いたら設計ミスのサイン。
pub mod auth;
pub mod budgets;
pub mod categories;
pub mod health;
pub mod wish_items;
