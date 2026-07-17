#![allow(dead_code)]
//! InMemoryBudgetRepository — BudgetRepository のインメモリ実装
//! テスト用。DBなしでドメイン・ユースケース層を検証する目的で使う。
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::budget::Budget;
use crate::domain::repositories::wish_item_repository::RepositoryError;
use crate::domain::repositories::BudgetRepository;
use crate::domain::value_objects::YearMonth;
use crate::infrastructure::in_memory::in_memory_store::InMemoryStore;

/// Budget をメモリ上で管理するリポジトリ。
/// 内部の CRUD 操作は [`InMemoryStore`] に委譲する。
pub struct InMemoryBudgetRepository {
    store: InMemoryStore<Budget>,
}

impl InMemoryBudgetRepository {
    /// 空のリポジトリを生成する。
    pub fn new() -> Self {
        Self {
            store: InMemoryStore::new(),
        }
    }

    /// テスト用の初期データを持つリポジトリを生成する。
    ///
    /// # Parameters
    /// - `budgets` — 初期状態として投入する Budget のリスト
    pub fn with_budgets(budgets: Vec<Budget>) -> Self {
        Self {
            store: InMemoryStore::with_items(budgets.into_iter().map(|b| (b.id(), b))),
        }
    }
}

#[async_trait]
impl BudgetRepository for InMemoryBudgetRepository {
    /// 指定した ID の Budget を返す。存在しない、または他ユーザーの所有物の場合は `Ok(None)`。
    async fn find_by_id(&self, user_id: &str, id: Uuid) -> Result<Option<Budget>, RepositoryError> {
        Ok(self
            .store
            .find_by_id(id)
            .await
            .filter(|b| b.user_id() == user_id))
    }

    /// 指定したユーザーの、指定した年月に対応する Budget を返す。存在しない場合は `Ok(None)`。
    ///
    /// インメモリ実装では全件スキャンで検索する。
    /// 本番の PostgreSQL 実装では `WHERE user_id = $1 AND year = $2 AND month = $3` インデックス検索になる。
    async fn find_by_year_month(
        &self,
        user_id: &str,
        ym: YearMonth,
    ) -> Result<Option<Budget>, RepositoryError> {
        let all = self.store.find_all().await;
        Ok(all
            .into_iter()
            .find(|b| b.user_id() == user_id && b.year_month() == ym))
    }

    /// Budget を保存する。同じ ID が既に存在する場合は上書きする（upsert）。
    async fn save(&self, budget: &Budget) -> Result<(), RepositoryError> {
        self.store.save(budget.id(), budget.clone()).await;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::{Price, YearMonth};

    const USER: &str = "user-1";
    const OTHER_USER: &str = "user-2";

    fn make_budget_for(user_id: &str, year: u16, month: u8) -> Budget {
        let ym = YearMonth::new(year, month).unwrap();
        let (b, _) = Budget::new(user_id.to_string(), ym, Price::new(50000).unwrap());
        b
    }

    fn make_budget(year: u16, month: u8) -> Budget {
        make_budget_for(USER, year, month)
    }

    // --- find_by_id ---

    #[tokio::test]
    async fn find_by_id_returns_saved_budget() {
        let repo = InMemoryBudgetRepository::new();
        let budget = make_budget(2026, 6);
        let id = budget.id();
        repo.save(&budget).await.unwrap();

        let found = repo.find_by_id(USER, id).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().id(), id);
    }

    #[tokio::test]
    async fn find_by_id_returns_none_when_missing() {
        let repo = InMemoryBudgetRepository::new();
        let result = repo.find_by_id(USER, Uuid::new_v4()).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn find_by_id_returns_none_for_other_users_budget() {
        let repo = InMemoryBudgetRepository::new();
        let budget = make_budget_for(OTHER_USER, 2026, 6);
        let id = budget.id();
        repo.save(&budget).await.unwrap();

        let result = repo.find_by_id(USER, id).await.unwrap();
        assert!(result.is_none());
    }

    // --- find_by_year_month ---

    #[tokio::test]
    async fn find_by_year_month_returns_matching_budget() {
        let repo = InMemoryBudgetRepository::new();
        repo.save(&make_budget(2026, 6)).await.unwrap();

        let ym = YearMonth::new(2026, 6).unwrap();
        let found = repo.find_by_year_month(USER, ym).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().year_month(), ym);
    }

    #[tokio::test]
    async fn find_by_year_month_returns_none_when_missing() {
        let repo = InMemoryBudgetRepository::new();
        repo.save(&make_budget(2026, 6)).await.unwrap();

        let ym = YearMonth::new(2026, 7).unwrap();
        let found = repo.find_by_year_month(USER, ym).await.unwrap();
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn find_by_year_month_returns_correct_one_among_multiple() {
        let repo = InMemoryBudgetRepository::new();
        repo.save(&make_budget(2026, 5)).await.unwrap();
        repo.save(&make_budget(2026, 6)).await.unwrap();
        repo.save(&make_budget(2026, 7)).await.unwrap();

        let ym = YearMonth::new(2026, 6).unwrap();
        let found = repo.find_by_year_month(USER, ym).await.unwrap().unwrap();
        assert_eq!(found.year_month(), ym);
    }

    #[tokio::test]
    async fn find_by_year_month_does_not_return_other_users_budget() {
        let repo = InMemoryBudgetRepository::new();
        repo.save(&make_budget_for(OTHER_USER, 2026, 6))
            .await
            .unwrap();

        let ym = YearMonth::new(2026, 6).unwrap();
        let found = repo.find_by_year_month(USER, ym).await.unwrap();
        assert!(found.is_none());
    }

    // --- save ---

    #[tokio::test]
    async fn save_overwrites_existing() {
        let repo = InMemoryBudgetRepository::new();
        let mut budget = make_budget(2026, 6);
        repo.save(&budget).await.unwrap();

        budget.record_purchase(Price::new(10000).unwrap());
        repo.save(&budget).await.unwrap();

        let found = repo.find_by_id(USER, budget.id()).await.unwrap().unwrap();
        assert_eq!(found.balance().value(), budget.balance().value());
    }

    // --- with_budgets ---

    #[tokio::test]
    async fn with_budgets_seeds_data() {
        let budget = make_budget(2026, 6);
        let id = budget.id();
        let repo = InMemoryBudgetRepository::with_budgets(vec![budget]);

        let found = repo.find_by_id(USER, id).await.unwrap();
        assert!(found.is_some());
    }
}
