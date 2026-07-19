use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode},
    Json,
};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::infrastructure::auth::AuthError;
use crate::presentation::handlers::internal_error;
use crate::presentation::state::AppState;

#[derive(Deserialize)]
pub struct IssueTokenRequest {
    user_id: String,
}

/// POST /auth/token — JWT を発行する。
///
/// 本番では認証（パスワード検証など）後にのみ呼ぶこと。
/// 現時点では user_id を受け取ってそのままトークンを発行する（認証なし）。
pub async fn issue_token(
    State(state): State<AppState>,
    Json(body): Json<IssueTokenRequest>,
) -> (StatusCode, Json<Value>) {
    match state.auth_service.generate_token(&body.user_id) {
        Ok(token) => (StatusCode::CREATED, Json(json!({ "token": token }))),
        Err(e) => internal_error(e),
    }
}

/// GET /auth/verify — Authorization: Bearer <token> ヘッダーのトークンを検証する。
///
/// 成功時は `user_id`（= JWT の sub クレーム）を返す。
/// 失敗時はエラー種別（期限切れ / 無効）を HTTP レスポンスで通知する。
pub async fn verify_token(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> (StatusCode, Json<Value>) {
    let token = match extract_bearer_token(&headers) {
        Some(t) => t,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "Authorization: Bearer <token> ヘッダーが必要です" })),
            )
        }
    };

    match state.auth_service.validate_token(token) {
        Ok(claims) => (StatusCode::OK, Json(json!({ "user_id": claims.sub }))),
        Err(AuthError::Expired) => (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "トークンの有効期限が切れています。再ログインしてください。" })),
        ),
        Err(_) => (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "トークンが無効です。" })),
        ),
    }
}

/// `Authorization: Bearer <token>` ヘッダーからトークン文字列を取り出す。
pub(crate) fn extract_bearer_token(headers: &HeaderMap) -> Option<&str> {
    headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
}
