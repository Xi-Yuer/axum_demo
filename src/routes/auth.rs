use axum::{routing::{get, post}, Router};
use crate::controllers::auth_controller;
use crate::AppState;

/// 认证路由
/// 
/// 路由路径（相对于 /api/auth）：
/// - POST /api/auth/register - 用户注册（不需要认证，handler 中没有 AuthUser）
/// - POST /api/auth/login - 用户登录（不需要认证，handler 中没有 AuthUser）
/// - GET /api/auth/me - 获取当前用户信息（需要认证，handler 中有 AuthUser）
/// 
/// 注意：认证由 handler 中的提取器控制，不需要中间件
pub fn routes() -> Router<AppState> {
    Router::new()
        // 公开路由（handler 中没有认证参数）
        .route("/register", post(auth_controller::register))
        .route("/login", post(auth_controller::login))
        
        // 需要认证的路由（handler 中有 AuthUser 参数）
        .route("/me", get(auth_controller::me))
}

