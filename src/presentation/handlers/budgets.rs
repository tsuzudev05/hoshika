use axum::{
    extract::{Extension, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::application::dto::SetBudgetInput;
use crate::application::use_cases::get_budget_status::GetBudgetStatusUseCase;
use crate::application::use_cases::set_budget::{SetBudgetUseCase, UseCaseError};
use crate::infrastructure::auth::JwtClaims;
use crate::presentation::state::AppState;

#[derive(Deserialize)]
pub struct BudgetStatusQuery {
    year: u16,
    month: u8,
}

pub async fn get_budget_status(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
    Query(params): Query<BudgetStatusQuery>,
) -> (StatusCode, Json<Value>) {
    let use_case = GetBudgetStatusUseCase::new(state.budget_repo);
    match use_case
        .execute(&claims.sub, params.year, params.month)
        .await
    {
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

pub async fn set_budget(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
    Json(body): Json<SetBudgetInput>,
) -> (StatusCode, Json<Value>) {
    let use_case = SetBudgetUseCase::new(state.budget_repo);
    match use_case.execute(&claims.sub, body).await {
        Ok(output) => (StatusCode::OK, Json(json!(output))),
        Err(UseCaseError::InvalidAmount) => (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({"error": "amount must be greater than 0"})),
        ),
        Err(UseCaseError::DomainError(msg)) => (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({"error": msg})),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        ),
    }
}
