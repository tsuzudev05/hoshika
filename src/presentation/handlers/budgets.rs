use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::application::use_cases::get_budget_status::GetBudgetStatusUseCase;
use crate::presentation::state::AppState;

#[derive(Deserialize)]
pub struct BudgetStatusQuery {
    year: u16,
    month: u8,
}

pub async fn get_budget_status(
    State(state): State<AppState>,
    Query(params): Query<BudgetStatusQuery>,
) -> (StatusCode, Json<Value>) {
    let use_case = GetBudgetStatusUseCase::new(state.budget_repo);
    match use_case.execute(params.year, params.month).await {
        Ok(Some(output)) => (StatusCode::OK, Json(json!(output))),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "budget not found for the specified year/month"})),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        ),
    }
}
