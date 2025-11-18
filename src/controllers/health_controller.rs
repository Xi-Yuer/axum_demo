use crate::errors::Result;
use crate::response::{success, ApiResponse};
use sea_orm::{ConnectionTrait, DatabaseConnection};
use serde_json::json;

/// 健康检查端点
pub async fn health_check() -> ApiResponse<serde_json::Value> {
    success(json!({
        "status": "ok",
        "message": "服务运行正常",
    }))
}

/// 详细健康检查（包含数据库连接状态）
pub async fn health_check_detailed(
    axum::extract::State(db): axum::extract::State<DatabaseConnection>,
) -> Result<ApiResponse<serde_json::Value>> {
    // 检查数据库连接
    use sea_orm::DatabaseBackend;
    use sea_orm::Statement;

    db.execute(Statement::from_string(
        DatabaseBackend::MySql,
        "SELECT 1".to_string(),
    ))
    .await
    .map_err(crate::errors::AppError::Database)?;

    Ok(success(json!({
        "status": "ok",
        "database": "connected",
    })))
}
