use axum::{
    routing::{get, post, put, delete},
    Router,
};
use crate::controllers::{auth_controller, health_controller, user_controller, article_controller};
use crate::AppState;
use crate::middleware::{auth_middleware, create_cors_layer};
use crate::trace_layer;

/// 创建应用路由
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // 健康检查
        .route("/", get(health_controller::health_check))
        .route("/health", get(health_controller::health_check))
        .route("/health/detailed", get(health_controller::health_check_detailed))

        // 认证路由（公开）
        .route("/api/auth/register", post(auth_controller::register))
        .route("/api/auth/login", post(auth_controller::login))

        // 需要认证的路由
        .route("/api/auth/me", get(auth_controller::me))

        // 用户路由
        .route("/api/users", get(user_controller::list_users))
        .route("/api/users/:id", get(user_controller::get_user))
        .route("/api/users/:id", put(user_controller::update_user))
        .route("/api/users/:id", delete(user_controller::delete_user))

        // 文章路由
        .route("/api/articles", get(article_controller::list_articles))
        .route("/api/articles/simple", get(article_controller::list_articles_simple))  // 示例：使用 From<Result>
        .route("/api/articles/:id", get(article_controller::get_article))
        .route("/api/articles/:id/simple", get(article_controller::get_article_simple))  // 示例：使用 From<Result>
        .route("/api/articles", post(article_controller::create_article))

        // 应用中间件
        // TraceLayer 提供了完整的 HTTP 请求追踪功能
        .layer(create_cors_layer())
        .layer(trace_layer!())
        .layer(axum::middleware::from_fn(auth_middleware))

        // 状态共享
        .with_state(state)
}

