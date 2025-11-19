use axum::{routing::{get, put, delete}, Router};
use crate::controllers::user_controller;
use crate::AppState;

/// 用户路由
/// 
/// 路由路径（相对于 /api/users）：
/// - GET /api/users - 获取用户列表（分页，不需要认证）
/// - GET /api/users/:id - 获取指定用户信息（不需要认证）
/// - PUT /api/users/:id - 更新用户信息（需要认证，handler 中有 AuthUser）
/// - DELETE /api/users/:id - 删除用户（需要认证，handler 中有 AuthUser）
/// 
/// 注意：认证由 handler 中的提取器控制，不需要中间件
pub fn routes() -> Router<AppState> {
    Router::new()
        // 公开路由（handler 中没有认证参数）
        .route("/", get(user_controller::list_users))
        .route("/:id", get(user_controller::get_user))
        
        // 需要认证的路由（handler 中有 AuthUser 参数）
        .route("/:id", put(user_controller::update_user))
        .route("/:id", delete(user_controller::delete_user))
}

