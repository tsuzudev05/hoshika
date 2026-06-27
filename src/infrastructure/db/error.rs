use crate::domain::repositories::wish_item_repository::RepositoryError;

pub(super) fn to_repo_err(e: sqlx::Error) -> RepositoryError {
    match e {
        // fetch_one など行が必須のクエリで該当行が存在しなかった場合
        sqlx::Error::RowNotFound => RepositoryError::NotFound,
        // PostgreSQL SQLSTATE 23505: unique_violation（ユニーク制約違反）
        sqlx::Error::Database(ref db_err) if db_err.code().as_deref() == Some("23505") => {
            RepositoryError::Conflict(db_err.message().to_string())
        }
        _ => RepositoryError::Unexpected(e.to_string()),
    }
}
