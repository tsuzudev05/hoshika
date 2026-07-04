use sqlx::Row;
use uuid::Uuid;

use super::error::to_repo_err;
use crate::domain::repositories::wish_item_repository::RepositoryError;
use crate::domain::value_objects::Category;

pub(super) fn row_to_category(row: &sqlx::postgres::PgRow) -> Result<Category, RepositoryError> {
    let id: Uuid = row.try_get("id").map_err(to_repo_err)?;
    let name: String = row.try_get("name").map_err(to_repo_err)?;
    Ok(Category { id, name })
}
