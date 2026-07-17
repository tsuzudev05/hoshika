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
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let method = request.method().clone();
    let path = request.uri().path().to_string();

    let token = extract_bearer_token(request.headers()).ok_or_else(|| {
        tracing::warn!(%method, %path, "認証失敗: Authorization ヘッダーが未指定または不正な形式");
        StatusCode::UNAUTHORIZED
    })?;

    let claims = state.auth_service.validate_token(token).map_err(|err| {
        tracing::warn!(%method, %path, error = %err, "認証失敗: トークン検証エラー");
        StatusCode::UNAUTHORIZED
    })?;

    // ハンドラーが Extension<JwtClaims> で user_id（sub）を取り出せるようにする
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}
