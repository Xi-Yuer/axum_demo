use crate::response::ApiResponse;
use axum::http::{HeaderValue, StatusCode};
use axum::response::IntoResponse;
use axum::{extract::Request, middleware::Next, response::Response, Json};
use tower_http::cors::CorsLayer;

/// 创建配置好的 TraceLayer 中间件
/// TraceLayer 提供了完整的 HTTP 请求追踪功能，包括：
/// - 请求开始日志
/// - 响应完成日志（包含状态码和耗时）
/// - 错误日志
///
/// 注意：由于 TraceLayer 的类型非常复杂，这里使用宏来简化使用
/// 在 routes.rs 中使用 axum_demo::trace_layer!() 来调用
#[macro_export]
macro_rules! trace_layer {
    () => {
        tower_http::trace::TraceLayer::new_for_http()
            .make_span_with(|request: &axum::http::Request<_>| {
                tracing::info_span!(
                    "http_request",
                    method = %request.method(),
                    uri = %request.uri(),
                    version = ?request.version(),
                )
            })
            .on_request(|request: &axum::http::Request<_>, _span: &tracing::Span| {
                tracing::info!("请求开始: {} {}", request.method(), request.uri());
            })
            .on_response(|_response: &axum::http::Response<_>, latency: std::time::Duration, _span: &tracing::Span| {
                tracing::info!("请求完成 - 状态: {} - 耗时: {:?}", _response.status(), latency);
            })
            .on_failure(|_error: tower_http::classify::ServerErrorsFailureClass, _latency: std::time::Duration, _span: &tracing::Span| {
                tracing::error!("请求失败 - 错误: {:?} - 耗时: {:?}", _error, _latency);
            })
    };
}

/// 认证中间件（示例，实际使用提取器更灵活）
pub async fn auth_middleware(request: Request, next: Next) -> Response {
    // 检查是否需要认证
    let auth_header = request.headers().get("authorization");

    if auth_header.is_none() && !is_public_path(request.uri().path()) {
        let error_response: ApiResponse<()> = ApiResponse::error(401, "需要认证");
        return (StatusCode::UNAUTHORIZED, Json(error_response)).into_response();
    }

    next.run(request).await
}

/// 判断是否为公开路径
fn is_public_path(path: &str) -> bool {
    path.starts_with("/api") || path == "/" || path == "/health"
}

/// 创建 CORS 中间件 Layer
/// CorsLayer 是一个 Layer，不是中间件函数，需要直接作为 Layer 使用
pub fn create_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin("*".parse::<HeaderValue>().unwrap())
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::DELETE,
            axum::http::Method::OPTIONS, // 预检请求需要 OPTIONS 方法
        ])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
        ])
        .allow_credentials(false) // 如果允许 credentials，需要明确指定 origin
}
