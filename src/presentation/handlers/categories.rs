use axum::{extract::State, http::StatusCode, Json};
use serde_json::{json, Value};

use crate::application::dto::CategoryOutput;
use crate::presentation::handlers::internal_error;
use crate::presentation::state::AppState;

pub async fn list_categories(State(state): State<AppState>) -> (StatusCode, Json<Value>) {
    match state.category_repo.find_all().await {
        Ok(categories) => {
            let outputs: Vec<CategoryOutput> = categories
                .into_iter()
                .map(|c| CategoryOutput {
                    id: c.id,
                    name: c.name,
                })
                .collect();
            (StatusCode::OK, Json(json!(outputs)))
        }
        Err(e) => internal_error(e),
    }
}
