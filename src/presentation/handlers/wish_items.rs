use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::application::{
    dto::{AddWishItemInput, ReviewWishItemInput, WishItemOutput},
    use_cases::{
        add_wish_item::{AddWishItemUseCase, UseCaseError},
        review_wish_item::{ReviewError, ReviewWishItemUseCase},
    },
};
use crate::domain::entities::wish_item::WishItem;
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

pub async fn list_wish_items(State(state): State<AppState>) -> (StatusCode, Json<Value>) {
    match state.wish_item_repo.find_all().await {
        Ok(items) => {
            let outputs: Vec<WishItemOutput> = items.iter().map(wish_item_to_output).collect();
            (StatusCode::OK, Json(json!(outputs)))
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        ),
    }
}

pub async fn add_wish_item(
    State(state): State<AppState>,
    Json(body): Json<AddWishItemInput>,
) -> (StatusCode, Json<Value>) {
    let use_case = AddWishItemUseCase::new(state.wish_item_repo, state.category_repo);
    match use_case.execute(body).await {
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
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        ),
    }
}

pub async fn review_wish_item(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<ReviewWishItemInput>,
) -> (StatusCode, Json<Value>) {
    let use_case = ReviewWishItemUseCase::new(state.wish_item_repo);
    match use_case.execute(id, body.still_want).await {
        Ok(()) => (StatusCode::OK, Json(json!({}))),
        Err(ReviewError::NotFound(_)) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "wish item not found"})),
        ),
        Err(ReviewError::DomainError(msg)) => (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({"error": msg})),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        ),
    }
}
