use axum::{routing::{get, post}, Router};
use crate::controllers::article_controller;
use crate::AppState;

/// 文章路由
/// 
/// 路由路径（相对于 /api/articles）：
/// - GET /api/articles - 获取文章列表（可选认证，handler 中有 OptionalAuthUser）
/// - GET /api/articles/simple - 获取文章列表（简单版本，可选认证）
/// - GET /api/articles/:id - 获取指定文章（可选认证）
/// - GET /api/articles/:id/simple - 获取指定文章（简单版本，可选认证）
/// - POST /api/articles - 创建文章（需要认证，handler 中有 AuthUser）
/// 
/// 注意：认证由 handler 中的提取器控制，不需要中间件
pub fn routes() -> Router<AppState> {
    Router::new()
        // 可选认证的路由（handler 中有 OptionalAuthUser）
        .route("/", get(article_controller::list_articles))
        .route("/simple", get(article_controller::list_articles_simple))
        .route("/:id", get(article_controller::get_article))
        .route("/:id/simple", get(article_controller::get_article_simple))
        
        // 需要认证的路由（handler 中有 AuthUser）
        .route("/", post(article_controller::create_article))
}

