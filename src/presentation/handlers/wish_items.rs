use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::application::{
    dto::{AddWishItemInput, PurchaseWishItemInput, ReviewWishItemInput, WishItemOutput},
    use_cases::{
        add_wish_item::{AddWishItemUseCase, UseCaseError},
        purchase_wish_item::{PurchaseWishItemUseCase, UseCaseError as PurchaseError},
        review_wish_item::{ReviewError, ReviewWishItemUseCase},
    },
};
use crate::domain::entities::wish_item::WishItem;
use crate::infrastructure::auth::JwtClaims;
use crate::presentation::handlers::internal_error;
use crate::presentation::state::AppState;

fn wish_item_to_output(item: &WishItem) -> WishItemOutput {
    WishItemOutput {
        id: item.id(),
        name: item.name().to_string(),
        price: item.price().value(),
        category_name: item.category().name.clone(),
        status: format!("{:?}", item.status()),
        memo: item.memo().value().to_string(),
        added_at: item.added_at().to_rfc3339(),
    }
}

pub async fn list_wish_items(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
) -> (StatusCode, Json<Value>) {
    match state.wish_item_repo.find_all(&claims.sub).await {
        Ok(items) => {
            let outputs: Vec<WishItemOutput> = items.iter().map(wish_item_to_output).collect();
            (StatusCode::OK, Json(json!(outputs)))
        }
        Err(e) => internal_error(e),
    }
}

pub async fn add_wish_item(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
    Json(body): Json<AddWishItemInput>,
) -> (StatusCode, Json<Value>) {
    let use_case = AddWishItemUseCase::new(state.wish_item_repo, state.category_repo);
    match use_case.execute(&claims.sub, body).await {
        Ok(output) => (StatusCode::CREATED, Json(json!(output))),
        Err(UseCaseError::CategoryNotFound(id)) => (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({"error": format!("category not found: {id}")})),
        ),
        Err(UseCaseError::InvalidPrice) => (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({"error": "invalid price"})),
        ),
        Err(UseCaseError::DomainError(msg)) => (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({"error": msg})),
        ),
        Err(e) => internal_error(e),
    }
}

pub async fn review_wish_item(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
    Path(id): Path<Uuid>,
    Json(body): Json<ReviewWishItemInput>,
) -> (StatusCode, Json<Value>) {
    let use_case = ReviewWishItemUseCase::new(state.wish_item_repo);
    match use_case.execute(&claims.sub, id, body.still_want).await {
        Ok(()) => (StatusCode::OK, Json(json!({}))),
        Err(ReviewError::NotFound(_)) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "wish item not found"})),
        ),
        Err(ReviewError::DomainError(msg)) => (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({"error": msg})),
        ),
        Err(e) => internal_error(e),
    }
}

pub async fn purchase_wish_item(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
    Path(id): Path<Uuid>,
    Json(body): Json<PurchaseWishItemInput>,
) -> (StatusCode, Json<Value>) {
    let use_case = PurchaseWishItemUseCase::new(
        state.wish_item_repo,
        state.budget_repo,
        state.purchase_record_repo,
    );
    match use_case
        .execute(&claims.sub, id, body.actual_price, body.memo)
        .await
    {
        Ok(()) => (StatusCode::OK, Json(json!({}))),
        Err(PurchaseError::WishItemNotFound(_)) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "wish item not found"})),
        ),
        Err(PurchaseError::BudgetNotFound) => (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({"error": "budget not set for the current month"})),
        ),
        Err(PurchaseError::InvalidPrice) => (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({"error": "invalid price"})),
        ),
        Err(PurchaseError::DomainError(msg)) => (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({"error": msg})),
        ),
        Err(e) => internal_error(e),
    }
}
