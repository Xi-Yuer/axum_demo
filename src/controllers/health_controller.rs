use axum::Json;
use serde_json::json;
use crate::errors::Result;
use crate::response::{ApiResponse, success};
use sea_orm::{ConnectionTrait, DatabaseConnection};

/// 健康检查端点
pub async fn health_check() -> Json<ApiResponse<serde_json::Value>> {
    Json(success(json!({
        "status": "ok",
        "message": "服务运行正常",
    })))
}

/// 详细健康检查（包含数据库连接状态）
pub async fn health_check_detailed(
    axum::extract::State(db): axum::extract::State<DatabaseConnection>,
) -> Result<Json<ApiResponse<serde_json::Value>>> {
    // 检查数据库连接
    use sea_orm::DatabaseBackend;
    use sea_orm::Statement;
    
    db.execute(Statement::from_string(
        DatabaseBackend::MySql,
        "SELECT 1".to_string(),
    ))
    .await
    .map_err(crate::errors::AppError::Database)?;

    Ok(Json(success(json!({
        "status": "ok",
        "database": "connected",
    }))))
}

