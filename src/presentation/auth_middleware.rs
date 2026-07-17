//! JWT 認証ミドルウェア — 業務 API を保護する。
//!
//! `Authorization: Bearer <token>` ヘッダーの JWT を検証し、無効・未指定なら 401 を返す。
//! `/health` `/auth/token` `/auth/verify` には適用しない（`router.rs` 参照）。

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};

use crate::presentation::handlers::auth::extract_bearer_token;
use crate::presentation::state::AppState;

pub async fn require_auth(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = extract_bearer_token(request.headers()).ok_or(StatusCode::UNAUTHORIZED)?;

    state
        .auth_service
        .validate_token(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(next.run(request).await)
}
