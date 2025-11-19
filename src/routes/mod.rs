use axum::Router;
use crate::AppState;

/// 健康检查路由模块
mod health;
/// 认证路由模块
mod auth;
/// 用户路由模块
mod users;
/// 文章路由模块
mod articles;

/// 创建应用路由
/// 
/// 使用模块化的方式组织路由，每个模块负责自己的路由定义
/// 这样可以轻松扩展到成百上千个路由
/// 
/// 认证策略（完全依赖提取器）：
/// - 需要认证：handler 参数中使用 `AuthUser`，没有认证会自动返回 401
/// - 可选认证：handler 参数中使用 `OptionalAuthUser`，可以处理有/无认证的情况
/// - 不需要认证：handler 中不添加认证参数
/// 
/// 优点：
/// - 类型安全：编译时检查，不会遗漏认证
/// - 灵活性高：每个 handler 可以独立控制认证需求
/// - 代码清晰：handler 签名直接表明是否需要认证
/// - 无需维护：不需要路由组、中间件或白名单
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // 健康检查路由（公开，不需要认证）
        .nest("/", health::routes())
        
        // API 路由（统一使用 /api 前缀）
        .nest("/api", api_routes())
        
        // 应用全局中间件（CORS 和日志追踪）
        .layer(crate::middleware::create_cors_layer())
        .layer(crate::trace_layer!())
        
        // 状态共享
        .with_state(state)
}

/// API 路由集合
fn api_routes() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::routes())
        .nest("/users", users::routes())
        .nest("/articles", articles::routes())
        
        // 未来可以轻松添加更多模块：
        // .nest("/products", products::routes())  // 认证由 handler 中的 AuthUser 控制
        // .nest("/public", public::routes())      // 公开路由，handler 中没有认证参数
        // .nest("/orders", orders::routes())      // 认证由 handler 中的 AuthUser 控制
}

