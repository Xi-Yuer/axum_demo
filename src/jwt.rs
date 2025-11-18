use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::config::JwtConfig;
use crate::errors::{AppError, Result};

/// JWT Claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid, // 用户 ID
    pub username: String,
    pub exp: i64,  // 过期时间
    pub iat: i64,  // 签发时间
}

impl Claims {
    /// 创建新的 Claims
    pub fn new(user_id: Uuid, username: String, expiration_days: i64) -> Self {
        let now = Utc::now();
        Claims {
            sub: user_id,
            username,
            exp: (now + Duration::days(expiration_days)).timestamp(),
            iat: now.timestamp(),
        }
    }

    /// 生成 JWT token
    pub fn to_token(&self, secret: &str) -> Result<String> {
        encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .map_err(|e| AppError::Jwt(format!("生成 token 失败: {}", e)))
    }
}

/// 验证 token
pub fn verify_token(token: &str) -> Result<Claims> {
    // 这里应该从配置中获取 secret，为了简化示例，使用硬编码
    // 在实际应用中应该从 AppState 或配置中获取
    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string());

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|e| AppError::Jwt(format!("验证 token 失败: {}", e)))?;

    Ok(token_data.claims)
}

/// 从配置生成 token
pub fn generate_token(user_id: Uuid, username: String, config: &JwtConfig) -> Result<String> {
    let claims = Claims::new(user_id, username, config.expiration_days);
    claims.to_token(&config.secret)
}

