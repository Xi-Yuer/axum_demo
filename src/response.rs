use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::Utc;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

/// 统一的 API 响应结构
///
/// 所有 API 响应都遵循这个格式，方便前端统一处理：
/// ```json
/// {
///   "code": 200,
///   "message": "success",
///   "data": {...},
///   "timestamp": "2024-01-01T00:00:00Z"
/// }
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// 业务状态码（200 表示成功，其他表示失败）
    pub code: u16,
    /// 响应消息
    pub message: String,
    /// 响应数据（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    /// 时间戳
    pub timestamp: String,
}

impl<T> ApiResponse<T> {
    /// 创建成功响应
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            message: "success".to_string(),
            data: Some(data),
            timestamp: Self::get_timestamp(),
        }
    }

    /// 创建成功响应（带自定义消息）
    pub fn success_with_message(data: T, message: impl Into<String>) -> Self {
        Self {
            code: 200,
            message: message.into(),
            data: Some(data),
            timestamp: Self::get_timestamp(),
        }
    }

    /// 创建成功响应（无数据）
    pub fn success_no_data() -> Self {
        Self {
            code: 200,
            message: "success".to_string(),
            data: None,
            timestamp: Self::get_timestamp(),
        }
    }

    /// 创建失败响应
    pub fn error(code: u16, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
            timestamp: Self::get_timestamp(),
        }
    }

    fn get_timestamp() -> String {
        let utc = Utc::now();
        let china_offset = FixedOffset::east_opt(8 * 3600).unwrap(); // UTC+8
        let china_time: DateTime<FixedOffset> = utc.with_timezone(&china_offset);
        china_time.to_rfc3339()
    }
}

/// 实现 IntoResponse，可以直接在处理器中使用
impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        // 根据 code 设置 HTTP 状态码
        let status = match self.code {
            200 => StatusCode::OK,
            201 => StatusCode::CREATED,
            400 => StatusCode::BAD_REQUEST,
            401 => StatusCode::UNAUTHORIZED,
            403 => StatusCode::FORBIDDEN,
            404 => StatusCode::NOT_FOUND,
            422 => StatusCode::UNPROCESSABLE_ENTITY,
            500 => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::OK, // 默认使用 OK，但 code 字段会反映真实状态
        };

        (status, Json(self)).into_response()
    }
}

/// 为 Result 类型提供便捷转换
impl<T, E> From<Result<T, E>> for ApiResponse<T>
where
    E: std::fmt::Display,
{
    fn from(result: Result<T, E>) -> Self {
        match result {
            Ok(data) => ApiResponse::success(data),
            Err(e) => ApiResponse::error(500, format!("{}", e)),
        }
    }
}

/// 辅助函数：快速创建成功响应
pub fn success<T>(data: T) -> ApiResponse<T> {
    ApiResponse::success(data)
}

/// 辅助函数：快速创建成功响应（带消息）
pub fn success_with_message<T>(data: T, message: impl Into<String>) -> ApiResponse<T> {
    ApiResponse::success_with_message(data, message)
}

/// 辅助函数：快速创建失败响应
pub fn error(code: u16, message: impl Into<String>) -> ApiResponse<()> {
    ApiResponse::error(code, message)
}
