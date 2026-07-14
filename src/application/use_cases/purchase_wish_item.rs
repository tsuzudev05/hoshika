#![allow(dead_code)]
//! PurchaseWishItem ユースケース
//! 「次に買う」状態のWishItemを購入済みにし、当月の予算から実支払額を差し引いて
//! PurchaseRecordを記録する。希望価格と実支払額はズレうる（セールなど）ため、
//! 実際に支払った金額を入力させる。
use std::sync::Arc;

use chrono::{Datelike, Utc};
use uuid::Uuid;

use crate::domain::entities::purchase_record::PurchaseRecord;
use crate::domain::repositories::wish_item_repository::RepositoryError;
use crate::domain::repositories::{BudgetRepository, PurchaseRecordRepository, WishItemRepository};
use crate::domain::value_objects::{Memo, Price, YearMonth};

pub struct PurchaseWishItemUseCase {
    wish_item_repo: Arc<dyn WishItemRepository>,
    budget_repo: Arc<dyn BudgetRepository>,
    purchase_record_repo: Arc<dyn PurchaseRecordRepository>,
}

impl PurchaseWishItemUseCase {
    pub fn new(
        wish_item_repo: Arc<dyn WishItemRepository>,
        budget_repo: Arc<dyn BudgetRepository>,
        purchase_record_repo: Arc<dyn PurchaseRecordRepository>,
    ) -> Self {
        Self {
            wish_item_repo,
            budget_repo,
            purchase_record_repo,
        }
    }

    pub async fn execute(
        &self,
        wish_item_id: Uuid,
        actual_price: u64,
        memo: Option<String>,
    ) -> Result<(), UseCaseError> {
        let mut item = self
            .wish_item_repo
            .find_by_id(wish_item_id)
            .await?
            .ok_or(UseCaseError::WishItemNotFound(wish_item_id))?;

        let now = Utc::now();
        let year_month = YearMonth::new(now.year() as u16, now.month() as u8)
            .map_err(|e| UseCaseError::DomainError(e.to_string()))?;
        let mut budget = self
            .budget_repo
            .find_by_year_month(year_month)
            .await?
            .ok_or(UseCaseError::BudgetNotFound)?;

        let price = Price::new(actual_price).map_err(|_| UseCaseError::InvalidPrice)?;

        item.purchase()
            .map_err(|e| UseCaseError::DomainError(e.to_string()))?;
        budget.record_purchase(price.clone());

        let record = PurchaseRecord::new(
            budget.id(),
            item.id(),
            price,
            Memo::new(memo.unwrap_or_default()),
        );

        self.wish_item_repo.save(&item).await?;
        self.budget_repo.save(&budget).await?;
        self.purchase_record_repo.save(&record).await?;

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UseCaseError {
    #[error("wish item not found: {0}")]
    WishItemNotFound(Uuid),
    #[error("budget not set for the current month")]
    BudgetNotFound,
    #[error("invalid price")]
    InvalidPrice,
    #[error("domain error: {0}")]
    DomainError(String),
    #[error("repository error: {0}")]
    Repository(#[from] RepositoryError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    use crate::domain::entities::budget::Budget;
    use crate::domain::entities::wish_item::WishItem;
    use crate::domain::value_objects::{Category, Memo, Price, WishItemName, WishItemStatus};
    use crate::infrastructure::in_memory::{
        InMemoryBudgetRepository, InMemoryPurchaseRecordRepository, InMemoryWishItemRepository,
    };

    fn make_next_to_buy_item(price: u64) -> WishItem {
        let (mut item, _) = WishItem::new(
            WishItemName::new("テスト本").unwrap(),
            Price::new(price).unwrap(),
            Category {
                id: Uuid::new_v4(),
                name: "書籍".to_string(),
            },
            Memo::new(""),
        );
        item.review(true).unwrap(); // Inbox -> NextToBuy
        item
    }

    fn make_current_month_budget(amount: u64) -> Budget {
        let now = Utc::now();
        let ym = YearMonth::new(now.year() as u16, now.month() as u8).unwrap();
        let (budget, _) = Budget::new(ym, Price::new(amount).unwrap());
        budget
    }

    struct Fixture {
        wish_item_repo: Arc<InMemoryWishItemRepository>,
        budget_repo: Arc<InMemoryBudgetRepository>,
        purchase_record_repo: Arc<InMemoryPurchaseRecordRepository>,
        use_case: PurchaseWishItemUseCase,
    }

    /// budget を渡すと当月の予算として投入する。wish_item_repo は空の状態で返すので、
    /// 各テストが対象のWishItemを個別にsaveする。
    fn setup(budget: Option<Budget>) -> Fixture {
        let wish_item_repo = Arc::new(InMemoryWishItemRepository::new());
        let budget_repo = Arc::new(match budget {
            Some(b) => InMemoryBudgetRepository::with_budgets(vec![b]),
            None => InMemoryBudgetRepository::new(),
        });
        let purchase_record_repo = Arc::new(InMemoryPurchaseRecordRepository::new());

        let use_case = PurchaseWishItemUseCase::new(
            wish_item_repo.clone(),
            budget_repo.clone(),
            purchase_record_repo.clone(),
        );

        Fixture {
            wish_item_repo,
            budget_repo,
            purchase_record_repo,
            use_case,
        }
    }

    // --- 正常系 ---

    #[tokio::test]
    async fn execute_transitions_item_to_purchased() {
        let item = make_next_to_buy_item(3000);
        let id = item.id();
        let fixture = setup(Some(make_current_month_budget(50000)));
        fixture.wish_item_repo.save(&item).await.unwrap();

        fixture.use_case.execute(id, 2800, None).await.unwrap();

        let updated = fixture
            .wish_item_repo
            .find_by_id(id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(updated.status(), &WishItemStatus::Purchased);
    }

    #[tokio::test]
    async fn execute_deducts_actual_price_from_current_month_budget() {
        let item = make_next_to_buy_item(3000);
        let id = item.id();
        let budget = make_current_month_budget(50000);
        let budget_id = budget.id();
        let fixture = setup(Some(budget));
        fixture.wish_item_repo.save(&item).await.unwrap();

        fixture.use_case.execute(id, 2800, None).await.unwrap();

        let updated_budget = fixture
            .budget_repo
            .find_by_id(budget_id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(updated_budget.balance().value(), 47200);
    }

    #[tokio::test]
    async fn execute_saves_purchase_record_with_actual_price() {
        let item = make_next_to_buy_item(3000);
        let id = item.id();
        let budget = make_current_month_budget(50000);
        let budget_id = budget.id();
        let fixture = setup(Some(budget));
        fixture.wish_item_repo.save(&item).await.unwrap();

        fixture
            .use_case
            .execute(id, 2800, Some("セールで安かった".to_string()))
            .await
            .unwrap();

        let records = fixture.purchase_record_repo.find_all().await;
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].actual_price().value(), 2800);
        assert_eq!(records[0].wish_item_id(), id);
        assert_eq!(records[0].budget_id(), budget_id);
        assert_eq!(records[0].memo().value(), "セールで安かった");
    }

    // --- 異常系 ---

    #[tokio::test]
    async fn execute_returns_error_when_wish_item_not_found() {
        let fixture = setup(Some(make_current_month_budget(50000)));

        let result = fixture.use_case.execute(Uuid::new_v4(), 1000, None).await;

        assert!(matches!(result, Err(UseCaseError::WishItemNotFound(_))));
    }

    #[tokio::test]
    async fn execute_returns_error_when_budget_not_set_for_current_month() {
        let item = make_next_to_buy_item(1000);
        let id = item.id();
        let fixture = setup(None);
        fixture.wish_item_repo.save(&item).await.unwrap();

        let result = fixture.use_case.execute(id, 1000, None).await;

        assert!(matches!(result, Err(UseCaseError::BudgetNotFound)));
    }

    #[tokio::test]
    async fn execute_returns_domain_error_when_item_is_not_next_to_buy() {
        let (item, _) = WishItem::new(
            WishItemName::new("テスト本").unwrap(),
            Price::new(1000).unwrap(),
            Category {
                id: Uuid::new_v4(),
                name: "書籍".to_string(),
            },
            Memo::new(""),
        );
        let id = item.id(); // Inbox のまま
        let fixture = setup(Some(make_current_month_budget(50000)));
        fixture.wish_item_repo.save(&item).await.unwrap();

        let result = fixture.use_case.execute(id, 1000, None).await;

        assert!(matches!(result, Err(UseCaseError::DomainError(_))));
    }

    #[tokio::test]
    async fn execute_allows_budget_to_go_negative_when_exceeding() {
        let item = make_next_to_buy_item(60000);
        let id = item.id();
        let budget = make_current_month_budget(50000);
        let budget_id = budget.id();
        let fixture = setup(Some(budget));
        fixture.wish_item_repo.save(&item).await.unwrap();

        fixture.use_case.execute(id, 60000, None).await.unwrap();

        let updated_budget = fixture
            .budget_repo
            .find_by_id(budget_id)
            .await
            .unwrap()
            .unwrap();
        assert!(updated_budget.balance().is_exceeded());
    }
}
