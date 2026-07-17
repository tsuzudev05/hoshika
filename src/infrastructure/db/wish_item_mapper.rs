use chrono::{DateTime, Utc};
use sqlx::Row;
use uuid::Uuid;

use super::error::to_repo_err;
use crate::domain::entities::wish_item::WishItem;
use crate::domain::repositories::wish_item_repository::RepositoryError;
use crate::domain::value_objects::{Category, Memo, Price, WishItemName, WishItemStatus};

pub(super) fn status_to_str(status: &WishItemStatus) -> &'static str {
    match status {
        WishItemStatus::Inbox => "Inbox",
        WishItemStatus::NextToBuy => "NextToBuy",
        WishItemStatus::OnHold => "OnHold",
        WishItemStatus::Archived => "Archived",
        WishItemStatus::Purchased => "Purchased",
    }
}

fn parse_status(s: &str) -> Result<WishItemStatus, RepositoryError> {
    match s {
        "Inbox" => Ok(WishItemStatus::Inbox),
        "NextToBuy" => Ok(WishItemStatus::NextToBuy),
        "OnHold" => Ok(WishItemStatus::OnHold),
        "Archived" => Ok(WishItemStatus::Archived),
        "Purchased" => Ok(WishItemStatus::Purchased),
        _ => Err(RepositoryError::Unexpected(format!("unknown status: {s}"))),
    }
}

pub(super) fn row_to_wish_item(row: &sqlx::postgres::PgRow) -> Result<WishItem, RepositoryError> {
    let id: Uuid = row.try_get("id").map_err(to_repo_err)?;
    let user_id: String = row.try_get("user_id").map_err(to_repo_err)?;
    let name: String = row.try_get("name").map_err(to_repo_err)?;
    let price_val: i64 = row.try_get("price").map_err(to_repo_err)?;
    let category_id: Uuid = row.try_get("category_id").map_err(to_repo_err)?;
    let category_name: String = row.try_get("category_name").map_err(to_repo_err)?;
    let status_str: String = row.try_get("status").map_err(to_repo_err)?;
    let memo: String = row.try_get("memo").map_err(to_repo_err)?;
    let added_at: DateTime<Utc> = row.try_get("added_at").map_err(to_repo_err)?;
    let updated_at: DateTime<Utc> = row.try_get("updated_at").map_err(to_repo_err)?;

    let name = WishItemName::new(name).map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
    let price =
        Price::new(price_val as u64).map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
    let status = parse_status(&status_str)?;
    let category = Category {
        id: category_id,
        name: category_name,
    };
    let memo = Memo::new(memo);

    Ok(WishItem::reconstitute(
        id, user_id, name, price, category, status, memo, added_at, updated_at,
    ))
}
