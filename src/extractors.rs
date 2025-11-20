use crate::errors::{AppError, Result};
use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use serde::Deserialize;
use uuid::Uuid;

/// 分页参数提取器
#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub page: u64,
    pub page_size: u64,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 10,
        }
    }
}

impl Pagination {
    pub fn offset(&self) -> u64 {
        let page = self.page;
        let page_size = self.page_size;
        (page.saturating_sub(1)) * page_size
    }

    pub fn limit(&self) -> u64 {
        self.page_size
    }
}

/// 自定义认证提取器
/// 从请求头中提取 JWT token 并验证
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: uuid::Uuid,
    pub username: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        // 从请求头获取 Authorization
        let auth_header = parts
            .headers
            .get("authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or(AppError::Unauthorized)?;

        // 提取 Bearer token
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AppError::Unauthorized)?;

        // 验证 token
        let claims = crate::jwt::verify_token(token)?;

        Ok(AuthUser {
            user_id: claims.sub,
            username: claims.username,
        })
    }
}

/// 可选的认证用户（用于某些路由可能不需要认证）
#[derive(Debug, Clone)]
pub struct OptionalAuthUser(pub Option<AuthUser>);

impl OptionalAuthUser {
    /// 获取用户 ID（如果存在）
    pub fn user_id(&self) -> Option<Uuid> {
        self.0.as_ref().map(|u| u.user_id)
    }

    /// 获取用户（如果存在）
    pub fn user(&self) -> Option<&AuthUser> {
        self.0.as_ref()
    }

    /// 检查是否有用户
    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    /// 获取用户，如果不存在则返回错误
    pub fn require(self) -> Result<AuthUser> {
        self.0.ok_or(AppError::Unauthorized)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for OptionalAuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        match AuthUser::from_request_parts(parts, state).await {
            Ok(user) => Ok(OptionalAuthUser(Some(user))),
            Err(_) => Ok(OptionalAuthUser(None)),
        }
    }
}
