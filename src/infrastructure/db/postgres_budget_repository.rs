#![allow(dead_code)]
//! TODO: Phase 02 で実装
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::entities::budget::Budget;
use crate::domain::repositories::wish_item_repository::RepositoryError;
use crate::domain::repositories::BudgetRepository;
use crate::domain::value_objects::YearMonth;

pub struct PostgresBudgetRepository {
    pool: Arc<PgPool>,
}

impl PostgresBudgetRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BudgetRepository for PostgresBudgetRepository {
    async fn find_by_id(&self, _id: Uuid) -> Result<Option<Budget>, RepositoryError> {
        todo!("Phase 02 で実装")
    }

    async fn find_by_year_month(&self, _ym: YearMonth) -> Result<Option<Budget>, RepositoryError> {
        todo!("Phase 02 で実装")
    }

    async fn save(&self, _budget: &Budget) -> Result<(), RepositoryError> {
        todo!("Phase 02 で実装")
    }
}
