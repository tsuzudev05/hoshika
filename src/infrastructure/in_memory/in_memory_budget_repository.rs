#![allow(dead_code)]
use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::domain::entities::budget::Budget;
use crate::domain::repositories::wish_item_repository::RepositoryError;
use crate::domain::repositories::BudgetRepository;
use crate::domain::value_objects::YearMonth;

pub struct InMemoryBudgetRepository {
    store: Arc<Mutex<HashMap<Uuid, Budget>>>,
}

impl InMemoryBudgetRepository {
    pub fn new() -> Self {
        Self {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl BudgetRepository for InMemoryBudgetRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Budget>, RepositoryError> {
        let store = self.store.lock().await;
        Ok(store.get(&id).cloned())
    }

    async fn find_by_year_month(&self, ym: YearMonth) -> Result<Option<Budget>, RepositoryError> {
        let store = self.store.lock().await;
        Ok(store.values().find(|b| b.year_month() == ym).cloned())
    }

    async fn save(&self, budget: &Budget) -> Result<(), RepositoryError> {
        let mut store = self.store.lock().await;
        store.insert(budget.id(), budget.clone());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::{Price, YearMonth};

    fn make_budget(year: u16, month: u8) -> Budget {
        let ym = YearMonth::new(year, month).unwrap();
        let (b, _) = Budget::new(ym, Price::new(50000).unwrap());
        b
    }

    #[tokio::test]
    async fn save_and_find_by_id() {
        let repo = InMemoryBudgetRepository::new();
        let budget = make_budget(2026, 6);
        let id = budget.id();

        repo.save(&budget).await.unwrap();

        let found = repo.find_by_id(id).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().id(), id);
    }

    #[tokio::test]
    async fn find_by_year_month_returns_matching_budget() {
        let repo = InMemoryBudgetRepository::new();
        let budget = make_budget(2026, 6);
        repo.save(&budget).await.unwrap();

        let ym = YearMonth::new(2026, 6).unwrap();
        let found = repo.find_by_year_month(ym).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().year_month(), ym);
    }

    #[tokio::test]
    async fn find_by_year_month_returns_none_when_missing() {
        let repo = InMemoryBudgetRepository::new();
        repo.save(&make_budget(2026, 6)).await.unwrap();

        let ym = YearMonth::new(2026, 7).unwrap();
        let found = repo.find_by_year_month(ym).await.unwrap();
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn save_overwrites_existing() {
        let repo = InMemoryBudgetRepository::new();
        let mut budget = make_budget(2026, 6);
        repo.save(&budget).await.unwrap();

        budget.record_purchase(Price::new(10000).unwrap());
        repo.save(&budget).await.unwrap();

        let found = repo.find_by_id(budget.id()).await.unwrap().unwrap();
        assert_eq!(found.balance().value(), budget.balance().value());
    }
}
