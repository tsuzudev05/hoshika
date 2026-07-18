//! Axum ハンドラー（parse → usecase → serialize のみ）
//! ビジネスロジックをここに書いたら設計ミスのサイン。
pub mod auth;
pub mod budgets;
pub mod categories;
pub mod health;
pub mod wish_items;

use axum::{http::StatusCode, Json};
use serde_json::{json, Value};

/// リポジトリ層の `Unexpected` など、想定外（インフラ層由来）のエラーを 500 として返す。
///
/// 404/422 等のドメイン上想定済みのエラーは各ハンドラーで個別に分類し、
/// この関数を経由させない。ここを通るのは「分類できない予期しないエラー」のみに
/// 絞ることで、ERROR ログ（Sentry へのイベント送信元）にドメイン上正常なケース
/// （見つからない・入力不正など）のノイズが混ざらないようにしている。
pub(crate) fn internal_error(err: impl std::fmt::Display) -> (StatusCode, Json<Value>) {
    let message = err.to_string();
    tracing::error!(error = %message, "unexpected error");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"error": message})),
    )
}
