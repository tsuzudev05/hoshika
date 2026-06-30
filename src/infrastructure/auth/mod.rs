//! JWT 認証サービス
//!
//! # 認証の流れ
//!
//! ```text
//! [ログイン成功時]
//!   generate_token(user_id)
//!     → HS256 で署名した JWT 文字列を生成
//!     → クライアントに返す（クライアントが保持）
//!
//! [API リクエスト時]
//!   Authorization: Bearer <token> ヘッダーを受け取る
//!     → validate_token(token) で署名・有効期限を検証
//!     → 成功: JwtClaims（user_id など）を取得してリクエストを処理
//!     → 失敗: 401 Unauthorized を返す
//! ```
//!
//! # JWT の構造（Header.Payload.Signature）
//!
//! | 部位      | 内容                              |
//! |-----------|-----------------------------------|
//! | Header    | アルゴリズム（HS256）              |
//! | Payload   | クレーム（sub / exp / iat）        |
//! | Signature | 秘密鍵による署名（改ざん検知用）   |
//!
//! Domain 層はこのサービスを知らない。認証の詳細は Infrastructure 層に閉じている。

use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// JWT のペイロード部分（クレームセット）。
///
/// フィールド名は JWT 標準（RFC 7519）の登録済みクレーム名をそのまま使用する。
/// serde によって JSON にシリアライズされるため、名前を変えると JWT の仕様から外れる。
#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct JwtClaims {
    /// subject — トークンの主体（このアプリではユーザー ID）
    pub sub: String,
    /// expiration — トークンの有効期限（Unix タイムスタンプ、秒）
    pub exp: i64,
    /// issued at — トークンの発行日時（Unix タイムスタンプ、秒）
    pub iat: i64,
}

/// JWT の発行・検証で発生するエラー。
#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum AuthError {
    /// 署名が不正、またはフォーマットが壊れているトークン
    #[error("invalid token")]
    InvalidToken,
    /// 有効期限切れのトークン（期限切れは再ログインを促す）
    #[error("token expired")]
    Expired,
    /// トークン生成時の内部エラー
    #[error("token generation failed: {0}")]
    GenerationFailed(String),
    /// 起動時に `JWT_SECRET` 環境変数が未設定
    #[error("JWT_SECRET is not set")]
    MissingSecret,
}

/// JWT の発行（generate_token）と検証（validate_token）を担うサービス。
///
/// 秘密鍵は HMAC-SHA256（HS256）で署名・検証に使われる。
/// 同じ秘密鍵から EncodingKey（署名用）と DecodingKey（検証用）を別々に生成して保持する。
#[allow(dead_code)]
pub struct JwtAuthService {
    /// トークンへの署名に使うキー（generate_token で使用）
    encoding_key: EncodingKey,
    /// トークンの署名検証に使うキー（validate_token で使用）
    decoding_key: DecodingKey,
    /// トークンの有効期間（秒）。generate_token 時に exp = 現在時刻 + この値 を設定する。
    expires_in_secs: i64,
}

#[allow(dead_code)]
impl JwtAuthService {
    pub fn new(secret: &[u8], expires_in_secs: i64) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),
            expires_in_secs,
        }
    }

    /// `JWT_SECRET` 環境変数から生成する。有効期限は 24 時間。
    pub fn from_env() -> Result<Self, AuthError> {
        let secret = std::env::var("JWT_SECRET").map_err(|_| AuthError::MissingSecret)?;
        Ok(Self::new(secret.as_bytes(), 60 * 60 * 24))
    }

    /// `user_id` を subject にした JWT を発行する。
    ///
    /// 生成されるトークンには sub / exp / iat が含まれる。
    pub fn generate_token(&self, user_id: &str) -> Result<String, AuthError> {
        let now = Utc::now().timestamp();
        let claims = JwtClaims {
            sub: user_id.to_string(),
            exp: now + self.expires_in_secs,
            iat: now,
        };
        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AuthError::GenerationFailed(e.to_string()))
    }

    /// トークンの署名と有効期限を検証し、成功したら `JwtClaims` を返す。
    ///
    /// 失敗時は `AuthError::Expired`（期限切れ）か `AuthError::InvalidToken`（それ以外）を返す。
    pub fn validate_token(&self, token: &str) -> Result<JwtClaims, AuthError> {
        decode::<JwtClaims>(token, &self.decoding_key, &Validation::default())
            .map(|data| data.claims)
            .map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::Expired,
                _ => AuthError::InvalidToken,
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_service() -> JwtAuthService {
        JwtAuthService::new(b"test-secret-key", 3600)
    }

    #[test]
    fn generate_and_validate_round_trip() {
        let svc = make_service();
        let token = svc.generate_token("user-123").unwrap();
        let claims = svc.validate_token(&token).unwrap();
        assert_eq!(claims.sub, "user-123");
    }

    #[test]
    fn validate_rejects_tampered_token() {
        let svc = make_service();
        let result = svc.validate_token("invalid.token.here");
        assert!(matches!(result, Err(AuthError::InvalidToken)));
    }

    #[test]
    fn validate_rejects_expired_token() {
        // jsonwebtoken のデフォルト leeway は 60 秒なので、それを超える過去日時にする
        let svc = JwtAuthService::new(b"test-secret-key", -120);
        let token = svc.generate_token("user-456").unwrap();

        let validator = make_service();
        let result = validator.validate_token(&token);
        assert!(matches!(result, Err(AuthError::Expired)));
    }
}
