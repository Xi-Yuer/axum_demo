use axum::{routing::get, Router};
use crate::controllers::health_controller;
use crate::AppState;

/// 健康检查路由
/// 
/// 路由路径：
/// - GET / - 健康检查
/// - GET /health - 健康检查
/// - GET /health/detailed - 详细健康检查
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(health_controller::health_check))
        .route("/health", get(health_controller::health_check))
        .route("/health/detailed", get(health_controller::health_check_detailed))
}

