//! Budget ハンドラー（TODO: Phase 03 で実装）
use axum::{http::StatusCode, Json};
use serde_json::{json, Value};

pub async fn get_budget_status() -> (StatusCode, Json<Value>) {
    (StatusCode::NOT_IMPLEMENTED, Json(json!({"message": "Phase 02 で実装予定"})))
}
