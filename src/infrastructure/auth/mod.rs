use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Claims {
    /// subject（ユーザーID）
    pub sub: String,
    /// expiration（Unix 秒）
    pub exp: i64,
    /// issued at（Unix 秒）
    pub iat: i64,
}

#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum AuthError {
    #[error("invalid token")]
    InvalidToken,
    #[error("token expired")]
    Expired,
    #[error("token generation failed: {0}")]
    GenerationFailed(String),
    #[error("JWT_SECRET is not set")]
    MissingSecret,
}

/// JWT の発行・検証を担う。Domain 層はこのサービスを知らない。
#[allow(dead_code)]
pub struct JwtAuthService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
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
    pub fn generate_token(&self, user_id: &str) -> Result<String, AuthError> {
        let now = Utc::now().timestamp();
        let claims = Claims {
            sub: user_id.to_string(),
            exp: now + self.expires_in_secs,
            iat: now,
        };
        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AuthError::GenerationFailed(e.to_string()))
    }

    /// トークンを検証し、成功したら `Claims` を返す。
    pub fn validate_token(&self, token: &str) -> Result<Claims, AuthError> {
        decode::<Claims>(token, &self.decoding_key, &Validation::default())
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
