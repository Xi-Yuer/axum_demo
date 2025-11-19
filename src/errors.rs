use crate::response::ApiResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;

/// 应用错误类型
#[derive(Error, Debug)]
pub enum AppError {
    #[error("数据库错误: {0}")]
    Database(#[from] sea_orm::DbErr),

    #[error("未找到资源")]
    NotFound,

    #[error("未授权")]
    Unauthorized,

    #[error("禁止访问")]
    Forbidden,

    #[error("验证错误: {0}")]
    Validation(String),

    #[error("内部服务器错误: {0}")]
    Internal(#[from] anyhow::Error),

    #[error("JWT 错误: {0}")]
    Jwt(String),
}

/// 实现 IntoResponse 以便在 Axum 中使用
/// 使用统一的 ApiResponse 格式
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (code, message, status) = match self {
            AppError::Database(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                e.to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                "资源未找到".to_string(),
                StatusCode::NOT_FOUND,
            ),
            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "未授权".to_string(),
                StatusCode::UNAUTHORIZED,
            ),
            AppError::Forbidden => (
                StatusCode::FORBIDDEN,
                "禁止访问".to_string(),
                StatusCode::FORBIDDEN,
            ),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg, StatusCode::BAD_REQUEST),
            AppError::Internal(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                e.to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            AppError::Jwt(msg) => (
                StatusCode::UNAUTHORIZED,
                format!("JWT 错误: {}", msg),
                StatusCode::UNAUTHORIZED,
            ),
        };

        let response = ApiResponse::<()>::error(code, message);
        (status, Json(response)).into_response()
    }
}

/// Result 类型别名
pub type Result<T> = std::result::Result<T, AppError>;
